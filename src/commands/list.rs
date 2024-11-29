use crate::storage::load_health_checks;

pub fn run() {
    let checks = load_health_checks();
    if checks.is_empty() {
        println!("Nessun health check registrato.");
    } else {
        println!("Health checks registrati:");
        for check in checks {
            println!("- {}", check.name);
        }
    }
}
