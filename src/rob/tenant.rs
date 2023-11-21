use crate::rob::timestamps::*;
use crate::rob::utils::*;
use crate::rob::ValidateInputRules;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ulid::Ulid;
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metadata: Option<Value>,
    pub timestamps: Timestamps,
}

impl Tenant {
    pub fn new(name: String, description: String, metadata: Option<Value>) -> Self {
        Tenant {
            id: Ulid::new().to_string(),
            name,
            description,
            metadata,
            timestamps: Timestamps::new(),
        }
    }

    pub fn new_from_obj(new_tenant: &NewTenant) -> Self {
        Tenant {
            id: Ulid::new().to_string(),
            name: new_tenant.name.clone(),
            description: new_tenant.description.clone(),
            metadata: new_tenant.metadata.clone(),
            timestamps: Timestamps::new(),
        }
    }

    pub fn apply_update(&mut self, update: &UpdateTenant) {
        let mut did_update = false;
        update_field!(self, update, name, did_update);
        update_field!(self, update, description, did_update);
        update_field!(self, update, metadata, did_update);
        if did_update {
            self.timestamps.update();
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewTenant {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 3, max = 500))]
    pub description: String,
    pub metadata: Option<Value>,
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
}

impl ValidateInputRules for UpdateTenant {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PopulatedTenant {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metadata: Option<Value>,
    pub timestamps: Timestamps,
}
