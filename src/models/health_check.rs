use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheck {
    pub name: String,
    pub check_type: String,
    pub config: Option<String>,
}
