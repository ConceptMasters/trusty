use validator::ValidationErrors;

pub trait ValidateInputRules {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors>;
}
