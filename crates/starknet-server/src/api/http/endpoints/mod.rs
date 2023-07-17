use axum::Json;

use super::error::HttpApiError;
use super::HttpApiResult;
use starknet_types::models::http_models::ForkStatus;

/// Dumping and loading
pub(crate) mod dump_load;

/// Postman
pub(crate) mod postman;

/// Blocks
pub(crate) mod blocks;

/// Time
pub(crate) mod time;

/// Accounts
pub(crate) mod accounts;

/// Mint token - Local faucet
pub(crate) mod mint_token;

/// Is alive
pub(crate) async fn is_alive() -> HttpApiResult<String> {
    Ok("Alive!!!".to_string())
}

/// Restart
pub(crate) async fn restart() -> HttpApiResult<()> {
    Err(HttpApiError::GeneralError)
}

/// Fork
pub(crate) async fn get_fork_status() -> HttpApiResult<Json<ForkStatus>> {
    Err(HttpApiError::GeneralError)
}
