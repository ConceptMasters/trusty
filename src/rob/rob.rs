#[path = "client-creds-doc.rs"]
pub mod client_creds_doc;
#[path = "encrypted-secret.rs"]
pub mod encrypted_secret;
#[path = "key-pair-doc.rs"]
pub mod key_pair_doc;
#[path = "meta-input.rs"]
pub mod meta_input;
pub mod namespace;
pub mod organizationprofile;
pub mod product;
#[path = "pta-context.rs"]
pub mod pta_context;
pub mod rbac;
pub mod role;
pub mod tenant;
pub mod timestamps;
pub mod token;
pub mod user;
mod utils;

#[macro_use]
extern crate lazy_static;

use validator::ValidationErrors;

pub trait ValidateInputRules {
    fn validate_input_rules(&self) -> Result<(), ValidationErrors>;
}
