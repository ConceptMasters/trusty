use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAllowedRequest {
    pub external_user_id: String,
    pub tenant: String,
    pub product: String,
    pub resource: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IsAllowedResult {
    pub result: bool,
}
