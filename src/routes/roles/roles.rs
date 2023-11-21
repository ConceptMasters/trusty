use crate::errors::Error;
use crate::rob::{
    role::{NewRole, Role, UpdateRole},
    ValidateInputRules,
};
use crate::store::Store;
use crate::validation::{ValidateDataIntegrity, ValidateDataIntegrityWithNamespace};
use log::debug;
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_role(
    store: Arc<dyn Store>,
    new_role: NewRole,
) -> Result<impl warp::Reply, warp::Rejection> {
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
    store: Arc<dyn Store>,
    update_role: UpdateRole,
) -> Result<impl warp::Reply, warp::Rejection> {
    update_role
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    update_role.validate_data_integrity(store.clone()).await?;
    let mut role = store.get_role(id.clone()).await?;
    role.apply_update(&update_role);
    store.update_role(id, &role).await?;
    Ok(json(&role))
}

pub async fn get_role(
    id: String,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let role = store.get_role(id).await?;
    Ok(json(&role))
}

pub async fn get_roles(store: Arc<dyn Store>) -> Result<impl warp::Reply, warp::Rejection> {
    let roles = store.get_roles().await?;
    Ok(json(&roles))
}

pub async fn delete_role(
    id: String,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_role = store.get_role(id.clone()).await?;
    store.delete_role(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
