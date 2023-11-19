use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptedSecret {
    pub cyphertext: String,
    pub nonce: String,
}
