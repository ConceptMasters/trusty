use crate::errors::Error;
use crate::race::race_store::{MongoStore, StoreType};
use crate::rob::rbac::{IsAllowedRequest, IsAllowedResult};

pub struct AccessControlEngine {
    #[allow(dead_code)]
    store: StoreType,
}

impl AccessControlEngine {
    pub async fn init_with_mongo_store(
        mongo_uri: String,
        mongo_db_name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mongo_store = MongoStore::init(mongo_uri, mongo_db_name).await?;
        Ok(Self {
            store: StoreType::Mongo(mongo_store),
        })
    }
    pub async fn is_allowed(
        &self,
        is_allowed_request: &IsAllowedRequest,
    ) -> Result<IsAllowedResult, Error> {
        match &self.store {
            StoreType::Mongo(store) => {
                let user_role_ids = store
                    .get_role_ids_for_user(is_allowed_request.external_user_id.clone())
                    .await?;
                let matching_roles = store
                    .get_roles_matching_request(user_role_ids, is_allowed_request)
                    .await?;
                Ok(IsAllowedResult {
                    result: !matching_roles.is_empty(),
                })
            }
        }
    }
}
