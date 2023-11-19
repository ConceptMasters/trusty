mod common;

use common::{get_new_token, TEST_NAMESPACE_ID, TRUSTY_BASE_URL};

use pretty_assertions::assert_eq;
use serde_json::Value;
use std::collections::HashMap;
use ulid::Ulid;

use rob::tenant::Tenant;

const TEST_PRODUCT_ID_1: &str = "test-product-1";
const TEST_PRODUCT_ID_2: &str = "test-product-2";

async fn prepare_db_for_tests(token: String) {
    let _delete_namespace_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/namespaces/{}",
            TRUSTY_BASE_URL, TEST_NAMESPACE_ID
        ))
        .bearer_auth(token.clone())
        .send()
        .await;
    let _delete_product_1_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID_1
        ))
        .bearer_auth(token.clone())
        .send()
        .await;
    let _delete_product_2_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID_2
        ))
        .bearer_auth(token.clone())
        .send()
        .await;
}

#[tokio::test]
#[ignore]
async fn namespace() {
    // Grab a token
    let token = get_new_token().await.unwrap();

    // Clean up db before tests
    prepare_db_for_tests(token.clone()).await;

    // Create a namespace for testing
    let new_namespace: HashMap<&str, &str> = vec![("id", TEST_NAMESPACE_ID)].into_iter().collect();
    let create_namespace_response = reqwest::Client::new()
        .post(format!("{}/v1/namespaces", TRUSTY_BASE_URL))
        .bearer_auth(token.clone())
        .json(&new_namespace)
        .send()
        .await
        .unwrap();
    let create_namespace_response_status = create_namespace_response.status();
    println!(
        "New namespace for testing: {:#?}",
        create_namespace_response.text().await.unwrap()
    );
    assert_eq!(create_namespace_response_status, 201);

    // Create products for testing
    let mut create_test_product_1_data_str = r#"{
        "id": ""#
        .to_string();
    create_test_product_1_data_str.push_str(TEST_PRODUCT_ID_1);
    create_test_product_1_data_str.push_str(r#"", "namespace_id": ""#);
    create_test_product_1_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_product_1_data_str.push_str(
        r#"",
        "name": "name",
        "description": "description",
        "img": "img",
        "url": "url",
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
    assert_eq!(create_test_product_1_response_status, 201);
    println!(
        "New product for testing: {:#?}",
        create_test_product_1_response.text().await.unwrap()
    );

    let mut create_test_product_2_data_str = r#"{
        "id": ""#
        .to_string();
    create_test_product_2_data_str.push_str(TEST_PRODUCT_ID_2);
    create_test_product_2_data_str.push_str(r#"", "namespace_id": ""#);
    create_test_product_2_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_product_2_data_str.push_str(
        r#"",
        "name": "name",
        "description": "description",
        "img": "img",
        "url": "url",
        "can_self_register": true
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
    assert_eq!(create_test_product_2_response_status, 201);
    println!(
        "New product for testing: {:#?}",
        create_test_product_2_response.text().await.unwrap()
    );

    // Test creating a tenant
    let mut create_test_tenant_1_data_str = r#"{
        "namespace_id": ""#
        .to_string();
    create_test_tenant_1_data_str.push_str(TEST_NAMESPACE_ID);
    create_test_tenant_1_data_str.push_str(
        r#"",
        "name": "name",
        "description": "description",
        "subscribed_products": []
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
    let new_test_tenant_1: Tenant = create_test_tenant_1_response.json().await.unwrap();
    println!("New tenant: {:#?}", new_test_tenant_1);
    assert_eq!(new_test_tenant_1.namespace_id, TEST_NAMESPACE_ID);
    assert_eq!(new_test_tenant_1.name, "name");
    assert_eq!(new_test_tenant_1.description, "description");
    assert_eq!(new_test_tenant_1.metadata, None);
    let empty_string_vec: Vec<String> = vec![];
    assert_eq!(new_test_tenant_1.subscribed_products, empty_string_vec);
    // Test that this generated id is a ULID
    let ulid_id = Ulid::from_string(new_test_tenant_1.id.as_str());
    assert!(ulid_id.is_ok());
    assert_eq!(ulid_id.unwrap().to_string(), new_test_tenant_1.id);

    // Test updating a tenant
    let mut update_test_tenant_1_data_str = r#"{
        "description": "updated description",
        "metadata": {"a": [true]},
        "subscribed_products": [""#
        .to_string();
    update_test_tenant_1_data_str.push_str(TEST_PRODUCT_ID_1);
    update_test_tenant_1_data_str.push_str(
        r#""]
    }"#,
    );
    let update_test_tenant_1_json: Value =
        serde_json::from_str(update_test_tenant_1_data_str.as_str()).unwrap();
    let update_test_tenant_1_response = reqwest::Client::new()
        .patch(format!(
            "{}/v1/tenants/{}",
            TRUSTY_BASE_URL, new_test_tenant_1.id
        ))
        .bearer_auth(token.clone())
        .json(&update_test_tenant_1_json)
        .send()
        .await
        .unwrap();
    let update_test_tenant_1_response_status = update_test_tenant_1_response.status();
    assert_eq!(update_test_tenant_1_response_status, 200);
    let updated_test_tenant_1: Tenant = update_test_tenant_1_response.json().await.unwrap();
    println!("Updated tenant: {:#?}", updated_test_tenant_1);
    assert_eq!(updated_test_tenant_1.id, new_test_tenant_1.id);
    assert_eq!(
        updated_test_tenant_1.namespace_id,
        new_test_tenant_1.namespace_id
    );
    assert_eq!(updated_test_tenant_1.name, new_test_tenant_1.name);
    assert_eq!(updated_test_tenant_1.description, "updated description");
    assert_eq!(
        updated_test_tenant_1.metadata,
        serde_json::from_str(r#"{"a": [true]}"#).unwrap()
    );
    assert_eq!(
        updated_test_tenant_1.subscribed_products,
        serde_json::from_str::<Vec<String>>(format!(r#"["{}"]"#, TEST_PRODUCT_ID_1).as_str())
            .unwrap()
    );

    // Test calling the subscribe tenant to product endopint
    let subscribe_tenant_to_product_response = reqwest::Client::new()
        .patch(format!(
            "{}/v1/tenants/{}/subscribe/{}",
            TRUSTY_BASE_URL, new_test_tenant_1.id, TEST_PRODUCT_ID_2
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let subscribe_tenant_to_product_response_status = subscribe_tenant_to_product_response.status();
    assert_eq!(subscribe_tenant_to_product_response_status, 200);

    // Test getting a tenant
    let get_tenant_response = reqwest::Client::new()
        .get(format!(
            "{}/v1/tenants/{}",
            TRUSTY_BASE_URL, new_test_tenant_1.id
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let get_test_tenant_1_response_status = get_tenant_response.status();
    assert_eq!(get_test_tenant_1_response_status, 200);
    let get_test_tenant_1_response_object: Tenant = get_tenant_response.json().await.unwrap();
    println!("Fetched tenant: {:#?}", get_test_tenant_1_response_object);
    assert_eq!(get_test_tenant_1_response_object.id, new_test_tenant_1.id);
    assert_eq!(
        get_test_tenant_1_response_object.namespace_id,
        TEST_NAMESPACE_ID
    );
    assert_eq!(
        get_test_tenant_1_response_object.name,
        new_test_tenant_1.name
    );
    assert_eq!(
        get_test_tenant_1_response_object.description,
        updated_test_tenant_1.description
    );
    assert_eq!(
        get_test_tenant_1_response_object.metadata,
        updated_test_tenant_1.metadata
    );
    assert_eq!(
        get_test_tenant_1_response_object.subscribed_products,
        serde_json::from_str::<Vec<String>>(
            format!(r#"["{}", "{}"]"#, TEST_PRODUCT_ID_1, TEST_PRODUCT_ID_2).as_str()
        )
        .unwrap()
    );

    // Test deleting a tenant
    let delete_tenant_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/tenants/{}",
            TRUSTY_BASE_URL, new_test_tenant_1.id
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let delete_tenant_response_status = delete_tenant_response.status();
    assert_eq!(delete_tenant_response_status, 204);
    println!("Deleted tenant with id: {}", new_test_tenant_1.id);

    // Delete the products created for testing
    let delete_product_1_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID_1
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let delete_product_1_response_status = delete_product_1_response.status();
    assert_eq!(delete_product_1_response_status, 204);
    println!("Deleted product: {}", TEST_PRODUCT_ID_1);

    let delete_product_2_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID_2
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let delete_product_2_response_status = delete_product_2_response.status();
    assert_eq!(delete_product_2_response_status, 204);
    println!("Deleted product: {}", TEST_PRODUCT_ID_2);

    // Delete the namespace created for testing
    let delete_namespace_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/namespaces/{}",
            TRUSTY_BASE_URL, TEST_NAMESPACE_ID
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let delete_namespace_response_status = delete_namespace_response.status();
    assert_eq!(delete_namespace_response_status, 204);
    println!("Deleted namespace: {}", TEST_NAMESPACE_ID);
}
