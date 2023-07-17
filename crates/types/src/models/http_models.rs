use serde::{Deserialize, Serialize};
use starknet_api::transaction::Fee;

use crate::models::block::BlockHashHex;
use crate::models::transaction::{Calldata, EntryPointSelectorHex, Nonce, TransactionHashHex};
use crate::models::{ContractAddressHex, FeltHex};

#[derive(Deserialize, Debug)]
pub struct Path {
    path: String,
}

#[derive(Deserialize, Debug)]
pub struct PostmanLoadL1MessagingContract {
    #[serde(rename = "networkUrl")]
    network_url: String,
    address: ContractAddressHex,
}

#[derive(Deserialize)]
pub struct MessageToL2 {
    l2_contract_address: ContractAddressHex,
    entry_point_selector: EntryPointSelectorHex,
    l1_contract_addresss: ContractAddressHex,
    payload: Calldata,
    paid_fee_on_l1: Fee,
    nonce: Nonce,
}

#[derive(Deserialize)]
pub struct MessageFromL2 {
    l2_contract_address: ContractAddressHex,
    l1_contract_addresss: ContractAddressHex,
    payload: Calldata,
}

#[derive(Serialize)]
pub struct MessageHash {
    message_hash: FeltHex,
}

#[derive(Serialize)]
pub struct CreatedBlock {
    block_hash: BlockHashHex,
}

#[derive(Deserialize)]
pub struct AbortingBlocks {
    #[serde(rename = "startingBlockHash")]
    starting_block_hash: BlockHashHex,
}

#[derive(Serialize)]
pub struct AbortedBlocks {
    aborted: Vec<BlockHashHex>,
}

#[derive(Deserialize)]
pub struct Time {
    time: u64,
}

#[derive(Serialize)]
pub struct SerializableAccount {
    pub initial_balance: String,
    pub address: ContractAddressHex,
    pub public_key: FeltHex,
    pub private_key: FeltHex,
}

#[derive(Deserialize)]
pub struct ContractAddress {
    contract_address: ContractAddressHex,
}

#[derive(Serialize)]
pub struct Balance {
    amount: u128,
    unit: String,
}

#[derive(Serialize)]
pub struct FeeToken {
    symbol: String,
    address: ContractAddressHex,
}

#[derive(Deserialize)]
pub struct MintTokens {
    address: ContractAddressHex,
    amount: u128,
    lite: Option<bool>,
}

#[derive(Serialize)]
pub struct MintTokensResponse {
    new_balance: u128,
    unit: String,
    tx_hash: TransactionHashHex,
}

#[derive(Serialize)]
pub struct ForkStatus {
    url: String,
    block: u128,
}
