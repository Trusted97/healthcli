use crate::storage::load_health_checks;

pub fn run(name: Option<String>) {
    let checks = load_health_checks();
    match name {
        Some(n) => {
            if let Some(check) = checks.iter().find(|c| c.name == n) {
                println!("Running check: {:?}", check);
                // Qui esegui il check
            } else {
                println!("Check '{}' non trovato!", n);
            }
        }
        None => {
            println!("Eseguendo tutti gli health check...");
            for check in &checks {
                println!("Running check: {:?}", check);
                // Qui esegui il check
            }
        }
    }
}
