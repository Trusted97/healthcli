use crate::storage::load_health_checks;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use std::time::{Instant, Duration};

pub async fn run(name: Option<String>) {
    let checks = load_health_checks();
    let client = Client::new();

    match name {
        Some(n) => {
            if let Some(check) = checks.iter().find(|c| c.name == n) {
                println!("Running check: {:?}", check.name);
                run_check(check, &client).await;
            } else {
                println!("Check '{}' not found!", n);
            }
        }
        None => {
            println!("Running all health checks...");
            for check in &checks {
                println!("Running check: {:?}", check.name);
                run_check(check, &client).await;
            }
        }
    }
}

async fn run_check(check: &crate::models::health_check::HealthCheck, client: &Client) {
    match check.check_type.as_str() {
        "url" => {
            if let Some(config) = &check.config {
                println!("Pinging URL: {}", config);

                // Record start time
                let start_time = Instant::now();

                let timeout_duration = Duration::new(5, 0);  // Timeout for 5 seconds
                let result = client.get(config).timeout(timeout_duration).send().await;

                // Calculate elapsed time
                let elapsed_time = start_time.elapsed();
                match result {
                    Ok(response) => {
                        let status = response.status();

                        // Log URL response status and response time
                        println!("URL responded with status: {}", status);
                        println!("Response time: {} ms", elapsed_time.as_millis());

                        // Check if the status is in the success range (2xx)
                        if status.is_success() {
                            println!("✅ The URL is healthy.");
                        } else {
                            println!("⚠️ The URL returned an error status.");
                        }
                    }
                    Err(err) => {
                        // Handle errors and different types of errors
                        println!("❌ Failed to ping URL: {}", err);
                    }
                }
            } else {
                println!("Missing configuration for URL check.");
            }
        }
        "database" => {
            if let Some(config) = &check.config {
                println!("Connecting to database: {}", config);
                let pool = PgPoolOptions::new()
                    .connect(config)
                    .await;
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
