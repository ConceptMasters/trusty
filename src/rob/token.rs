use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenContext {
    pub namespace: String,
    pub tenant: String,
    pub product: String,
}
