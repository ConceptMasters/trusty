pub mod namespace;
pub mod rbac;
pub mod role;
pub mod tenant;
pub mod timestamps;
pub mod user;
mod utils;

#[macro_use]
extern crate lazy_static;

use validator::ValidationErrors;

pub trait ValidateInputRules {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors>;
}
