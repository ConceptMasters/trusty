use crate::store::Store;
use crate::validation::ValidateDataIntegrity;
use platform_errors::Error;
use rob::{
    product::{NewProduct, Product, UpdateProduct},
    token::TokenContext,
    ValidateInputRules,
};
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_product(
    ctx: TokenContext,
    store: Arc<dyn Store>,
    new_product: NewProduct,
) -> Result<impl warp::Reply, warp::Rejection> {
    if ctx.namespace != new_product.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    new_product
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    new_product.validate_data_integrity(store.clone()).await?;
    let product = Product::new_from_obj(&new_product);
    store.add_product(&product).await?;
    Ok(with_status(json(&product), StatusCode::CREATED))
}

pub async fn update_product(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
    update_product: UpdateProduct,
) -> Result<impl warp::Reply, warp::Rejection> {
    update_product
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    let mut product = store.get_product(ctx.namespace.clone(), id.clone()).await?;
    if ctx.namespace != product.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    product.apply_update(&update_product);
    store.update_product(ctx.namespace, id, &product).await?;
    Ok(json(&product))
}

pub async fn get_product(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let product = store.get_product(ctx.namespace.clone(), id).await?;
    if ctx.namespace != product.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    Ok(json(&product))
}

pub async fn get_products(
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO apply namespace from token to filter options
    let products = store.get_products(ctx.namespace).await?;
    Ok(json(&products))
}

pub async fn delete_product(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_product = store.get_product(ctx.namespace.clone(), id.clone()).await?;
    if ctx.namespace.clone() != existing_product.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    store.delete_product(ctx.namespace, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
