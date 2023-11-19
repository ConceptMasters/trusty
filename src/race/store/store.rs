use crate::store::MongoStore;
use async_trait::async_trait;
use platform_errors::Error;
use rob::{rbac::IsAllowedRequest, role::Role};

pub enum StoreType {
    Mongo(MongoStore),
}

#[async_trait]
pub trait Store: Send + Sync {
    async fn get_role_ids_for_user(&self, user_id: String) -> Result<Vec<String>, Error>;
    async fn get_roles_matching_request(
        &self,
        role_ids: Vec<String>,
        access_control_request: &IsAllowedRequest,
        namespace: String,
    ) -> Result<Vec<Role>, Error>;
}
