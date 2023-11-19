use crate::store::Store;
use crate::validation::{ValidateDataIntegrity, ValidateDataIntegrityWithNamespace};
use platform_errors::Error;
use rob::{
    tenant::{NewTenant, Tenant, UpdateTenant},
    token::TokenContext,
    ValidateInputRules,
};
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_tenant(
    ctx: TokenContext,
    store: Arc<dyn Store>,
    new_tenant: NewTenant,
) -> Result<impl warp::Reply, warp::Rejection> {
    if ctx.namespace != new_tenant.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    new_tenant
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    new_tenant.validate_data_integrity(store.clone()).await?;
    let tenant = Tenant::new_from_obj(&new_tenant);
    store.add_tenant(&tenant).await?;
    Ok(with_status(json(&tenant), StatusCode::CREATED))
}

pub async fn update_tenant(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
    update_tenant: UpdateTenant,
) -> Result<impl warp::Reply, warp::Rejection> {
    update_tenant
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    update_tenant
        .validate_data_integrity(store.clone(), ctx.namespace.clone())
        .await?;
    let mut tenant = store.get_tenant(id.clone()).await?;
    if ctx.namespace != tenant.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    tenant.apply_update(&update_tenant);
    store.update_tenant(id, &tenant).await?;
    Ok(json(&tenant))
}

pub async fn subscribe_tenant_to_product(
    tenant_id: String,
    product_id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_tenant = store.get_tenant(tenant_id.clone()).await?;
    if ctx.namespace.clone() != existing_tenant.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    let _existing_product = store.get_product(ctx.namespace, product_id.clone()).await?;
    if existing_tenant
        .subscribed_products
        .contains(&product_id.clone())
    {
        return Err(Error::NotModified.into());
    }
    let updated_tenant = store
        .subscribe_tenant_to_product(tenant_id, product_id)
        .await?;
    Ok(json(&updated_tenant))
}

pub async fn get_tenant(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let tenant = store.get_tenant(id).await?;
    if ctx.namespace != tenant.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    Ok(json(&tenant))
}

pub async fn get_tenants(
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let tenants = store.get_tenants(ctx.namespace).await?;
    Ok(json(&tenants))
}

pub async fn delete_tenant(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_tenant = store.get_tenant(id.clone()).await?;
    if ctx.namespace != existing_tenant.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    store.delete_tenant(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
