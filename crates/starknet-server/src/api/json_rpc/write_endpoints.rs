use starknet_types::rpc::transactions::broadcasted_deploy_account_transaction::BroadcastedDeployAccountTransaction;
use starknet_types::rpc::transactions::broadcasted_invoke_transaction::BroadcastedInvokeTransaction;
use starknet_types::rpc::transactions::BroadcastedDeclareTransaction;

use super::error::{ApiError, StrictRpcResult};
use super::models::{
    DeclareTransactionOutput, DeployAccountTransactionOutput, InvokeTransactionOutput,
};
use super::StarknetResponse;
use crate::api::json_rpc::JsonRpcHandler;

impl JsonRpcHandler {
    pub(crate) async fn add_declare_transaction(
        &self,
        request: BroadcastedDeclareTransaction,
    ) -> StrictRpcResult {
        let (transaction_hash, class_hash) = match request {
            BroadcastedDeclareTransaction::V1(broadcasted_declare_txn) => self
                .api
                .starknet
                .write()
                .await
                .add_declare_transaction_v1(*broadcasted_declare_txn)?,
            BroadcastedDeclareTransaction::V2(broadcasted_declare_txn) => self
                .api
                .starknet
                .write()
                .await
                .add_declare_transaction_v2(*broadcasted_declare_txn)?,
        };

        Ok(StarknetResponse::AddDeclareTransaction(DeclareTransactionOutput {
            transaction_hash,
            class_hash,
        }))
    }

    pub(crate) async fn add_deploy_account_transaction(
        &self,
        request: BroadcastedDeployAccountTransaction,
    ) -> StrictRpcResult {
        let (transaction_hash, contract_address) =
            self.api.starknet.write().await.add_deploy_account_transaction(request).map_err(
                |err| match err {
                    starknet_core::error::Error::StateError(
                        starknet_core::error::StateError::NoneClassHash(_),
                    ) => ApiError::ClassHashNotFound,
                    unknown_error => ApiError::StarknetDevnetError(unknown_error),
                },
            )?;

        Ok(StarknetResponse::AddDeployAccountTransaction(DeployAccountTransactionOutput {
            transaction_hash,
            contract_address,
        }))
    }

    pub(crate) async fn add_invoke_transaction(
        &self,
        request: BroadcastedInvokeTransaction,
    ) -> StrictRpcResult {
        let transaction_hash = self.api.starknet.write().await.add_invoke_transaction(request)?;

        Ok(StarknetResponse::AddInvokeTransaction(InvokeTransactionOutput { transaction_hash }))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use starknet_core::starknet::starknet_config::StarknetConfig;
    use starknet_core::starknet::Starknet;
    use starknet_types::felt::Felt;

    use crate::api::json_rpc::models::{
        BroadcastedDeclareTransactionEnumWrapper, BroadcastedDeclareTransactionInput,
        BroadcastedDeployAccountTransactionEnumWrapper, BroadcastedInvokeTransactionEnumWrapper,
        BroadcastedInvokeTransactionInput,
    };
    use crate::api::json_rpc::StarknetRequest;

    #[test]
    fn perfromance() {
        let seed: u32 = 3819977873;
        let config = StarknetConfig { seed, ..Default::default() };
        let mut starknet = Starknet::new(&config).unwrap();
        let files = [
            "1703494795905_starknet_addDeclareTransaction.json",
            "1703494799624_starknet_addInvokeTransaction.json",
            "1703494802273_starknet_addDeclareTransaction.json",
            "1703494804953_starknet_addInvokeTransaction.json",
            "1703494809746_starknet_addDeclareTransaction.json",
        ];

        for file in files {
            let path = format!("../../{}", file);
            let req =
                serde_json::from_str::<StarknetRequest>(&std::fs::read_to_string(&path).unwrap())
                    .unwrap();

            match req {
                StarknetRequest::AddDeclareTransaction(BroadcastedDeclareTransactionInput {
                    declare_transaction,
                }) => {
                    let BroadcastedDeclareTransactionEnumWrapper::Declare(txn) =
                        declare_transaction;
                    match txn {
                        starknet_types::rpc::transactions::BroadcastedDeclareTransaction::V1(
                            v1,
                        ) => {
                            let (txn_hash, _) = starknet.add_declare_transaction_v1(*v1).unwrap();
                        }
                        starknet_types::rpc::transactions::BroadcastedDeclareTransaction::V2(_) => {
                            todo!()
                        }
                    }
                }
                StarknetRequest::AddInvokeTransaction(BroadcastedInvokeTransactionInput {
                    invoke_transaction,
                }) => {
                    let BroadcastedInvokeTransactionEnumWrapper::Invoke(txn) = invoke_transaction;
                    starknet.add_invoke_transaction(txn).unwrap();
                }
                _ => panic!("aa"),
            }
        }

        let class_hash = Felt::from_prefixed_hex_str(
            "0x36c7e49a16f8fc760a6fbdf71dde543d98be1fee2eda5daff59a0eeae066ed9",
        )
        .unwrap();

        let class = starknet
            .get_class(
                starknet_rs_core::types::BlockId::Tag(starknet_rs_core::types::BlockTag::Pending),
                class_hash,
            )
            .unwrap();

        println!("{}", serde_json::to_string(&class).unwrap().len());
    }

    #[test]
    fn check_correct_deserialization_of_deploy_account_transaction_request() {
        test_deploy_account_transaction();
    }

    /// The example uses declare_v1.json from test_data/rpc/declare_v1.json
    /// Which declares the example from https://www.cairo-lang.org/docs/hello_starknet/intro.html#your-first-contract
    /// The example was compiled locally and send via Postman to https://alpha4.starknet.io/gateway/add_transaction
    #[test]
    fn parsed_base64_gzipped_json_contract_class_correctly() {
        let json_string = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/rpc/declare_v1.json"
        ))
        .unwrap();

        let _broadcasted_declare_transaction_v1: BroadcastedDeclareTransactionEnumWrapper =
            serde_json::from_str(&json_string).unwrap();
    }

    fn test_deploy_account_transaction() -> BroadcastedDeployAccountTransactionEnumWrapper {
        let json_string = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/rpc/deploy_account.json"
        ))
        .unwrap();

        let broadcasted_deploy_account_transaction: BroadcastedDeployAccountTransactionEnumWrapper =
            serde_json::from_str(&json_string).unwrap();

        broadcasted_deploy_account_transaction
    }
}
