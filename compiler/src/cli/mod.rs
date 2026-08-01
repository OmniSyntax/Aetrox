// 👈 (Defines the CLI commands)
use clap::{Parser, Subcommand};

pub mod interactive;
pub mod scaffold;
pub mod ui;

#[derive(Parser)]
#[command(name = "aetrox", about = "The Elite Aetrox CLI", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Aetrox project
    New,
    /// Run the current project
    Run,
}