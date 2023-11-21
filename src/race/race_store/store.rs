use crate::errors::Error;
use crate::race::race_store::MongoStore;
use crate::rob::{rbac::IsAllowedRequest, role::Role};
use async_trait::async_trait;

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
    ) -> Result<Vec<Role>, Error>;
}
