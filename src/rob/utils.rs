use regex::Regex;
use validator::ValidationError;

macro_rules! update_field {
    ($self:ident, $update:ident, $field:ident, $did_update:ident) => {
        match &$update.$field {
            Some($field) => {
                $self.$field = $field.clone();
                $did_update = true;
            }
            None => {}
        }
    };
}

pub(crate) use update_field;

/// Return whether the given contains only alphanumeric characters and dashes
pub fn validate_url_safe_id(id: &String) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[0-9a-zA-Z-]+$").unwrap();
    if re.is_match(id.as_str()) {
        Ok(())
    } else {
        Err(ValidationError::new(
            "id should only contain alphanumeric characters and dashes",
        ))
    }
}
