use crate::models::health_check::HealthCheck;
use serde_json;
use std::fs;
use std::path::Path;

const STORAGE_PATH: &str = "health_checks.json";

pub fn load_health_checks() -> Vec<HealthCheck> {
    if Path::new(STORAGE_PATH).exists() {
        let data = fs::read_to_string(STORAGE_PATH).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub fn save_health_checks(checks: &[HealthCheck]) {
    let data = serde_json::to_string_pretty(checks).expect("Serialization failed");
    fs::write(STORAGE_PATH, data).expect("Unable to write file");
}
