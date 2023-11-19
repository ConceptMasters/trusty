mod common;

use common::{get_new_token, TEST_NAMESPACE_ID, TRUSTY_BASE_URL};

use pretty_assertions::assert_eq;
use std::collections::HashMap;

use rob::namespace::Namespace;

async fn prepare_db_for_tests(token: String) {
    let _delete_namespace_response = reqwest::Client::new()
        .delete(format!(
            "{}/v1/namespaces/{}",
            TRUSTY_BASE_URL, TEST_NAMESPACE_ID
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

    // Test creating a namespace
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

    // Test getting a namespace
    let get_namespace_response = reqwest::Client::new()
        .get(format!(
            "{}/v1/namespaces/{}",
            TRUSTY_BASE_URL, TEST_NAMESPACE_ID
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .unwrap();
    let get_namespace_response_status = get_namespace_response.status();
    let get_namespace_response_object: Namespace = get_namespace_response.json().await.unwrap();
    println!("Fetched namespace: {:#?}", get_namespace_response_object);
    assert_eq!(get_namespace_response_status, 200);
    assert_eq!(TEST_NAMESPACE_ID, get_namespace_response_object.id);

    // Test deleting a namespace
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
