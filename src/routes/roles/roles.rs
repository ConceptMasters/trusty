use crate::store::Store;
use crate::validation::{ValidateDataIntegrity, ValidateDataIntegrityWithNamespace};
use log::debug;
use platform_errors::Error;
use rob::{
    role::{NewRole, Role, UpdateRole},
    token::TokenContext,
    ValidateInputRules,
};
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_role(
    ctx: TokenContext,
    store: Arc<dyn Store>,
    new_role: NewRole,
) -> Result<impl warp::Reply, warp::Rejection> {
    if ctx.namespace != new_role.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    new_role
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    debug!("Validated new role input");
    new_role.validate_data_integrity(store.clone()).await?;
    debug!("Validated new role data integrity");
    let role = Role::new_from_obj(&new_role);
    debug!("Created Role from NewRole");
    store.add_role(&role).await?;
    Ok(with_status(json(&role), StatusCode::CREATED))
}

pub async fn update_role(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
    update_role: UpdateRole,
) -> Result<impl warp::Reply, warp::Rejection> {
    update_role
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    update_role
        .validate_data_integrity(store.clone(), ctx.namespace.clone())
        .await?;
    let mut role = store.get_role(id.clone()).await?;
    if ctx.namespace != role.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    role.apply_update(&update_role);
    store.update_role(id, &role).await?;
    Ok(json(&role))
}

pub async fn get_role(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let role = store.get_role(id).await?;
    if ctx.namespace != role.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    Ok(json(&role))
}

pub async fn get_roles(
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let roles = store.get_roles(ctx.namespace).await?;
    Ok(json(&roles))
}

pub async fn delete_role(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_role = store.get_role(id.clone()).await?;
    if ctx.namespace != existing_role.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    store.delete_role(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
