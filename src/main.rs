use clap::{Parser, Subcommand};

mod commands;
mod models;
mod storage;

#[derive(Parser)]
#[command(name = "HealthCLI")]
#[command(version = "1.0")]
#[command(about = "A cli for setup health check", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        name: Option<String>,
    },
    Register {
        name: String,
        check_type: String,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { name } => {
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
