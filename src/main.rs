use clap::{Parser, Subcommand};

mod commands;
mod models;
mod storage;

#[derive(Parser)]
#[command(name = "HealthCLI")]
#[command(version = "1.0")]
#[command(about = "A CLI to manage and perform health checks", long_about = "HealthCLI allows you to register, list, and execute health checks for various system components such as URLs, databases, and disk space.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute health checks
    Check {
        /// Name of the specific health check to execute (optional)
        #[arg(short, long, help = "Specify the name of the health check to run. If not provided, all checks will run.")]
        name: Option<String>,

        /// Type of health check to run (e.g., url, database, disk)
        #[arg(short, long, help = "Filter checks by type. Valid options: 'url', 'database', 'disk'.")]
        check_type: Option<String>,
    },
    /// Register a new health check
    Register {
        /// Name of the health check to register
        #[arg(short, long, help = "Name of the new health check.")]
        name: String,

        /// Type of the health check (e.g., url, database, disk)
        #[arg(short, long, help = "Type of the health check to register.")]
        check_type: String,
    },
    /// List all registered health checks
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { name, .. } => {
            commands::check::run(name);
        }
        Commands::Register { name, check_type } => {
            commands::register::run(name, check_type);
        }
        Commands::List => {
            commands::list::run();
        }
    }
}
