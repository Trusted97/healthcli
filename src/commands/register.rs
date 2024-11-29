use crate::models::health_check::HealthCheck;
use crate::storage::{load_health_checks, save_health_checks};

pub fn run(name: String, check_type: String) {
    let mut checks = load_health_checks();
    let new_check = HealthCheck {
        name: name.clone(),
        check_type,
        config: None,
    };
    checks.push(new_check);
    save_health_checks(&checks);
    println!("Health check '{}' registrato con successo!", name);
}
