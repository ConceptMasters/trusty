use crate::timestamps::*;
use crate::utils::*;
use crate::ValidateInputRules;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use u_turn::URN;
use u_turn_macro::URN;
use ulid::Ulid;
use validator::{Validate, ValidationErrors};

#[derive(URN, Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    #[object_id]
    pub id: String,
    #[namespace_id]
    pub namespace_id: String,
    pub name: String,
    pub description: String,
    pub metadata: Option<Value>,
    pub permissions: Vec<String>,
    #[product]
    pub product_id: String,
    #[tenant]
    pub tenant_id: String,
    pub timestamps: Timestamps,
}

impl Role {
    pub fn new(
        namespace_id: String,
        name: String,
        description: String,
        metadata: Option<Value>,
        permissions: Vec<String>,
        tenant_id: String,
        product_id: String,
    ) -> Self {
        Role {
            id: Ulid::new().to_string(),
            namespace_id,
            name,
            description,
            metadata,
            permissions,
            tenant_id,
            product_id,
            timestamps: Timestamps::new(),
        }
    }

    pub fn new_from_obj(new_role: &NewRole) -> Self {
        Role {
            id: Ulid::new().to_string(),
            namespace_id: new_role.namespace_id.clone(),
            name: new_role.name.clone(),
            description: new_role.description.clone(),
            metadata: new_role.metadata.clone(),
            permissions: new_role.permissions.clone(),
            tenant_id: new_role.tenant_id.clone(),
            product_id: new_role.product_id.clone(),
            timestamps: Timestamps::new(),
        }
    }

    pub fn apply_update(&mut self, update: &UpdateRole) {
        let mut did_update = false;
        update_field!(self, update, name, did_update);
        update_field!(self, update, description, did_update);
        update_field!(self, update, metadata, did_update);
        update_field!(self, update, permissions, did_update);
        update_field!(self, update, tenant_id, did_update);
        update_field!(self, update, product_id, did_update);
        if did_update {
            self.timestamps.update();
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewRole {
    pub namespace_id: String,
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 3, max = 500))]
    pub description: String,
    pub metadata: Option<Value>,
    pub permissions: Vec<String>,
    pub tenant_id: String,
    pub product_id: String,
}

impl ValidateInputRules for NewRole {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UpdateRole {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    #[validate(length(min = 3, max = 500))]
    pub description: Option<String>,
    pub metadata: Option<Option<Value>>,
    pub permissions: Option<Vec<String>>,
    pub tenant_id: Option<String>,
    pub product_id: Option<String>,
}

impl ValidateInputRules for UpdateRole {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}
