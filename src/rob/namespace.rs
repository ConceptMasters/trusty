use crate::timestamps::*;
use crate::utils::validate_url_safe_id;
use crate::ValidateInputRules;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Namespace {
    pub id: String,
    pub timestamps: Timestamps,
}

/// No setters for namespace. We can reference by name, so it could break the
/// platform to allow changing the name of a namespace.
impl Namespace {
    pub fn new(id: String) -> Self {
        Namespace {
            id,
            timestamps: Timestamps::new(),
        }
    }

    pub fn new_from_obj(new_namespace: &NewNamespace) -> Self {
        Namespace {
            id: new_namespace.id.clone(),
            timestamps: Timestamps::new(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewNamespace {
    #[validate(length(min = 2, max = 30), custom = "validate_url_safe_id")]
    pub id: String,
}

impl ValidateInputRules for NewNamespace {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}
