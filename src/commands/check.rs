use crate::storage::load_health_checks;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;

pub fn run(name: Option<String>) {
    let checks = load_health_checks();
    let client = Client::new();

    match name {
        Some(n) => {
            if let Some(check) = checks.iter().find(|c| c.name == n) {
                println!("Running check: {:?}", check);
                run_check(check, &client);
            } else {
                println!("Check '{}' not found!", n);
            }
        }
        None => {
            println!("Running all health checks...");
            for check in &checks {
                println!("Running check: {:?}", check);
                run_check(check, &client);
            }
        }
    }
}

fn run_check(check: &crate::models::health_check::HealthCheck, client: &Client) {
    match check.check_type.as_str() {
        "url" => {
            if let Some(config) = &check.config {
                println!("Pinging URL: {}", config);
                let result = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(client.get(config).send());
                match result {
                    Ok(response) => {
                        println!("URL responded with status: {}", response.status());
                    }
                    Err(err) => {
                        println!("Failed to ping URL: {}", err);
                    }
                }
            } else {
                println!("Missing configuration for URL check.");
            }
        }
        "database" => {
            if let Some(config) = &check.config {
                println!("Connecting to database: {}", config);
                let pool = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(PgPoolOptions::new().connect(config));
                match pool {
                    Ok(_) => println!("Database connection successful!"),
                    Err(err) => println!("Failed to connect to database: {}", err),
                }
            } else {
                println!("Missing configuration for database check.");
            }
        }
        "disk" => {
            match sys_info::disk_info() {
                Ok(disk) => {
                    println!("Disk total: {} MB", disk.total / 1024);
                    println!("Disk free: {} MB", disk.free / 1024);
                }
                Err(err) => {
                    println!("Failed to get disk info: {}", err);
                }
            }
        }
        _ => println!("Unsupported check type: {}", check.check_type),
    }
}
