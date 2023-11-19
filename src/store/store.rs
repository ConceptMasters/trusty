use async_trait::async_trait;
use crate::errors::Error;
use crate::rob::{
    role::Role,
    tenant::Tenant,
    user::{User, UserInfo, UserQuery},
};

#[async_trait]
pub trait Store: Send + Sync {
    // Tenants
    async fn add_tenant(&self, tenant: &Tenant) -> Result<(), Error>;
    async fn update_tenant(&self, id: String, update_tenant: &Tenant) -> Result<Tenant, Error>;
    async fn subscribe_tenant_to_product(
        &self,
        tenant_id: String,
        product_id: String,
    ) -> Result<Tenant, Error>;
    async fn delete_tenant(&self, id: String) -> Result<Tenant, Error>;
    async fn get_tenant(&self, id: String) -> Result<Tenant, Error>;
    async fn get_tenants(&self, namespace_id: String) -> Result<Vec<Tenant>, Error>;

    // Users
    async fn add_user(&self, user: &User) -> Result<(), Error>;
    async fn update_user(&self, id: String, update_user: &User) -> Result<User, Error>;
    async fn associate_user_with_tenant(
        &self,
        external_user_id: String,
        tenant_id: String,
    ) -> Result<User, Error>;
    async fn delete_user(&self, id: String) -> Result<User, Error>;
    async fn get_user(&self, id: String) -> Result<User, Error>;
    async fn get_users(&self, namespace_id: String, query: UserQuery) -> Result<Vec<User>, Error>;
    async fn get_user_info(&self, external_id: String) -> Result<UserInfo, Error>;

    // Roles
    async fn add_role(&self, role: &Role) -> Result<(), Error>;
    async fn update_role(&self, id: String, update_role: &Role) -> Result<Role, Error>;
    async fn delete_role(&self, id: String) -> Result<Role, Error>;
    async fn get_role(&self, id: String) -> Result<Role, Error>;
    async fn get_roles(&self, namespace_id: String) -> Result<Vec<Role>, Error>;
}
