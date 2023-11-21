use crate::errors::Error;
use crate::rob::{
    role::{NewRole, UpdateRole},
    tenant::Tenant,
    user::{NewUser, UpdateUser, User},
};
use crate::store::Store;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ValidateDataIntegrity {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error>;
}

#[async_trait]
impl ValidateDataIntegrity for NewRole {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure the tenant exists if specified
        let existing_tenant: Result<Tenant, Error> = store.get_tenant(self.tenant_id.clone()).await;
        match existing_tenant {
            Ok(_) => {}
            Err(Error::NotFound) => {
                return Err(Error::ValidationError(
                    format!("Did not find tenant with id: {}", self.tenant_id.clone()).to_string(),
                ))
            }
            Err(e) => return Err(e),
        };
        Ok(())
    }
}

#[async_trait]
impl ValidateDataIntegrity for UpdateRole {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure the tenant exists
        if self.tenant_id.is_some() {
            let tenant_id = self.tenant_id.clone().unwrap();
            let existing_tenant: Result<Tenant, Error> = store.get_tenant(tenant_id.clone()).await;
            match existing_tenant {
                Ok(_) => {}
                Err(Error::NotFound) => {
                    return Err(Error::ValidationError(
                        format!("Did not find tenant with id: {}", tenant_id.clone()).to_string(),
                    ))
                }
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}

#[async_trait]
impl ValidateDataIntegrity for NewUser {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure no user exists with the same external user id
        let existing_user: Result<User, Error> =
            store.get_user(self.external_provider.id.clone()).await;
        match existing_user {
            Ok(_) => {
                return Err(Error::ValidationError(
                    format!(
                        "Found existing user with external id: {}",
                        self.external_provider.id.clone()
                    )
                    .to_string(),
                ))
            }
            Err(Error::NotFound) => {}
            Err(e) => return Err(e),
        };

        // Ensure each tenant in the associated_tenants array exists if specified
        if !self.associated_tenants.is_empty() {
            let tenants = self.associated_tenants.clone();
            for tenant in tenants.iter() {
                let existing_tenant: Result<Tenant, Error> = store.get_tenant(tenant.clone()).await;
                match existing_tenant {
                    Ok(_) => {}
                    Err(Error::NotFound) => {
                        return Err(Error::ValidationError(
                            format!("Did not find tenant with id: {}", tenant).to_string(),
                        ))
                    }
                    Err(e) => return Err(e),
                };
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ValidateDataIntegrity for UpdateUser {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        if self.external_provider.is_some() {
            // Ensure no user exists with the same external user id
            let existing_user: Result<User, Error> = store
                .get_user(self.external_provider.clone().unwrap().id)
                .await;
            match existing_user {
                Ok(_) => {
                    return Err(Error::ValidationError(
                        format!(
                            "Found existing user with external id: {}",
                            self.external_provider.clone().unwrap().id
                        )
                        .to_string(),
                    ))
                }
                Err(Error::NotFound) => {}
                Err(e) => return Err(e),
            };
        }

        // Ensure each tenant in the associated_tenants array exists if specified
        if self.associated_tenants.is_some() && !self.associated_tenants.clone().unwrap().is_empty()
        {
            let tenants = self.associated_tenants.clone().unwrap().clone();
            for tenant in tenants.iter() {
                let existing_tenant: Result<Tenant, Error> = store.get_tenant(tenant.clone()).await;
                match existing_tenant {
                    Ok(_) => {}
                    Err(Error::NotFound) => {
                        return Err(Error::ValidationError(
                            format!("Did not find tenant with id: {}", tenant).to_string(),
                        ))
                    }
                    Err(e) => return Err(e),
                };
            }
        }
        Ok(())
    }
}
