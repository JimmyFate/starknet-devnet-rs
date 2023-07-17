use axum::{Extension, Json};

use crate::api::http::error::HttpApiError;
use crate::api::http::{HttpApiHandler, HttpApiResult};
use starknet_types::models::http_models::Path;

pub(crate) async fn dump(
    Json(_path): Json<Path>,
    Extension(_state): Extension<HttpApiHandler>,
) -> HttpApiResult<()> {
    Err(HttpApiError::GeneralError)
}

pub(crate) async fn load(
    Json(_path): Json<Path>,
    Extension(_state): Extension<HttpApiHandler>,
) -> HttpApiResult<()> {
    Err(HttpApiError::PathNotFound)
}
