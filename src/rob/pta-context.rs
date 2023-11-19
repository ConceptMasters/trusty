use kryptic::BaseContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductTenantAwareContext {
    pub namespace: String,
    pub product_id: String,
    pub tenant_id: String,
}

impl BaseContext for ProductTenantAwareContext {}
