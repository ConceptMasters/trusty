use crate::errors::Error;
use crate::race::race_store::Store;
use crate::rob::{rbac::IsAllowedRequest, role::Role, user::User};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use log::{debug, error};
use mongodb::bson::doc;
use mongodb::{Client, Collection, Database};

#[derive(Debug, Clone)]
pub struct MongoStore {
    // We might not need client and db, but we *probably* will later to do more fancy stuff
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    db: Database,
    role_col: Collection<Role>,
    user_col: Collection<User>,
}

impl MongoStore {
    pub async fn init(
        mongo_uri: String,
        mongo_db_name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (client, db) = Self::connect_db(mongo_uri, mongo_db_name).await?;
        let (role_col, user_col) = Self::connect(&db).await;
        Ok(Self {
            client,
            db,
            role_col,
            user_col,
        })
    }

    async fn connect_db(
        mongo_uri: String,
        mongo_db_name: String,
    ) -> Result<(Client, Database), Box<dyn std::error::Error>> {
        let client_result = Client::with_uri_str(mongo_uri).await;
        match &client_result {
            Ok(client) => {
                let db = client.database(mongo_db_name.as_str());
                Ok((client.clone(), db))
            }
            Err(e) => Err(e.clone().into()),
        }
    }

    async fn connect(db: &Database) -> (Collection<Role>, Collection<User>) {
        let role_col: Collection<Role> = db.collection("roles");
        let user_col: Collection<User> = db.collection("users");
        (role_col, user_col)
    }
}

#[async_trait]
impl Store for MongoStore {
    async fn get_role_ids_for_user(&self, user_id: String) -> Result<Vec<String>, Error> {
        let filter = doc! {
            "external_provider.id": user_id,
        };
        let user = self
            .user_col
            .find_one(filter, None)
            .await
            .map_err(|e| {
                error!("Failed create cursor to get user: {:?}", e);
                Error::DatabaseOperationFailed(format!("Failed create cursor to get user: {:?}", e))
            })?
            .unwrap(); // We know this user exists, so unwrap the option
        let roles = user.roles;
        Ok(roles)
    }

    async fn get_roles_matching_request(
        &self,
        role_ids: Vec<String>,
        access_control_request: &IsAllowedRequest,
    ) -> Result<Vec<Role>, Error> {
        debug!(
            "Getting roles matching IsAllowedRequest among role ids: {:?}",
            role_ids.clone()
        );
        let filter = doc! {
            "permissions": format!("{}:{}", access_control_request.resource, access_control_request.action),
            "tenant_id": access_control_request.tenant.clone(),
            /*
            "$or": [
                {
                    "tenant_id": access_control_request.tenant.clone(),
                },
                {
                    "tenant_id": Bson::Null,
                },
            ],
            */
            "id": {
                "$in": role_ids,
            }
        };

        let cursor = self.role_col.find(filter, None).await.map_err(|e| {
            error!("Failed create cursor to get roles: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed create cursor to get roles: {:?}", e))
        })?;
        let roles: Vec<Role> = cursor.try_collect().await.map_err(|e| {
            error!("Failed to get roles: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get roles: {:?}", e))
        })?;
        Ok(roles)
    }
}
