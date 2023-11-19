use crate::store::Store;
use crate::validation::ValidateDataIntegrity;
use platform_errors::Error;
use rob::{
    organizationprofile::{NewOrganizationProfile, OrganizationProfile, UpdateOrganizationProfile},
    token::TokenContext,
    ValidateInputRules,
};
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
};

pub async fn add_organization_profile(
    ctx: TokenContext,
    store: Arc<dyn Store>,
    new_organization_profile: NewOrganizationProfile,
) -> Result<impl warp::Reply, warp::Rejection> {
    if ctx.namespace != new_organization_profile.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    new_organization_profile
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    new_organization_profile
        .validate_data_integrity(store.clone())
        .await?;
    let organization_profile = OrganizationProfile::new_from_obj(&new_organization_profile);
    store
        .add_organization_profile(&organization_profile)
        .await?;
    Ok(with_status(
        json(&organization_profile),
        StatusCode::CREATED,
    ))
}

pub async fn update_organization_profile(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
    update_organization_profile: UpdateOrganizationProfile,
) -> Result<impl warp::Reply, warp::Rejection> {
    update_organization_profile
        .validate_input_rules()
        .map_err(|e| Error::ValidationError(e.to_string()))?;
    let mut organization_profile = store.get_organization_profile(id.clone()).await?;
    if ctx.namespace != organization_profile.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    organization_profile.apply_update(&update_organization_profile);
    store
        .update_organization_profile(id, &organization_profile)
        .await?;
    Ok(json(&organization_profile))
}

pub async fn get_organization_profile(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let organization_profile = store.get_organization_profile(id).await?;
    if ctx.namespace != organization_profile.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    Ok(json(&organization_profile))
}

pub async fn get_organization_profiles(
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let organization_profiles = store.get_organization_profiles(ctx.namespace).await?;
    Ok(json(&organization_profiles))
}

pub async fn delete_organization_profile(
    id: String,
    ctx: TokenContext,
    store: Arc<dyn Store>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let existing_organization_profile = store.get_organization_profile(id.clone()).await?;
    if ctx.namespace != existing_organization_profile.namespace_id.clone() {
        return Err(Error::Unauthorized.into());
    }
    store.delete_organization_profile(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
