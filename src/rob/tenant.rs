use crate::product::Product;
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
pub struct Tenant {
    #[object_id]
    pub id: String,
    #[namespace_id]
    pub namespace_id: String,
    pub name: String,
    pub description: String,
    pub metadata: Option<Value>,
    pub subscribed_products: Vec<String>,
    pub timestamps: Timestamps,
}

impl Tenant {
    pub fn new(
        namespace_id: String,
        name: String,
        description: String,
        metadata: Option<Value>,
        subscribed_products: Vec<String>,
    ) -> Self {
        Tenant {
            id: Ulid::new().to_string(),
            namespace_id,
            name,
            description,
            metadata,
            subscribed_products,
            timestamps: Timestamps::new(),
        }
    }

    pub fn new_from_obj(new_tenant: &NewTenant) -> Self {
        Tenant {
            id: Ulid::new().to_string(),
            namespace_id: new_tenant.namespace_id.clone(),
            name: new_tenant.name.clone(),
            description: new_tenant.description.clone(),
            metadata: new_tenant.metadata.clone(),
            subscribed_products: new_tenant.subscribed_products.clone(),
            timestamps: Timestamps::new(),
        }
    }

    pub fn apply_update(&mut self, update: &UpdateTenant) {
        let mut did_update = false;
        update_field!(self, update, name, did_update);
        update_field!(self, update, description, did_update);
        update_field!(self, update, metadata, did_update);
        update_field!(self, update, subscribed_products, did_update);
        if did_update {
            self.timestamps.update();
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewTenant {
    pub namespace_id: String,
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 3, max = 500))]
    pub description: String,
    pub metadata: Option<Value>,
    pub subscribed_products: Vec<String>,
}

impl ValidateInputRules for NewTenant {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UpdateTenant {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    #[validate(length(min = 3, max = 500))]
    pub description: Option<String>,
    pub metadata: Option<Option<Value>>,
    pub subscribed_products: Option<Vec<String>>,
}

impl ValidateInputRules for UpdateTenant {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PopulatedTenant {
    pub id: String,
    pub namespace_id: String,
    pub name: String,
    pub description: String,
    pub metadata: Option<Value>,
    pub subscribed_products: Vec<Product>,
    pub timestamps: Timestamps,
}
