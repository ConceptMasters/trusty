use crate::store::Store;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use log::{error, info};
use mongodb::{bson::doc, Client, Collection, Database};
use platform_errors::Error;
use rob::{
    namespace::Namespace,
    organizationprofile::OrganizationProfile,
    product::Product,
    role::Role,
    tenant::Tenant,
    user::{User, UserInfo, UserQuery},
};

async fn mongo_result<T>(
    result: Result<Option<T>, mongodb::error::Error>,
    operation: &str,
) -> Result<T, Error> {
    match result {
        Ok(None) => Err(Error::NotFound),
        Ok(Some(item)) => Ok(item),
        Err(e) => {
            error!("Failed to {}: {:?}", operation, e);
            Err(Error::DatabaseOperationFailed(format!(
                "Failed to {}: {:?}",
                operation, e
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MongoStore {
    // We might not need client and db, but we *probably* will later to do more fancy stuff
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    db: Database,

    namespace_col: Collection<Namespace>,
    organizationprofile_col: Collection<OrganizationProfile>,
    product_col: Collection<Product>,
    role_col: Collection<Role>,
    tenant_col: Collection<Tenant>,
    user_col: Collection<User>,
}

impl MongoStore {
    pub async fn init(
        mongo_uri: String,
        mongo_db_name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (client, db) = Self::connect_db(mongo_uri, mongo_db_name).await?;
        let (namespace_col, organizationprofile_col, product_col, role_col, tenant_col, user_col) =
            Self::connect(&db).await;
        Ok(Self {
            client,
            db,
            namespace_col,
            organizationprofile_col,
            product_col,
            role_col,
            tenant_col,
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
                return Ok((client.clone(), db));
            }
            Err(e) => return Err(e.clone().into()),
        }
    }

    async fn connect(
        db: &Database,
    ) -> (
        Collection<Namespace>,
        Collection<OrganizationProfile>,
        Collection<Product>,
        Collection<Role>,
        Collection<Tenant>,
        Collection<User>,
    ) {
        let namespace_col: Collection<Namespace> = db.collection("namespaces");
        let organizationprofile_col: Collection<OrganizationProfile> =
            db.collection("organiztionprofiles");
        let product_col: Collection<Product> = db.collection("products");
        let role_col: Collection<Role> = db.collection("roles");
        let tenant_col: Collection<Tenant> = db.collection("tenants");
        let user_col: Collection<User> = db.collection("users");
        (
            namespace_col,
            organizationprofile_col,
            product_col,
            role_col,
            tenant_col,
            user_col,
        )
    }
}

#[async_trait]
impl Store for MongoStore {
    ////////////////
    // Namespaces //
    ////////////////
    async fn add_namespace(&self, namespace: &Namespace) -> Result<(), Error> {
        self.namespace_col
            .insert_one(namespace.clone(), None)
            .await
            .map_err(|e| {
                error!("Failed to insert namespace: {:?}", e);
                Error::DatabaseOperationFailed(format!("Failed to insert namespace: {:?}", e))
            })?;
        info!("Store added namespace: {:?}", namespace);
        Ok(())
    }

    async fn delete_namespace(&self, id: String) -> Result<Namespace, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self.namespace_col.find_one_and_delete(filter, None).await;
        mongo_result(result, "delete namespace").await
    }

    async fn get_namespace(&self, id: String) -> Result<Namespace, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self.namespace_col.find_one(filter, None).await;
        mongo_result(result, "get namespace").await
    }

    async fn get_namespaces(&self) -> Result<Vec<Namespace>, Error> {
        let filter = doc! {};
        let cursor = self.namespace_col.find(filter, None).await.map_err(|e| {
            error!("Failed to create cursor to get namespaces: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get namespaces: {:?}", e))
        })?;
        let namespaces: Vec<Namespace> = cursor.try_collect().await.map_err(|e| {
            error!("Failed to get namespaces: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get namespaces: {:?}", e))
        })?;
        Ok(namespaces)
    }

    //////////////
    // Products //
    //////////////
    async fn add_product(&self, product: &Product) -> Result<(), Error> {
        self.product_col
            .insert_one(product.clone(), None)
            .await
            .map_err(|e| {
                error!("Failed to insert product: {:?}", e);
                Error::DatabaseOperationFailed(format!("Failed to insert product: {:?}", e))
            })?;
        info!("Store added product: {:?}", product);
        Ok(())
    }

    async fn update_product(
        &self,
        namespace_id: String,
        id: String,
        updated_product: &Product,
    ) -> Result<Product, Error> {
        let filter = doc! {
            "namespace_id": namespace_id,
            "id": id,
        };
        let result = self
            .product_col
            .find_one_and_replace(filter, updated_product, None)
            .await;
        mongo_result(result, "update product").await
    }

    async fn delete_product(&self, namespace_id: String, id: String) -> Result<Product, Error> {
        let filter = doc! {
            "namespace_id": namespace_id,
            "id": id,
        };
        let result = self.product_col.find_one_and_delete(filter, None).await;
        mongo_result(result, "delete product").await
    }

    async fn get_product(&self, namespace_id: String, id: String) -> Result<Product, Error> {
        let filter = doc! {
            "namespace_id": namespace_id,
            "id": id,
        };
        let result = self.product_col.find_one(filter, None).await;
        mongo_result(result, "get product").await
    }

    async fn get_products(&self, namespace_id: String) -> Result<Vec<Product>, Error> {
        let filter = doc! {"namespace_id": namespace_id};
        let cursor = self.product_col.find(filter, None).await.map_err(|e| {
            error!("Failed to create cursor to get products: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get products: {:?}", e))
        })?;
        let products: Vec<Product> = cursor.try_collect().await.map_err(|e| {
            error!("Failed to get products: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get products: {:?}", e))
        })?;
        Ok(products)
    }

    /////////////
    // Tenants //
    /////////////
    async fn add_tenant(&self, tenant: &Tenant) -> Result<(), Error> {
        self.tenant_col
            .insert_one(tenant.clone(), None)
            .await
            .map_err(|e| {
                error!("Failed to insert tenant: {:?}", e);
                Error::DatabaseOperationFailed(format!("Failed to insert tenant: {:?}", e))
            })?;
        info!("Store added tenant: {:?}", tenant);
        Ok(())
    }

    async fn update_tenant(&self, id: String, updated_tenant: &Tenant) -> Result<Tenant, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self
            .tenant_col
            .find_one_and_replace(filter, updated_tenant, None)
            .await;
        mongo_result(result, "update tenant").await
    }

    async fn subscribe_tenant_to_product(
        &self,
        tenant_id: String,
        product_id: String,
    ) -> Result<Tenant, Error> {
        let filter = doc! {
            "id": tenant_id,
        };
        let update = doc! {
            "$push": {
                "subscribed_products": product_id,
            }
        };
        let result = self
            .tenant_col
            .find_one_and_update(filter, update, None)
            .await;
        mongo_result(result, "subscribe tenant to product").await
    }

    async fn delete_tenant(&self, id: String) -> Result<Tenant, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self.tenant_col.find_one_and_delete(filter, None).await;
        mongo_result(result, "delete tenant").await
    }

    async fn get_tenant(&self, id: String) -> Result<Tenant, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self.tenant_col.find_one(filter, None).await;
        mongo_result(result, "get tenant").await
    }

    async fn get_tenants(&self, namespace_id: String) -> Result<Vec<Tenant>, Error> {
        let filter = doc! {"namespace_id": namespace_id};
        let cursor = self.tenant_col.find(filter, None).await.map_err(|e| {
            error!("Failed to create cursor to get tenants: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get tenants: {:?}", e))
        })?;
        let tenants: Vec<Tenant> = cursor.try_collect().await.map_err(|e| {
            error!("Failed to get tenants: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get tenants: {:?}", e))
        })?;
        Ok(tenants)
    }

    ///////////////////////////
    // Organization Profiles //
    ///////////////////////////
    async fn add_organization_profile(
        &self,
        organization_profile: &OrganizationProfile,
    ) -> Result<(), Error> {
        self.organizationprofile_col
            .insert_one(organization_profile.clone(), None)
            .await
            .map_err(|e| {
                error!("Failed to insert organization profile: {:?}", e);
                Error::DatabaseOperationFailed(format!(
                    "Failed to insert organization profile: {:?}",
                    e
                ))
            })?;
        info!(
            "Store added organization profile: {:?}",
            organization_profile
        );
        Ok(())
    }

    async fn update_organization_profile(
        &self,
        id: String,
        updated_organization_profile: &OrganizationProfile,
    ) -> Result<OrganizationProfile, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self
            .organizationprofile_col
            .find_one_and_replace(filter, updated_organization_profile, None)
            .await;
        mongo_result(result, "update organization profile").await
    }

    async fn delete_organization_profile(&self, id: String) -> Result<OrganizationProfile, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self
            .organizationprofile_col
            .find_one_and_delete(filter, None)
            .await;
        mongo_result(result, "delete organization profile").await
    }

    async fn get_organization_profile(&self, id: String) -> Result<OrganizationProfile, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self.organizationprofile_col.find_one(filter, None).await;
        mongo_result(result, "get organization profile").await
    }

    async fn get_organization_profiles(
        &self,
        namespace_id: String,
    ) -> Result<Vec<OrganizationProfile>, Error> {
        let filter = doc! {"namespace_id": namespace_id};
        let cursor = self
            .organizationprofile_col
            .find(filter, None)
            .await
            .map_err(|e| {
                error!(
                    "Failed to create cursor to get organization profiles: {:?}",
                    e
                );
                Error::DatabaseOperationFailed(format!(
                    "Failed to get organization profiles: {:?}",
                    e
                ))
            })?;
        let organization_profiles: Vec<OrganizationProfile> =
            cursor.try_collect().await.map_err(|e| {
                error!("Failed to get organization profiles: {:?}", e);
                Error::DatabaseOperationFailed(format!(
                    "Failed to get organization profiles: {:?}",
                    e
                ))
            })?;
        Ok(organization_profiles)
    }

    ///////////
    // Users //
    ///////////
    async fn add_user(&self, user: &User) -> Result<(), Error> {
        self.user_col
            .insert_one(user.clone(), None)
            .await
            .map_err(|e| {
                error!("Failed to insert user: {:?}", e);
                Error::DatabaseOperationFailed(format!("Failed to insert user: {:?}", e))
            })?;
        info!("Store added user: {:?}", user);
        Ok(())
    }

    async fn update_user(&self, external_id: String, updated_user: &User) -> Result<User, Error> {
        let filter = doc! {
            "external_provider.id": external_id,
        };
        let result = self
            .user_col
            .find_one_and_replace(filter, updated_user, None)
            .await;
        mongo_result(result, "update user").await
    }

    async fn associate_user_with_tenant(
        &self,
        external_user_id: String,
        tenant_id: String,
    ) -> Result<User, Error> {
        let filter = doc! {
            "external_provider.id": external_user_id,
        };
        let update = doc! {
            "$push": {
                "associated_tenants": tenant_id,
            }
        };
        let result = self
            .user_col
            .find_one_and_update(filter, update, None)
            .await;
        mongo_result(result, "associate user with tenant").await
    }

    async fn delete_user(&self, external_id: String) -> Result<User, Error> {
        let filter = doc! {
            "external_provider.id": external_id,
        };
        let result = self.user_col.find_one_and_delete(filter, None).await;
        mongo_result(result, "delete user").await
    }

    async fn get_user(&self, external_id: String) -> Result<User, Error> {
        let filter = doc! {
            "external_provider.id": external_id,
        };
        let result = self.user_col.find_one(filter, None).await;
        mongo_result(result, "get user").await
    }

    async fn get_users(&self, namespace_id: String, query: UserQuery) -> Result<Vec<User>, Error> {
        let mut filter = doc! {
            "namespace_id": namespace_id,
        };
        if query.id.is_some() {
            filter.insert("id", query.id);
        }
        if query.email.is_some() {
            filter.insert("email", query.email);
        }
        if query.external_provider_id.is_some() {
            filter.insert("external_provider.id", query.external_provider_id);
        }
        if query.is_active.is_some() {
            filter.insert("is_active", query.is_active);
        }
        if query.is_invited.is_some() {
            filter.insert("is_invited", query.is_invited);
        }
        if query.associated_tenant.is_some() {
            filter.insert(
                "associated_tenants",
                doc! {"$in": [query.associated_tenant]},
            );
        }
        let cursor = self.user_col.find(filter, None).await.map_err(|e| {
            error!("Failed to create cursor to get users: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get users: {:?}", e))
        })?;
        let users: Vec<User> = cursor.try_collect().await.map_err(|e| {
            error!("Failed to get users: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get users: {:?}", e))
        })?;
        Ok(users)
    }

    async fn get_user_info(&self, external_id: String) -> Result<UserInfo, Error> {
        // Let's enforce auth0 right now
        let stage_filter = doc! {
            "external_provider.provider_type": "auth0",
            "external_provider.id": external_id.clone(),
        };
        let stage_limit_1 = doc! {"$limit": 1};
        let stage_lookup_tenants = doc! {
            "$lookup": {
                "from": "tenants",
                "localField": "associated_tenants",
                "foreignField": "id",
                "as": "populated_tenants",
            }
        };
        let stage_lookup_roles = doc! {
            "$lookup": {
                "from": "roles",
                "localField": "roles",
                "foreignField": "id",
                "as": "populated_roles",
            }
        };
        let stage_lookup_tenant_products = doc! {
            "$lookup": {
                "from": "products",
                "localField": "populated_tenants.subscribed_products",
                "foreignField": "id",
                "as": "populated_tenants.populated_subscribed_products",
            }
        };
        let pipeline = vec![
            stage_filter,
            stage_limit_1,
            stage_lookup_tenants,
            stage_lookup_roles,
            stage_lookup_tenant_products,
        ];
        let mut user_info_cursor = self.user_col.aggregate(pipeline, None).await.map_err(|e| {
            Error::DatabaseOperationFailed(format!(
                "Failed to get user with external id: '{}' Error: {:?}",
                external_id.clone(),
                e
            ))
        })?;
        user_info_cursor
            .advance()
            .await
            .map_err(|_| Error::NotFound)?;
        let user_info: UserInfo =
            user_info_cursor
                .with_type()
                .deserialize_current()
                .map_err(|e| {
                    Error::DatabaseOperationFailed(format!(
                        "Failed to deserizalize user info object: {:?}",
                        e
                    ))
                })?;
        Ok(user_info)
    }

    ///////////
    // Roles //
    ///////////
    async fn add_role(&self, role: &Role) -> Result<(), Error> {
        self.role_col
            .insert_one(role.clone(), None)
            .await
            .map_err(|e| {
                error!("Failed to insert role: {:?}", e);
                Error::DatabaseOperationFailed(format!("Failed to insert role: {:?}", e))
            })?;
        info!("Store added role: {:?}", role);
        Ok(())
    }

    async fn update_role(&self, id: String, updated_role: &Role) -> Result<Role, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self
            .role_col
            .find_one_and_replace(filter, updated_role, None)
            .await;
        mongo_result(result, "update role").await
    }

    async fn delete_role(&self, id: String) -> Result<Role, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self.role_col.find_one_and_delete(filter, None).await;
        mongo_result(result, "delete role").await
    }

    async fn get_role(&self, id: String) -> Result<Role, Error> {
        let filter = doc! {
            "id": id,
        };
        let result = self.role_col.find_one(filter, None).await;
        mongo_result(result, "get role").await
    }

    async fn get_roles(&self, namespace_id: String) -> Result<Vec<Role>, Error> {
        let filter = doc! {"namespace_id": namespace_id};
        let cursor = self.role_col.find(filter, None).await.map_err(|e| {
            error!("Failed to create cursor to get roles: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get roles: {:?}", e))
        })?;
        let roles: Vec<Role> = cursor.try_collect().await.map_err(|e| {
            error!("Failed to get roles: {:?}", e);
            Error::DatabaseOperationFailed(format!("Failed to get roles: {:?}", e))
        })?;
        Ok(roles)
    }
}
