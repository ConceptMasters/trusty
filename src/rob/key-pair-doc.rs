use crate::{encrypted_secret::EncryptedSecret, pta_context::ProductTenantAwareContext};
use kryptic::{PrivateKey, PublicKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullKeyPair {
    pub id: String,
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeKeyPair {
    pub id: String,
    pub public_key: PublicKey,
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyPairDocument {
    pub id: String,
    pub public_key: PublicKey,
    pub private_key: EncryptedSecret,
    pub owner: ProductTenantAwareContext,
    pub meta: Value,
}

impl KeyPairDocument {
    pub fn from_key_pair(
        encryptor: impl kryptic::EncryptDecrypt,
        id: String,
        key_pair: &kryptic::KeyPair,
        owner: ProductTenantAwareContext,
        meta: Value,
    ) -> Result<Self, kryptic::KryptoError> {
        let (nonce, cyphertext) = encryptor.encrypt_as_str(&key_pair.private_key)?;
        let private_key = EncryptedSecret { cyphertext, nonce };
        Ok(Self {
            id,
            public_key: key_pair.public_key.clone(),
            private_key,
            owner,
            meta,
        })
    }

    pub fn to_key_pair(
        &self,
        encryptor: impl kryptic::EncryptDecrypt,
    ) -> Result<kryptic::KeyPair, kryptic::KryptoError> {
        let private_key = encryptor
            .decrypt_from_str(&self.private_key.nonce, &self.private_key.cyphertext)
            .map_err(|e| {
                kryptic::KryptoError::Failed(format!("Failed to decrypt private key: {}", e))
            })?;
        Ok(kryptic::KeyPair {
            public_key: self.public_key.clone(),
            private_key,
        })
    }

    pub fn to_full_key_pair(
        &self,
        encryptor: impl kryptic::EncryptDecrypt,
    ) -> Result<FullKeyPair, kryptic::KryptoError> {
        let key_pair = self.to_key_pair(encryptor)?;
        Ok(FullKeyPair {
            id: self.public_key.to_string(),
            public_key: key_pair.public_key,
            private_key: key_pair.private_key,
            meta: self.meta.clone(),
        })
    }

    pub fn to_safe_key_pair(&self) -> SafeKeyPair {
        SafeKeyPair {
            id: self.public_key.to_string(),
            public_key: self.public_key.clone(),
            meta: self.meta.clone(),
        }
    }
}
