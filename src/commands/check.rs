use crate::storage::{load_health_checks, save_health_checks};
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;

pub async fn run(name: Option<String>, remove: bool) {
    let mut checks = load_health_checks(); // Load existing checks
    let client = Client::new();

    if remove {
        if let Some(n) = name {
            // Find and remove the check by name
            if let Some(index) = checks.iter().position(|c| c.name == n) {
                checks.remove(index);
                save_health_checks(&checks); // Save the updated list
                println!("Check {n} has been removed.");
            } else {
                println!("Check {} not found!", n);
            }
        } else {
            println!("Please provide a check name to remove.");
        }
    } else {
        match name {
            Some(n) => {
                if let Some(check) = checks.iter().find(|c| c.name == n) {
                    println!("Running check: {}", check.name);
                    run_check(check, &client).await;
                } else {
                    println!("Check '{}' not found!", n);
                }
            }
            None => {
                println!("Running all health checks...");
                for check in &checks {
                    println!("Running check: {}", check.name);
                    run_check(check, &client).await;
                }
            }
        }
    }
}

async fn run_check(check: &crate::models::health_check::HealthCheck, client: &Client) {
    match check.check_type.as_str() {
        "url" => {
            if let Some(config) = &check.config {
                println!("Pinging URL: {}", config);
                let result = client.get(config).send().await;
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
                let pool = PgPoolOptions::new().connect(config).await;
                match pool {
                    Ok(_) => println!("Database connection successful!"),
                    Err(err) => println!("Failed to connect to database: {}", err),
                }
            } else {
                println!("Missing configuration for database check.");
            }
        }
        "disk" => match sys_info::disk_info() {
            Ok(disk) => {
                println!("Disk total: {} MB", disk.total / 1024);
                println!("Disk free: {} MB", disk.free / 1024);
            }
            Err(err) => {
                println!("Failed to get disk info: {}", err);
            }
        },
        _ => println!("Unsupported check type: {}", check.check_type),
    }
}
