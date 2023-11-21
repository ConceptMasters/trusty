use crate::errors::Error;
use crate::rob::{
    user::{NewUser, UpdateUser, User, UserQuery},
    ValidateInputRules,
};
use crate::store::Store;
use crate::validation::ValidateDataIntegrity;
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_user(
    store: Arc<dyn Store>,
    new_user: NewUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    new_user
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    new_user.validate_data_integrity(store.clone()).await?;
    let user = User::new_from_obj(&new_user);
    store.add_user(&user).await?;
    Ok(with_status(json(&user), StatusCode::CREATED))
}

pub async fn update_user(
    external_id: String,
    store: Arc<dyn Store>,
    update_user: UpdateUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    update_user
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    update_user.validate_data_integrity(store.clone()).await?;
    let mut user = store.get_user(external_id.clone()).await?;
    user.apply_update(&update_user);
    store.update_user(external_id, &user).await?;
    Ok(json(&user))
}

pub async fn associate_user_with_tenant(
    user_id: String,
    tenant_id: String,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_user = store.get_user(user_id.clone()).await?;
    let existing_tenant = store.get_tenant(tenant_id.clone()).await?;
    if existing_user
        .associated_tenants
        .contains(&tenant_id.clone())
    {
        return Err(Error::NotModified.into());
    }
    let updated_user = store.associate_user_with_tenant(user_id, tenant_id).await?;
    Ok(json(&updated_user))
}

pub async fn get_user(
    external_id: String,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user = store.get_user(external_id).await?;
    Ok(json(&user))
}

pub async fn get_users(
    store: Arc<dyn Store>,
    query: UserQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = store.get_users(query).await?;
    Ok(json(&users))
}

pub async fn delete_user(
    external_id: String,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_user = store.get_user(external_id.clone()).await?;
    store.delete_user(external_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_user_info(
    external_id: String,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_info = store.get_user_info(external_id.clone()).await?;
    Ok(json(&user_info))
}
