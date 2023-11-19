use crate::timestamps::*;
use crate::utils::*;
use crate::ValidateInputRules;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use u_turn::URN;
use u_turn_macro::URN;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)] // Used in tests
use chrono::Utc;

#[derive(URN, Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    #[object_id]
    pub id: String,
    #[namespace_id]
    pub namespace_id: String,
    pub name: String,
    pub description: String,
    pub metadata: Option<Value>,
    pub img: String,
    pub url: String,
    pub can_self_register: bool,
    pub timestamps: Timestamps,
}

impl Product {
    pub fn new(
        id: String,
        namespace_id: String,
        name: String,
        description: String,
        metadata: Option<Value>,
        img: String,
        url: String,
        can_self_register: bool,
    ) -> Self {
        Product {
            id,
            namespace_id,
            name,
            description,
            metadata,
            img,
            url,
            can_self_register,
            timestamps: Timestamps::new(),
        }
    }

    pub fn new_from_obj(new_product: &NewProduct) -> Self {
        Product {
            id: new_product.id.clone(),
            namespace_id: new_product.namespace_id.clone(),
            name: new_product.name.clone(),
            description: new_product.description.clone(),
            metadata: new_product.metadata.clone(),
            img: new_product.img.clone(),
            url: new_product.url.clone(),
            can_self_register: new_product.can_self_register,
            timestamps: Timestamps::new(),
        }
    }

    pub fn apply_update(&mut self, update: &UpdateProduct) {
        let mut did_update = false;
        update_field!(self, update, description, did_update);
        update_field!(self, update, metadata, did_update);
        update_field!(self, update, img, did_update);
        update_field!(self, update, url, did_update);
        update_field!(self, update, can_self_register, did_update);
        if did_update {
            self.timestamps.update();
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewProduct {
    #[validate(custom = "validate_url_safe_id")]
    pub id: String,
    pub namespace_id: String,
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 3, max = 500))]
    pub description: String,
    pub metadata: Option<Value>,
    #[validate(length(min = 1))]
    pub img: String,
    #[validate(length(min = 1))]
    pub url: String,
    pub can_self_register: bool,
}

impl ValidateInputRules for NewProduct {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UpdateProduct {
    #[validate(length(min = 3, max = 500))]
    pub description: Option<String>,
    pub metadata: Option<Option<Value>>,
    #[validate(length(min = 1))]
    pub img: Option<String>,
    #[validate(length(min = 1))]
    pub url: Option<String>,
    pub can_self_register: Option<bool>,
}

impl ValidateInputRules for UpdateProduct {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_to_urn() {
        let test_product: Product = Product {
            id: "1234".to_string(),
            namespace_id: "abcd".to_string(),
            name: "cool-product".to_string(),
            description: "i am a description".to_string(),
            metadata: None,
            img: "im/a/path.img".to_string(),
            url: "imawebsite.com".to_string(),
            can_self_register: false,
            timestamps: Timestamps {
                created_at: Utc::now(),
                updated_at: None,
            },
        };
        let urn_string = "urn:abcd:::product:1234".to_string();
        assert!(test_product.to_urn() == urn_string);
    }

    #[test]
    fn test_product_matches_urn_success() {
        let test_product: Product = Product {
            id: "1234".to_string(),
            namespace_id: "abcd".to_string(),
            name: "cool-product".to_string(),
            description: "i am a description".to_string(),
            metadata: None,
            img: "im/a/path.img".to_string(),
            url: "imawebsite.com".to_string(),
            can_self_register: false,
            timestamps: Timestamps {
                created_at: Utc::now(),
                updated_at: None,
            },
        };
        assert!(test_product.matches_urn("urn:abcd:::product:1234".to_string()))
    }

    #[test]
    fn test_product_matches_urn_failure_bad_prefix() {
        let test_product: Product = Product {
            id: "1234".to_string(),
            namespace_id: "abcd".to_string(),
            name: "cool-product".to_string(),
            description: "i am a description".to_string(),
            metadata: None,
            img: "im/a/path.img".to_string(),
            url: "imawebsite.com".to_string(),
            can_self_register: false,
            timestamps: Timestamps {
                created_at: Utc::now(),
                updated_at: None,
            },
        };
        assert!(!test_product.matches_urn("urnnnnnnn:abcd:::product:12345".to_string()))
    }

    #[test]
    fn test_product_matches_urn_failure_bad_id() {
        let test_product: Product = Product {
            id: "1234".to_string(),
            namespace_id: "abcd".to_string(),
            name: "cool-product".to_string(),
            description: "i am a description".to_string(),
            metadata: None,
            img: "im/a/path.img".to_string(),
            url: "imawebsite.com".to_string(),
            can_self_register: false,
            timestamps: Timestamps {
                created_at: Utc::now(),
                updated_at: None,
            },
        };
        assert!(!test_product.matches_urn("urn:abcd:::product:123456789".to_string()))
    }

    #[test]
    fn test_product_apply_update() {
        let mut test_product: Product = Product {
            id: "1234".to_string(),
            namespace_id: "abcd".to_string(),
            name: "cool-product".to_string(),
            description: "i am a description".to_string(),
            metadata: None,
            img: "im/a/path.img".to_string(),
            url: "imawebsite.com".to_string(),
            can_self_register: false,
            timestamps: Timestamps {
                created_at: Utc::now(),
                updated_at: None,
            },
        };
        let url = test_product.url.clone();
        let new_description = "i am a new description".to_string();
        let new_img = "new/img/path.jpg".to_string();
        let test_product_update: UpdateProduct = UpdateProduct {
            description: Some(new_description.clone()),
            metadata: Some(None),
            img: Some(new_img.clone()),
            url: None,
            can_self_register: None,
        };
        test_product.apply_update(&test_product_update);
        assert!(test_product.description == new_description);
        assert!(test_product.img == new_img);
        // Assert an unchanged value stayed the same
        assert!(test_product.url == url);
    }
}
