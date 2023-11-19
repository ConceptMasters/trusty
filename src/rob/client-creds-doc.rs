use crate::{encrypted_secret::EncryptedSecret, pta_context::ProductTenantAwareContext};
use kryptic::{ClientCredentials, ClientID, ClientSecret, EncryptDecrypt, KryptoError};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullClientCredentials {
    pub client_id: ClientID,
    pub client_secret: ClientSecret,
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeClientCredentials {
    pub client_id: ClientID,
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentialsDocument {
    pub id: String,
    pub client_id: ClientID,
    pub client_secret: EncryptedSecret,
    pub owner: ProductTenantAwareContext,
    pub meta: Value,
}

impl ClientCredentialsDocument {
    pub fn from_client_credentials(
        encryptor: impl EncryptDecrypt,
        id: String,
        client_credentials: &ClientCredentials,
        owner: ProductTenantAwareContext,
        meta: Value,
    ) -> Result<Self, KryptoError> {
        let (nonce, cyphertext) = encryptor.encrypt_as_str(&client_credentials.client_secret)?;
        let client_secret = EncryptedSecret { cyphertext, nonce };
        Ok(Self {
            id,
            client_id: client_credentials.client_id.to_string(),
            client_secret,
            owner,
            meta,
        })
    }

    pub fn to_client_credentials(
        &self,
        encryptor: impl EncryptDecrypt,
    ) -> Result<ClientCredentials, KryptoError> {
        let client_secret = encryptor
            .decrypt_from_str(&self.client_secret.nonce, &self.client_secret.cyphertext)
            .map_err(|e| KryptoError::Failed(format!("Failed to decrypt client secret: {}", e)))?;
        Ok(ClientCredentials {
            client_id: self.client_id.to_string(),
            client_secret,
        })
    }

    pub fn to_full_client_credentials(
        &self,
        encryptor: impl EncryptDecrypt,
    ) -> Result<FullClientCredentials, KryptoError> {
        let client_secret = encryptor
            .decrypt_from_str(&self.client_secret.nonce, &self.client_secret.cyphertext)
            .map_err(|e| KryptoError::Failed(format!("Failed to decrypt client secret: {}", e)))?;
        Ok(FullClientCredentials {
            client_id: self.client_id.to_string(),
            client_secret,
            meta: self.meta.clone(),
        })
    }

    pub fn to_safe_client_credentials(&self) -> SafeClientCredentials {
        SafeClientCredentials {
            client_id: self.client_id.to_string(),
            meta: self.meta.clone(),
        }
    }
}
