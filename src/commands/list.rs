use crate::storage::load_health_checks;

pub fn run() {
    let checks = load_health_checks();
    if checks.is_empty() {
        println!("No health check registered.");
    } else {
        println!("Health checks registered successfully:");
        for check in checks {
            println!("- {}", check.name);
        }
    }
}
