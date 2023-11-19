use serde::Deserialize;
use serde_json::Value;

pub const TRUSTY_BASE_URL: &str = "http://localhost:9030";
const OAUTHOR_BASE_URL: &str = "http://localhost:9032";
const SKS_BASE_URL: &str = "http://localhost:9033";

const TEST_PRODUCT_ID: &str = "1";
const TEST_TENANT_ID: &str = "1";
pub const TEST_NAMESPACE_ID: &str = "portal-test";

#[derive(Deserialize, Debug, Clone)]
struct NewKeysResponse {
    id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct CredsResponse {
    client_id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct CredsMeta {
    scopes: Vec<String>,
    audience: String,
    associated_keys_id: String,
    namespace: String,
    tenant_id: String,
    product_id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct CredsWithSecretResponse {
    client_id: String,
    client_secret: String,
    meta: CredsMeta,
}

#[derive(Deserialize, Debug, Clone)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
}

pub async fn get_new_token() -> Result<String, reqwest::Error> {
    // Create keypair
    let create_keys_json_data_str = r#"{"meta": {"info": "test keys"}}"#;
    let create_keys_json: Value = serde_json::from_str(create_keys_json_data_str).unwrap();
    let create_keys_response: NewKeysResponse = reqwest::Client::new()
        .post(format!("{}/v1/keys", SKS_BASE_URL))
        .header("x-tenant-id", TEST_TENANT_ID)
        .header("x-product-id", TEST_PRODUCT_ID)
        .header("x-namespace", TEST_NAMESPACE_ID)
        .json(&create_keys_json)
        .send()
        .await?
        .json()
        .await?;
    println!("New kid: {}", create_keys_response.id);

    // Create creds
    let mut create_creds_json_data_str = r#"
      {
        "meta": {
          "scopes":[
            "trusty",
            "data"
          ],
          "audience": "https://up.somos.com/",
          "namespace": ""#
        .to_string();
    create_creds_json_data_str.push_str(TEST_NAMESPACE_ID);
    create_creds_json_data_str.push_str(r#"","tenant_id": ""#);
    create_creds_json_data_str.push_str(TEST_TENANT_ID);
    create_creds_json_data_str.push_str(r#"","product_id": ""#);
    create_creds_json_data_str.push_str(TEST_PRODUCT_ID);
    create_creds_json_data_str.push_str(r#"","associated_keys_id": ""#);
    create_creds_json_data_str.push_str(create_keys_response.id.as_str());
    create_creds_json_data_str.push_str(r#""}}"#);
    println!("Create creds JSON body: {}", create_creds_json_data_str);
    let create_creds_json: Value =
        serde_json::from_str(create_creds_json_data_str.as_str()).unwrap();
    let creds_response: CredsResponse = reqwest::Client::new()
        .post(format!("{}/v1/creds", SKS_BASE_URL))
        .header("x-tenant-id", TEST_TENANT_ID)
        .header("x-product-id", TEST_PRODUCT_ID)
        .header("x-namespace", TEST_NAMESPACE_ID)
        .json(&create_creds_json)
        .send()
        .await?
        .json()
        .await?;
    println!("New client id: {}", creds_response.client_id);

    // Get client secret
    let creds_with_secret_response: CredsWithSecretResponse = reqwest::Client::new()
        .get(format!(
            "{}/v1/creds/{}",
            SKS_BASE_URL, creds_response.client_id
        ))
        .header("x-tenant-id", TEST_TENANT_ID)
        .header("x-product-id", TEST_PRODUCT_ID)
        .header("x-namespace", TEST_NAMESPACE_ID)
        .send()
        .await?
        .json()
        .await?;
    println!(
        "New client secret: {}",
        creds_with_secret_response.client_secret
    );
    println!(
        "Full creds with secret response: {:?}",
        creds_with_secret_response
    );

    // Get jwt
    let token_request_params = [
        ("grant_type", "client_credentials"),
        ("client_id", creds_response.client_id.as_str()),
        (
            "client_secret",
            creds_with_secret_response.client_secret.as_str(),
        ),
    ];
    let token_response: TokenResponse = reqwest::Client::new()
        .post(format!("{}/token", OAUTHOR_BASE_URL))
        .form(&token_request_params)
        .send()
        .await?
        .json()
        .await?;
    println!("New JWT: {}", token_response.access_token);

    Ok(token_response.access_token)
}
