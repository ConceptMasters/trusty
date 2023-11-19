use crate::timestamps::*;
use crate::utils::*;
use crate::ValidateInputRules;
use serde::{Deserialize, Serialize};
use u_turn::URN;
use u_turn_macro::URN;
use ulid::Ulid;
use validator::{Validate, ValidationErrors};

#[derive(URN, Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationProfile {
    #[object_id]
    pub id: String,
    #[namespace_id]
    pub namespace_id: String,
    #[tenant]
    pub tenant_id: String,
    pub organization_name: String,
    pub organization_type: String, // TODO enum?
    pub primary_contact_number: String,
    pub registered_address_line_1: Option<String>,
    pub registered_address_line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: Option<String>,
    pub timestamps: Timestamps,
}

impl OrganizationProfile {
    pub fn new(
        namespace_id: String,
        tenant_id: String,
        organization_name: String,
        organization_type: String,
        primary_contact_number: String,
        registered_address_line_1: Option<String>,
        registered_address_line_2: Option<String>,
        city: String,
        state: String,
        zip: Option<String>,
    ) -> Self {
        OrganizationProfile {
            id: Ulid::new().to_string(),
            namespace_id,
            tenant_id,
            organization_name,
            organization_type,
            primary_contact_number,
            registered_address_line_1,
            registered_address_line_2,
            city,
            state,
            zip,
            timestamps: Timestamps::new(),
        }
    }

    pub fn new_from_obj(new_organization_profile: &NewOrganizationProfile) -> Self {
        OrganizationProfile {
            id: Ulid::new().to_string(),
            namespace_id: new_organization_profile.namespace_id.clone(),
            tenant_id: new_organization_profile.tenant_id.clone(),
            organization_name: new_organization_profile.organization_name.clone(),
            organization_type: new_organization_profile.organization_type.clone(),
            primary_contact_number: new_organization_profile.primary_contact_number.clone(),
            registered_address_line_1: new_organization_profile.registered_address_line_1.clone(),
            registered_address_line_2: new_organization_profile.registered_address_line_2.clone(),
            city: new_organization_profile.city.clone(),
            state: new_organization_profile.state.clone(),
            zip: new_organization_profile.zip.clone(),
            timestamps: Timestamps::new(),
        }
    }

    pub fn apply_update(&mut self, update: &UpdateOrganizationProfile) {
        let mut did_update = false;
        update_field!(self, update, organization_name, did_update);
        update_field!(self, update, organization_type, did_update);
        update_field!(self, update, primary_contact_number, did_update);
        update_field!(self, update, registered_address_line_1, did_update);
        update_field!(self, update, registered_address_line_2, did_update);
        update_field!(self, update, city, did_update);
        update_field!(self, update, state, did_update);
        update_field!(self, update, zip, did_update);
        if did_update {
            self.timestamps.update();
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct NewOrganizationProfile {
    pub namespace_id: String,
    pub tenant_id: String,
    #[validate(length(min = 2, max = 30))]
    pub organization_name: String,
    pub organization_type: String, // TODO add validation to restrict to list of values
    #[validate(phone, length(min = 5, max = 50))]
    pub primary_contact_number: String,
    #[validate(length(min = 5, max = 50))]
    pub registered_address_line_1: Option<String>,
    #[validate(length(max = 50))]
    pub registered_address_line_2: Option<String>,
    #[validate(length(min = 3, max = 45))]
    pub city: String,
    #[validate(length(min = 2, max = 2))]
    pub state: String,
    #[validate(length(min = 5, max = 5))]
    pub zip: Option<String>,
}

impl ValidateInputRules for NewOrganizationProfile {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UpdateOrganizationProfile {
    #[validate(length(min = 2, max = 30))]
    pub organization_name: Option<String>,
    pub organization_type: Option<String>, // TODO add validation to restrict to list of values
    #[validate(phone, length(min = 5, max = 50))]
    pub primary_contact_number: Option<String>,
    #[validate(length(min = 5, max = 50))]
    pub registered_address_line_1: Option<Option<String>>,
    #[validate(length(max = 50))]
    pub registered_address_line_2: Option<Option<String>>,
    pub city: Option<String>,
    #[validate(length(min = 2, max = 2))]
    pub state: Option<String>,
    #[validate(length(min = 5, max = 5))]
    pub zip: Option<Option<String>>,
}

impl ValidateInputRules for UpdateOrganizationProfile {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors> {
        self.validate()
    }
}
