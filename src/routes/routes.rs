use super::*;
use crate::store::Store;
use platform_errors::return_error;
use race::AccessControlEngine;
use rob::{token::TokenContext, user::UserQuery};
use std::sync::Arc;
use warp::{http::Method, Filter, Rejection};

pub fn router(
    store: Arc<dyn Store>,
    race_core: Arc<AccessControlEngine>,
    with_jwt: impl Filter<Extract = (TokenContext,), Error = Rejection> + Clone + Send + Sync + 'static,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let with_store = warp::any().map(move || store.clone());
    let with_access_control_core_access = warp::any().map(move || race_core.clone());

    macro_rules! route_with_body {
        ($method:ident, $function_name:ident, $path:expr) => {
            warp::$method()
                .and($path)
                .and(warp::path::end())
                .and(with_jwt.clone())
                .and(with_store.clone())
                .and(warp::body::json())
                .and_then($function_name)
        };
    }

    macro_rules! route_without_body {
        ($method:ident, $function_name:ident, $path:expr) => {
            warp::$method()
                .and($path)
                .and(warp::path::end())
                .and(with_jwt.clone())
                .and(with_store.clone())
                .and_then($function_name)
        };
    }

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Content-Type",
            "Authorization",
            "user-id",
            "tenant-id",
            "namespace-id",
        ])
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::PATCH]);

    let healthz_route = warp::get()
        .and(warp::path("healthz"))
        .and(warp::path::end())
        .and_then(healthz);

    let is_allowed_route = warp::post()
        .and(warp::path!("v1" / "isallowed"))
        .and(warp::path::end())
        .and(with_jwt.clone())
        .and(with_store.clone())
        .and(with_access_control_core_access.clone())
        .and(warp::body::json())
        .and_then(is_allowed);

    // namespaces
    let add_namespace_route =
        route_with_body!(post, add_namespace, warp::path!("v1" / "namespaces"));
    let get_namespace_route = route_without_body!(
        get,
        get_namespace,
        warp::path!("v1" / "namespaces" / String)
    );
    let get_namespaces_route =
        route_without_body!(get, get_namespaces, warp::path!("v1" / "namespaces"));
    let delete_namespace_route = route_without_body!(
        delete,
        delete_namespace,
        warp::path!("v1" / "namespaces" / String)
    );

    // products
    let add_product_route = route_with_body!(post, add_product, warp::path!("v1" / "products"));
    let update_product_route = route_with_body!(
        patch,
        update_product,
        warp::path!("v1" / "products" / String)
    );
    let get_product_route =
        route_without_body!(get, get_product, warp::path!("v1" / "products" / String));
    let get_products_route = route_without_body!(get, get_products, warp::path!("v1" / "products"));
    let delete_product_route = route_without_body!(
        delete,
        delete_product,
        warp::path!("v1" / "products" / String)
    );

    // tenants
    let add_tenant_route = route_with_body!(post, add_tenant, warp::path!("v1" / "tenants"));
    let update_tenant_route =
        route_with_body!(patch, update_tenant, warp::path!("v1" / "tenants" / String));
    let subscribe_tenant_to_product_route = route_without_body!(
        patch,
        subscribe_tenant_to_product,
        warp::path!("v1" / "tenants" / String / "subscribe" / String)
    );
    let get_tenant_route =
        route_without_body!(get, get_tenant, warp::path!("v1" / "tenants" / String));
    let get_tenants_route = route_without_body!(get, get_tenants, warp::path!("v1" / "tenants"));
    let delete_tenant_route = route_without_body!(
        delete,
        delete_tenant,
        warp::path!("v1" / "tenants" / String)
    );

    // organization profiles
    let add_organization_profile_route = route_with_body!(
        post,
        add_organization_profile,
        warp::path!("v1" / "organization-profiles")
    );
    let update_organization_profile_route = route_with_body!(
        patch,
        update_organization_profile,
        warp::path!("v1" / "organization-profiles" / String)
    );
    let get_organization_profile_route = route_without_body!(
        get,
        get_organization_profile,
        warp::path!("v1" / "organization-profiles" / String)
    );
    let get_organization_profiles_route = route_without_body!(
        get,
        get_organization_profiles,
        warp::path!("v1" / "organization-profiles")
    );
    let delete_organization_profile_route = route_without_body!(
        delete,
        delete_organization_profile,
        warp::path!("v1" / "organization-profiles" / String)
    );

    // users
    let add_user_route = route_with_body!(post, add_user, warp::path!("v1" / "users"));
    let update_user_route =
        route_with_body!(patch, update_user, warp::path!("v1" / "users" / String));
    let associate_user_with_tenant_route = route_without_body!(
        patch,
        associate_user_with_tenant,
        warp::path!("v1" / "users" / String / "associate" / String)
    );
    let get_user_route = route_without_body!(get, get_user, warp::path!("v1" / "users" / String));
    let get_users_route = warp::get()
        .and(warp::path!("v1" / "users"))
        .and(warp::path::end())
        .and(with_jwt.clone())
        .and(with_store.clone())
        .and(warp::query::<UserQuery>())
        .and_then(get_users);
    let delete_user_route =
        route_without_body!(delete, delete_user, warp::path!("v1" / "users" / String));
    let get_user_info_route =
        route_without_body!(get, get_user_info, warp::path!("v1" / "userinfo" / String));

    // roles
    let add_role_route = route_with_body!(post, add_role, warp::path!("v1" / "roles"));
    let update_role_route =
        route_with_body!(patch, update_role, warp::path!("v1" / "roles" / String));
    let get_role_route = route_without_body!(get, get_role, warp::path!("v1" / "roles" / String));
    let get_roles_route = route_without_body!(get, get_roles, warp::path!("v1" / "roles"));
    let delete_role_route =
        route_without_body!(delete, delete_role, warp::path!("v1" / "roles" / String));

    healthz_route
        .or(is_allowed_route)
        // namespaces
        .or(add_namespace_route)
        .or(get_namespace_route)
        .or(get_namespaces_route)
        .or(delete_namespace_route)
        // products
        .or(add_product_route)
        .or(update_product_route)
        .or(get_product_route)
        .or(get_products_route)
        .or(delete_product_route)
        // tenants
        .or(add_tenant_route)
        .or(update_tenant_route)
        .or(subscribe_tenant_to_product_route)
        .or(get_tenant_route)
        .or(get_tenants_route)
        .or(delete_tenant_route)
        // organization profiles
        .or(add_organization_profile_route)
        .or(update_organization_profile_route)
        .or(get_organization_profile_route)
        .or(get_organization_profiles_route)
        .or(delete_organization_profile_route)
        // users
        .or(add_user_route)
        .or(update_user_route)
        .or(associate_user_with_tenant_route)
        .or(get_user_route)
        .or(get_users_route)
        .or(delete_user_route)
        .or(get_user_info_route)
        // roles
        .or(add_role_route)
        .or(update_role_route)
        .or(get_role_route)
        .or(get_roles_route)
        .or(delete_role_route)
        // finish up
        .with(cors)
        .recover(return_error)
}
