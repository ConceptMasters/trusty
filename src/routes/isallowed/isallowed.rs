use crate::store::Store;
//use platform_errors::Error;
use race::AccessControlEngine;
use rob::{
    rbac::{IsAllowedRequest, IsAllowedResult},
    token::TokenContext,
};
use std::sync::Arc;
use warp::reply::json;

pub async fn is_allowed(
    ctx: TokenContext,
    _store: Arc<dyn Store>,
    access_control: Arc<AccessControlEngine>,
    is_allowed_request: IsAllowedRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result: IsAllowedResult = access_control
        .is_allowed(&is_allowed_request, ctx.namespace.clone())
        .await?;
    Ok(json(&result))
}
