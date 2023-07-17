use axum::Json;

use crate::api::http::error::HttpApiError;
use crate::api::http::HttpApiResult;
use starknet_types::models::http_models::Time;

pub(crate) async fn set_time(Json(_data): Json<Time>) -> HttpApiResult<()> {
    Err(HttpApiError::GeneralError)
}

pub(crate) async fn increase_time(Json(_data): Json<Time>) -> HttpApiResult<()> {
    Err(HttpApiError::GeneralError)
}
