use crate::store::Store;
use async_trait::async_trait;
use platform_errors::Error;
use rob::{
    namespace::{Namespace, NewNamespace},
    organizationprofile::NewOrganizationProfile,
    product::{NewProduct, Product},
    role::{NewRole, UpdateRole},
    tenant::{NewTenant, Tenant, UpdateTenant},
    user::{NewUser, UpdateUser, User},
};
use std::sync::Arc;

#[async_trait]
pub trait ValidateDataIntegrity {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error>;
}

#[async_trait]
pub trait ValidateDataIntegrityWithNamespace {
    async fn validate_data_integrity(
        &self,
        store: Arc<dyn Store>,
        namespace_id: String,
    ) -> Result<(), Error>;
}

#[async_trait]
impl ValidateDataIntegrity for NewNamespace {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        let existing_namespace: Result<Namespace, Error> =
            store.get_namespace(self.id.clone()).await;
        // Check that this is not a duplicate id
        match existing_namespace {
            Ok(_) => {
                return Err(Error::ValidationError(
                    format!("Namespace with id already exists: {}", self.id.clone()).to_string(),
                ))
            }
            Err(Error::NotFound) => {}
            Err(e) => return Err(e),
        };
        Ok(())
    }
}

#[async_trait]
impl ValidateDataIntegrity for NewOrganizationProfile {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure the namespace exists
        let existing_namespace: Result<Namespace, Error> =
            store.get_namespace(self.namespace_id.clone()).await;
        match existing_namespace {
            Ok(_) => {}
            Err(Error::NotFound) => {
                return Err(Error::ValidationError(
                    format!(
                        "Did not find namespace with id: {}",
                        self.namespace_id.clone()
                    )
                    .to_string(),
                ))
            }
            Err(e) => return Err(e),
        };

        // Ensure the tenant exists
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
impl ValidateDataIntegrity for NewProduct {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure the namespace exists
        let existing_namespace: Result<Namespace, Error> =
            store.get_namespace(self.namespace_id.clone()).await;
        match existing_namespace {
            Ok(_) => {}
            Err(Error::NotFound) => {
                return Err(Error::ValidationError(
                    format!(
                        "Did not find namespace with id: {}",
                        self.namespace_id.clone()
                    )
                    .to_string(),
                ))
            }
            Err(e) => return Err(e),
        };
        // Ensure the product does not already exist
        let existing_product: Result<Product, Error> = store
            .get_product(self.namespace_id.clone(), self.id.clone())
            .await;
        match existing_product {
            Ok(_) => {
                return Err(Error::ValidationError(
                    format!("Product with id already exists: {}", self.id.clone()).to_string(),
                ))
            }
            Err(Error::NotFound) => {}
            Err(e) => return Err(e),
        };
        Ok(())
    }
}

#[async_trait]
impl ValidateDataIntegrity for NewRole {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure the namespace exists
        let existing_namespace: Result<Namespace, Error> =
            store.get_namespace(self.namespace_id.clone()).await;
        match existing_namespace {
            Ok(_) => {}
            Err(Error::NotFound) => {
                return Err(Error::ValidationError(
                    format!(
                        "Did not find namespace with id: {}",
                        self.namespace_id.clone()
                    )
                    .to_string(),
                ))
            }
            Err(e) => return Err(e),
        };

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

        // Ensure the product exists if specified
        let existing_product: Result<Product, Error> = store
            .get_product(self.namespace_id.clone(), self.product_id.clone())
            .await;
        match existing_product {
            Ok(_) => {}
            Err(Error::NotFound) => {
                return Err(Error::ValidationError(
                    format!("Did not find product with id: {}", self.product_id.clone())
                        .to_string(),
                ))
            }
            Err(e) => return Err(e),
        };
        Ok(())
    }
}

#[async_trait]
impl ValidateDataIntegrityWithNamespace for UpdateRole {
    async fn validate_data_integrity(
        &self,
        store: Arc<dyn Store>,
        namespace_id: String,
    ) -> Result<(), Error> {
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

        // Ensure the product exists
        if self.product_id.is_some() {
            let product_id = self.product_id.clone().unwrap();
            let existing_product: Result<Product, Error> =
                store.get_product(namespace_id, product_id.clone()).await;
            match existing_product {
                Ok(_) => {}
                Err(Error::NotFound) => {
                    return Err(Error::ValidationError(
                        format!("Did not find product with id: {}", product_id.clone()).to_string(),
                    ))
                }
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}

#[async_trait]
impl ValidateDataIntegrity for NewTenant {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure the namespace exists
        let existing_namespace: Result<Namespace, Error> =
            store.get_namespace(self.namespace_id.clone()).await;
        match existing_namespace {
            Ok(_) => {}
            Err(Error::NotFound) => {
                return Err(Error::ValidationError(
                    format!(
                        "Did not find namespace with id: {}",
                        self.namespace_id.clone()
                    )
                    .to_string(),
                ))
            }
            Err(e) => return Err(e),
        };

        // Ensure each product in the subscribed_products array exists if specified
        if !self.subscribed_products.is_empty() {
            let products = self.subscribed_products.clone();
            for product in products.iter() {
                let existing_product: Result<Product, Error> = store
                    .get_product(self.namespace_id.clone(), product.clone())
                    .await;
                match existing_product {
                    Ok(_) => {}
                    Err(Error::NotFound) => {
                        return Err(Error::ValidationError(
                            format!("Did not find product with id: {}", product).to_string(),
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
impl ValidateDataIntegrityWithNamespace for UpdateTenant {
    async fn validate_data_integrity(
        &self,
        store: Arc<dyn Store>,
        namespace_id: String,
    ) -> Result<(), Error> {
        // Ensure each product in the subscribed_products array exists if specified
        if self.subscribed_products.is_some()
            && !self.subscribed_products.clone().unwrap().is_empty()
        {
            let products = self.subscribed_products.clone().unwrap().clone();
            for product in products.iter() {
                let existing_product: Result<Product, Error> = store
                    .get_product(namespace_id.clone(), product.clone())
                    .await;
                match existing_product {
                    Ok(_) => {}
                    Err(Error::NotFound) => {
                        return Err(Error::ValidationError(
                            format!("Did not find product with id: {}", product).to_string(),
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
impl ValidateDataIntegrity for NewUser {
    async fn validate_data_integrity(&self, store: Arc<dyn Store>) -> Result<(), Error> {
        // Ensure the namespace exists
        let existing_namespace: Result<Namespace, Error> =
            store.get_namespace(self.namespace_id.clone()).await;
        match existing_namespace {
            Ok(_) => {}
            Err(Error::NotFound) => {
                return Err(Error::ValidationError(
                    format!(
                        "Did not find namespace with id: {}",
                        self.namespace_id.clone()
                    )
                    .to_string(),
                ))
            }
            Err(e) => return Err(e),
        };

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
