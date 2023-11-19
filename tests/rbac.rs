mod common;

use common::{get_new_token, TEST_NAMESPACE_ID, TRUSTY_BASE_URL};

use pretty_assertions::assert_eq;
use serde_json::Value;
use std::collections::HashMap;

use rob::{
    rbac::{IsAllowedRequest, IsAllowedResult},
    role::Role,
    tenant::Tenant,
    user::User,
};

/*
Creates the following for the tests:
- Namespace with id "portal"
- Product with id "test-product-1"
- Product with id "test-product-2"
- Tenant subscribed to "test-product-1" and "test-product-2"
- Tenant subscribed to "test-product-1"
- Tenant subscribed to no products
- Role with
- Role with
- User with
- User with

Returns a tuple containing (in this order):
- Id of created namespace
- Vector of ids of created products
- Vector of ids of created tenants
- Vector of external ids of created users
- Vector of ids of created roles
 */
async fn create_rbac_test_objects(
    token: String,
) -> (String, Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let mut created_tenant_ids: Vec<String> = vec![];
    let mut created_role_ids: Vec<String> = vec![];
    let mut created_external_user_ids: Vec<String> = vec![];

    // Create portal namespace
    //////////////////////////
    let new_namespace: HashMap<&str, &str> = vec![("id", TEST_NAMESPACE_ID)].into_iter().collect();
    let create_namespace_response = reqwest::Client::new()
        .post(format!("{}/v1/namespaces", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&new_namespace)
        .send()
        .await
        .unwrap();
    let create_namespace_response_status = create_namespace_response.status();
    println!("{:#?}", create_namespace_response.text().await.unwrap());
    assert_eq!(create_namespace_response_status, 201);

    // Create test products
    ///////////////////////
    // Create rbac-test-product-1
    let mut create_test_product_1_data_str = r#"{
        "id": "rbac-test-product-1",
        "namespace_id": ""#
        .to_string();
    create_test_product_1_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_product_1_data_str.push_str(
        r#"",
        "name": "rbac test product 1",
        "description": "rbac test product 1",
        "img": "does not matter",
        "url": "does not matter",
        "can_self_register": true
    }"#,
    );
    let create_test_product_1_json: Value =
        serde_json::from_str(create_test_product_1_data_str.as_str()).unwrap();
    let create_test_product_1_response = reqwest::Client::new()
        .post(format!("{}/v1/products", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_product_1_json)
        .send()
        .await
        .unwrap();
    let create_test_product_1_response_status = create_test_product_1_response.status();
    println!(
        "{:#?}",
        create_test_product_1_response.text().await.unwrap()
    );
    assert_eq!(create_test_product_1_response_status, 201);

    // Create rbac-test-product-2
    let mut create_test_product_2_data_str = r#"{
        "id": "rbac-test-product-2",
        "namespace_id": ""#
        .to_string();
    create_test_product_2_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_product_2_data_str.push_str(
        r#"",
        "name": "rbac test product 2",
        "description": "rbac test product 2",
        "img": "does not matter",
        "url": "does not matter",
        "can_self_register": false
    }"#,
    );
    let create_test_product_2_json: Value =
        serde_json::from_str(create_test_product_2_data_str.as_str()).unwrap();
    let create_test_product_2_response = reqwest::Client::new()
        .post(format!("{}/v1/products", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_product_2_json)
        .send()
        .await
        .unwrap();
    let create_test_product_2_response_status = create_test_product_2_response.status();
    println!(
        "{:#?}",
        create_test_product_2_response.text().await.unwrap()
    );
    assert_eq!(create_test_product_2_response_status, 201);

    // Create test tenants
    ///////////////////////
    // Create rbac test tenant 1
    let mut create_test_tenant_1_data_str = r#"{
        "namespace_id": ""#
        .to_string();
    create_test_tenant_1_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_tenant_1_data_str.push_str(
        r#"",
        "name": "rbac test tenant 1",
        "description": "rbac test tenant 1",
        "metadata": null,
        "subscribed_products": ["rbac-test-product-1", "rbac-test-product-2"]
    }"#,
    );
    let create_test_tenant_1_json: Value =
        serde_json::from_str(create_test_tenant_1_data_str.as_str()).unwrap();
    let create_test_tenant_1_response = reqwest::Client::new()
        .post(format!("{}/v1/tenants", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_tenant_1_json)
        .send()
        .await
        .unwrap();
    let create_test_tenant_1_response_status = create_test_tenant_1_response.status();
    assert_eq!(create_test_tenant_1_response_status, 201);
    let returned_tenant_1: Tenant = create_test_tenant_1_response.json().await.unwrap();
    created_tenant_ids.push(returned_tenant_1.id);

    // Create rbac test tenant 2
    let mut create_test_tenant_2_data_str = r#"{
        "namespace_id": ""#
        .to_string();
    create_test_tenant_2_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_tenant_2_data_str.push_str(
        r#"",
        "name": "rbac test tenant 2",
        "description": "rbac test tenant 2",
        "metadata": null,
        "subscribed_products": ["rbac-test-product-1"]
    }"#,
    );
    let create_test_tenant_2_json: Value =
        serde_json::from_str(create_test_tenant_2_data_str.as_str()).unwrap();
    let create_test_tenant_2_response = reqwest::Client::new()
        .post(format!("{}/v1/tenants", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_tenant_2_json)
        .send()
        .await
        .unwrap();
    let create_test_tenant_2_response_status = create_test_tenant_2_response.status();
    assert_eq!(create_test_tenant_2_response_status, 201);
    let returned_tenant_2: Tenant = create_test_tenant_2_response.json().await.unwrap();
    created_tenant_ids.push(returned_tenant_2.id);

    // Create rbac test tenant 3
    let mut create_test_tenant_3_data_str = r#"{
        "namespace_id": ""#
        .to_string();
    create_test_tenant_3_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_tenant_3_data_str.push_str(
        r#"",
        "name": "rbac test tenant 3",
        "description": "rbac test tenant 3",
        "metadata": null,
        "subscribed_products": []
    }"#,
    );
    let create_test_tenant_3_json: Value =
        serde_json::from_str(create_test_tenant_3_data_str.as_str()).unwrap();
    let create_test_tenant_3_response = reqwest::Client::new()
        .post(format!("{}/v1/tenants", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_tenant_3_json)
        .send()
        .await
        .unwrap();
    let create_test_tenant_3_response_status = create_test_tenant_3_response.status();
    assert_eq!(create_test_tenant_3_response_status, 201);
    let returned_tenant_3: Tenant = create_test_tenant_3_response.json().await.unwrap();
    created_tenant_ids.push(returned_tenant_3.id);

    // Create test roles
    ////////////////////
    // Create rbac test role 1
    let mut create_test_role_1_data_str = r#"{
        "namespace_id": ""#
        .to_string();
    create_test_role_1_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_role_1_data_str.push_str(
        r#"",
        "name": "rbac test role 1",
        "description": "rbac test role 1",
        "metadata": null,
        "permissions": ["resource1:action1"],
        "tenant_id": ""#,
    );
    create_test_role_1_data_str.push_str(created_tenant_ids[0].as_str());
    create_test_role_1_data_str.push_str(
        r#"",
        "product_id": "rbac-test-product-1"
    }"#,
    );
    //println!("JSON to create role 1: {}", create_test_role_1_data_str);
    let create_test_role_1_json: Value =
        serde_json::from_str(create_test_role_1_data_str.as_str()).unwrap();
    let create_test_role_1_response = reqwest::Client::new()
        .post(format!("{}/v1/roles", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_role_1_json)
        .send()
        .await
        .unwrap();
    let create_test_role_1_response_status = create_test_role_1_response.status();
    assert_eq!(create_test_role_1_response_status, 201);
    let returned_role_1: Role = create_test_role_1_response.json().await.unwrap();
    created_role_ids.push(returned_role_1.id);

    // Create test users
    ///////////////////////
    // Create user with external id "rbac-test-user-1"
    let mut create_test_user_1_data_str = r#"{
        "namespace_id": ""#
        .to_string();
    create_test_user_1_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_user_1_data_str.push_str(
        r#"",
        "email": "rbactestemail1@email.com",
        "external_provider": {
          "provider_type": "auth0",
          "id": "rbac-test-user-1"
        },
        "first_name": "hi",
        "last_name": "there",
        "is_active": true,
        "is_invited": false,
        "metadata": null,
        "associated_tenants": [""#,
    );
    create_test_user_1_data_str.push_str(created_tenant_ids[0].as_str());
    create_test_user_1_data_str.push_str(
        r#""],
        "roles": [""#,
    );
    create_test_user_1_data_str.push_str(created_role_ids[0].as_str());
    create_test_user_1_data_str.push_str(
        r#""]
    }"#,
    );
    println!("JSON to create user: {}", create_test_user_1_data_str);
    let create_test_user_1_json: Value =
        serde_json::from_str(create_test_user_1_data_str.as_str()).unwrap();
    let create_test_user_1_response = reqwest::Client::new()
        .post(format!("{}/v1/users", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_user_1_json)
        .send()
        .await
        .unwrap();
    let create_test_user_1_response_status = create_test_user_1_response.status();
    //println!("{:#?}", create_test_user_1_response.text().await.unwrap());
    assert_eq!(create_test_user_1_response_status, 201);
    let returned_user_1: User = create_test_user_1_response.json().await.unwrap();
    created_external_user_ids.push(returned_user_1.external_provider.id);

    // Create user with external id "rbac-test-user-2"
    let mut create_test_user_2_data_str = r#"{
        "namespace_id": ""#
        .to_string();
    create_test_user_2_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_user_2_data_str.push_str(
        r#"",
        "email": "rbactestemail2@email.com",
        "external_provider": {
          "provider_type": "auth0",
          "id": "rbac-test-user-2"
        },
        "first_name": "hi",
        "last_name": "there",
        "is_active": true,
        "is_invited": false,
        "metadata": null,
        "associated_tenants": [""#,
    );
    create_test_user_2_data_str.push_str(created_tenant_ids[0].as_str());
    create_test_user_2_data_str.push_str(
        r#""],
        "roles": []
    }"#,
    );
    println!("JSON to create user: {}", create_test_user_2_data_str);
    let create_test_user_2_json: Value =
        serde_json::from_str(create_test_user_2_data_str.as_str()).unwrap();
    let create_test_user_2_response = reqwest::Client::new()
        .post(format!("{}/v1/users", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&create_test_user_2_json)
        .send()
        .await
        .unwrap();
    let create_test_user_2_response_status = create_test_user_2_response.status();
    assert_eq!(create_test_user_2_response_status, 201);
    let returned_user_2: User = create_test_user_2_response.json().await.unwrap();
    created_external_user_ids.push(returned_user_2.external_provider.id);

    // Return created object ids
    (
        "portal".into(),
        vec!["rbac-test-product-1".into(), "rbac-test-product-2".into()],
        created_tenant_ids,
        created_role_ids,
        created_external_user_ids,
    )
}

async fn prepare_db_for_rbac_tests(token: String) {
    let _delete_namespace_response = reqwest::Client::new()
        .delete(format!("{}/v1/namespaces/portal", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .send()
        .await;
    let _delete_test_product_1_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/rbac-test-product-1",
            TRUSTY_BASE_URL
        ))
        .bearer_auth(token.clone())
        .send()
        .await;
    let _delete_test_product_2_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/rbac-test-product-2",
            TRUSTY_BASE_URL
        ))
        .bearer_auth(token.clone())
        .send()
        .await;
}

// Do not unwrap any of these because they might fail if they were already deleted or never created
async fn clean_up_rbac_test_objects(
    token: String,
    created_namespace: String,
    created_product_ids: Vec<String>,
    created_tenant_ids: Vec<String>,
    created_role_ids: Vec<String>,
    created_external_user_ids: Vec<String>,
) {
    let _delete_namespace_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/namespaces/{}",
            TRUSTY_BASE_URL, created_namespace
        ))
        .bearer_auth(token.clone())
        .send()
        .await;
    for product in &created_product_ids {
        let _delete_test_product_response = reqwest::Client::new()
            .delete(format!("{}/v1/products/{}", TRUSTY_BASE_URL, product))
            .bearer_auth(token.clone())
            .send()
            .await;
    }
    for tenant in &created_tenant_ids {
        let _delete_test_tenant_response = reqwest::Client::new()
            .delete(format!("{}/v1/tenants/{}", TRUSTY_BASE_URL, tenant))
            .bearer_auth(token.clone())
            .send()
            .await;
    }
    for role in &created_role_ids {
        let _delete_test_role_response = reqwest::Client::new()
            .delete(format!("{}/v1/roles/{}", TRUSTY_BASE_URL, role))
            .bearer_auth(token.clone())
            .send()
            .await;
    }
    for user in &created_external_user_ids {
        let _delete_test_user_response = reqwest::Client::new()
            .delete(format!("{}/v1/users/{}", TRUSTY_BASE_URL, user))
            .bearer_auth(token.clone())
            .send()
            .await;
    }
}

#[tokio::test]
#[ignore]
async fn rbac() {
    // Grab a token
    let token = get_new_token().await.unwrap();

    prepare_db_for_rbac_tests(token.clone()).await;

    // Create testing objects
    let (
        created_namespace,
        created_product_ids,
        created_tenant_ids,
        created_role_ids,
        created_external_user_ids,
    ) = create_rbac_test_objects(token.clone()).await;

    // Now we can test RBAC
    ///////////////////////
    // Test that user 1 has access via the assigned role
    let rbac_test_1_request_obj = IsAllowedRequest {
        external_user_id: created_external_user_ids[0].clone(),
        tenant: created_tenant_ids[0].clone(),
        product: created_product_ids[0].clone(),
        resource: "resource1".into(),
        action: "action1".into(),
    };
    let rbac_test_1_is_allowed_response = reqwest::Client::new()
        .post(format!("{}/v1/isallowed", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&rbac_test_1_request_obj)
        .send()
        .await
        .unwrap();
    let rbac_test_1_is_allowed_response_status = rbac_test_1_is_allowed_response.status();
    //println!("{:#?}", create_test_user_1_response.text().await.unwrap());
    assert_eq!(rbac_test_1_is_allowed_response_status, 200);
    let rbac_test_1_result: IsAllowedResult = rbac_test_1_is_allowed_response.json().await.unwrap();
    assert!(rbac_test_1_result.result);

    // Test that user 2 does not have access because is has no roles associated
    let rbac_test_2_request_obj = IsAllowedRequest {
        external_user_id: created_external_user_ids[1].clone(),
        tenant: created_tenant_ids[0].clone(),
        product: created_product_ids[0].clone(),
        resource: "resource1".into(),
        action: "action1".into(),
    };
    let rbac_test_2_is_allowed_response = reqwest::Client::new()
        .post(format!("{}/v1/isallowed", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&rbac_test_2_request_obj)
        .send()
        .await
        .unwrap();
    let rbac_test_2_is_allowed_response_status = rbac_test_2_is_allowed_response.status();
    //println!("{:#?}", create_test_user_2_response.text().await.unwrap());
    assert_eq!(rbac_test_2_is_allowed_response_status, 200);
    let rbac_test_2_result: IsAllowedResult = rbac_test_2_is_allowed_response.json().await.unwrap();
    assert!(!rbac_test_2_result.result);

    // Clean up objects we created from this test
    clean_up_rbac_test_objects(
        token.clone(),
        created_namespace,
        created_product_ids,
        created_tenant_ids,
        created_role_ids,
        created_external_user_ids,
    )
    .await;
}
