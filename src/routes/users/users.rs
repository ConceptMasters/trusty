use crate::store::Store;
use crate::validation::ValidateDataIntegrity;
use platform_errors::Error;
use rob::{
    token::TokenContext,
    user::{NewUser, UpdateUser, User, UserQuery},
    ValidateInputRules,
};
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_user(
    ctx: TokenContext,
    store: Arc<dyn Store>,
    new_user: NewUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    if ctx.namespace != new_user.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
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
    ctx: TokenContext,
    store: Arc<dyn Store>,
    update_user: UpdateUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    update_user
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    update_user.validate_data_integrity(store.clone()).await?;
    let mut user = store.get_user(external_id.clone()).await?;
    if ctx.namespace != user.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    user.apply_update(&update_user);
    store.update_user(external_id, &user).await?;
    Ok(json(&user))
}

pub async fn associate_user_with_tenant(
    user_id: String,
    tenant_id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_user = store.get_user(user_id.clone()).await?;
    if ctx.namespace.clone() != existing_user.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    let existing_tenant = store.get_tenant(tenant_id.clone()).await?;
    if ctx.namespace != existing_tenant.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
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
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user = store.get_user(external_id).await?;
    if ctx.namespace != user.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    Ok(json(&user))
}

pub async fn get_users(
    ctx: TokenContext,
    store: Arc<dyn Store>,
    query: UserQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = store.get_users(ctx.namespace, query).await?;
    Ok(json(&users))
}

pub async fn delete_user(
    external_id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_user = store.get_user(external_id.clone()).await?;
    if ctx.namespace != existing_user.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    store.delete_user(external_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_user_info(
    external_id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_info = store.get_user_info(external_id.clone()).await?;
    if ctx.namespace != user_info.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    Ok(json(&user_info))
}
