mod common;

use common::{get_new_token, TEST_NAMESPACE_ID, TRUSTY_BASE_URL};

use pretty_assertions::assert_eq;
use serde_json::Value;
use std::collections::HashMap;

use rob::product::Product;

const TEST_PRODUCT_ID: &str = "test-product-1";

async fn prepare_db_for_tests(token: String) {
    let _delete_namespace_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/namespaces/{}",
            TRUSTY_BASE_URL, TEST_NAMESPACE_ID
        ))
        .bearer_auth(token.clone())
        .send()
        .await;
    let _delete_product_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID
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
        "New namespace: {:#?}",
        create_namespace_response.text().await.unwrap()
    );
    assert_eq!(create_namespace_response_status, 201);

    // Test creating a product
    let mut create_test_product_1_data_str = r#"{
        "id": ""#
        .to_string();
    create_test_product_1_data_str.push_str(TEST_PRODUCT_ID);
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
    let created_test_product_1: Product = create_test_product_1_response.json().await.unwrap();
    println!("New product: {:#?}", created_test_product_1);
    assert_eq!(created_test_product_1.id, TEST_PRODUCT_ID);
    assert_eq!(created_test_product_1.namespace_id, TEST_NAMESPACE_ID);
    assert_eq!(created_test_product_1.name, "name");
    assert_eq!(created_test_product_1.description, "description");
    assert_eq!(created_test_product_1.img, "img");
    assert_eq!(created_test_product_1.url, "url");
    assert_eq!(created_test_product_1.can_self_register, true);

    // Test updating a product
    let update_test_product_1_data_str = r#"{
        "description": "updated description",
        "img": "updated img",
        "metadata": {"a": [true]}
    }"#;
    let update_test_product_1_json: Value =
        serde_json::from_str(update_test_product_1_data_str).unwrap();
    let update_test_product_1_response = reqwest::Client::new()
        .patch(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID
        ))
        .bearer_auth(token.clone())
        .json(&update_test_product_1_json)
        .send()
        .await
        .unwrap();
    let update_test_product_1_response_status = update_test_product_1_response.status();
    assert_eq!(update_test_product_1_response_status, 200);
    let updated_test_product_1: Product = update_test_product_1_response.json().await.unwrap();
    println!("Updated product: {:#?}", updated_test_product_1);
    assert_eq!(updated_test_product_1.id, TEST_PRODUCT_ID);
    assert_eq!(updated_test_product_1.namespace_id, TEST_NAMESPACE_ID);
    assert_eq!(updated_test_product_1.name, "name");
    assert_eq!(updated_test_product_1.description, "updated description");
    assert_eq!(updated_test_product_1.img, "updated img");
    assert_eq!(updated_test_product_1.url, "url");
    assert_eq!(updated_test_product_1.can_self_register, true);
    assert_eq!(
        updated_test_product_1.metadata,
        serde_json::from_str(r#"{"a": [true]}"#).unwrap()
    );

    // Test getting a product
    let get_product_response = reqwest::Client::new()
        .get(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let get_test_product_1_response_status = get_product_response.status();
    let get_test_product_1_response_object: Product = get_product_response.json().await.unwrap();
    println!("Fetched product: {:#?}", get_test_product_1_response_object);
    assert_eq!(get_test_product_1_response_status, 200);
    assert_eq!(get_test_product_1_response_object.id, TEST_PRODUCT_ID);
    assert_eq!(
        get_test_product_1_response_object.namespace_id,
        TEST_NAMESPACE_ID
    );
    assert_eq!(get_test_product_1_response_object.name, "name");
    assert_eq!(
        get_test_product_1_response_object.description,
        "updated description"
    );
    assert_eq!(get_test_product_1_response_object.img, "updated img");
    assert_eq!(get_test_product_1_response_object.url, "url");
    assert_eq!(get_test_product_1_response_object.can_self_register, true);
    assert_eq!(
        get_test_product_1_response_object.metadata,
        serde_json::from_str(r#"{"a": [true]}"#).unwrap()
    );

    // Test deleting a product
    let delete_product_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/products/{}",
            TRUSTY_BASE_URL, TEST_PRODUCT_ID
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let delete_product_response_status = delete_product_response.status();
    assert_eq!(delete_product_response_status, 204);
    println!("Deleted product: {}", TEST_PRODUCT_ID);

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
