use crate::store::Store;
use crate::validation::ValidateDataIntegrity;
use platform_errors::Error;
use rob::{
    namespace::{Namespace, NewNamespace},
    token::TokenContext,
    ValidateInputRules,
};
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_namespace(
    _ctx: TokenContext,
    store: Arc<dyn Store>,
    new_namespace: NewNamespace,
) -> Result<impl warp::Reply, warp::Rejection> {
    new_namespace
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    new_namespace.validate_data_integrity(store.clone()).await?;
    let namespace = Namespace::new_from_obj(&new_namespace);
    store.add_namespace(&namespace).await?;
    Ok(with_status(json(&namespace), StatusCode::CREATED))
}

pub async fn get_namespace(
    id: String,
    _ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let namespace = store.get_namespace(id).await?;
    Ok(json(&namespace))
}

pub async fn get_namespaces(
    _ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let namespaces = store.get_namespaces().await?;
    Ok(json(&namespaces))
}

pub async fn delete_namespace(
    id: String,
    _ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.delete_namespace(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
