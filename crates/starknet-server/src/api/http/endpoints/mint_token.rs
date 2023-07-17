use axum::Json;

use crate::api::http::error::HttpApiError;
use crate::api::http::HttpApiResult;
use starknet_types::models::http_models::{FeeToken, MintTokens, MintTokensResponse};

pub(crate) async fn get_fee_token() -> HttpApiResult<Json<FeeToken>> {
    Err(HttpApiError::GeneralError)
}

pub(crate) async fn mint(Json(_data): Json<MintTokens>) -> HttpApiResult<Json<MintTokensResponse>> {
    Err(HttpApiError::GeneralError)
}
