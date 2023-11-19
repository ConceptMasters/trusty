use crate::rob::role::Role;
use crate::rob::tenant::PopulatedTenant;
use crate::rob::timestamps::*;
use crate::rob::utils::*;
use crate::rob::ValidateInputRules;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ulid::Ulid;
use validator::{Validate, ValidationErrors};

lazy_static! {
    static ref PROVIDER_TYPE_RE: Regex = Regex::new(r"^auth0$").unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UserExternalProvider {
    #[validate(length(min = 1, max = 30), regex = "PROVIDER_TYPE_RE")]
    pub provider_type: String,
    #[validate(length(min = 3, max = 50))]
    pub id: String,
}

impl UserExternalProvider {
    pub fn new(provider_type: String, id: String) -> Self {
        UserExternalProvider { provider_type, id }
    }
    // There should be no update on UserExternalProvider. Just create a new object.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub external_provider: UserExternalProvider,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_invited: bool,
    pub metadata: Option<Value>,
    pub associated_tenants: Vec<String>,
    pub roles: Vec<String>,
    pub timestamps: Timestamps,
}

impl User {
    pub fn new(
        email: String,
        external_provider: UserExternalProvider,
        first_name: String,
        last_name: String,
        is_active: bool,
        is_invited: bool,
        metadata: Option<Value>,
        associated_tenants: Vec<String>,
        roles: Vec<String>,
    ) -> Self {
        User {
            id: Ulid::new().to_string(),
            email,
            external_provider,
            first_name,
            last_name,
            is_active,
            is_invited,
            metadata,
            associated_tenants,
            roles,
            timestamps: Timestamps::new(),
        }
    }

    pub fn new_from_obj(new_user: &NewUser) -> Self {
        User {
            id: Ulid::new().to_string(),
            email: new_user.email.clone(),
            external_provider: new_user.external_provider.clone(),
            first_name: new_user.first_name.clone(),
            last_name: new_user.last_name.clone(),
            is_active: new_user.is_active,
            is_invited: new_user.is_active,
            metadata: new_user.metadata.clone(),
            associated_tenants: new_user.associated_tenants.clone(),
            roles: new_user.roles.clone(),
            timestamps: Timestamps::new(),
        }
    }

    pub fn apply_update(&mut self, update: &UpdateUser) {
        let mut did_update = false;
        update_field!(self, update, email, did_update);
        update_field!(self, update, external_provider, did_update);
        update_field!(self, update, first_name, did_update);
        update_field!(self, update, last_name, did_update);
        update_field!(self, update, is_active, did_update);
        update_field!(self, update, metadata, did_update);
        update_field!(self, update, associated_tenants, did_update);
        update_field!(self, update, roles, did_update);

        if did_update {
            self.timestamps.update();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserQuery {
    pub id: Option<String>,
    pub email: Option<String>,
    pub external_provider_id: Option<String>,
    pub is_active: Option<bool>,
    pub is_invited: Option<bool>,
    pub associated_tenant: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate]
    pub external_provider: UserExternalProvider,
    #[validate(length(min = 1, max = 70))]
    pub first_name: String,
    #[validate(length(min = 1, max = 70))]
    pub last_name: String,
    pub is_active: bool,
    pub is_invited: bool,
    pub metadata: Option<Value>,
    pub associated_tenants: Vec<String>,
    pub roles: Vec<String>,
}

impl ValidateInputRules for NewUser {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
    #[validate]
    pub external_provider: Option<UserExternalProvider>,
    #[validate(length(min = 1, max = 70))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 70))]
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
    pub metadata: Option<Option<Value>>,
    pub associated_tenants: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
}

impl ValidateInputRules for UpdateUser {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub namespace_id: String,
    pub email: String,
    pub external_provider: UserExternalProvider,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_invited: bool,
    pub metadata: Value,
    pub associated_tenants: Vec<String>,
    pub populated_associated_tenants: Vec<PopulatedTenant>,
    pub roles: Vec<String>,
    pub populated_roles: Vec<Role>,
    pub timestamps: Timestamps,
}
