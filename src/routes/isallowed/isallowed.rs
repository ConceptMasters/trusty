use crate::store::Store;
use crate::race::AccessControlEngine;
use crate::rob::{
    rbac::{IsAllowedRequest, IsAllowedResult},
};
use std::sync::Arc;
use warp::reply::json;

pub async fn is_allowed(
    _store: Arc<dyn Store>,
    access_control: Arc<AccessControlEngine>,
    is_allowed_request: IsAllowedRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result: IsAllowedResult = access_control
        .is_allowed(&is_allowed_request)
        .await?;
    Ok(json(&result))
}
