use clap::{Parser, Subcommand};
use std::fs;

mod lexer;
mod parser; // Make sure your parser module is declared

use lexer::{SourceFile, Lexer};
use parser::Parser as AstParser; // Rename to avoid conflict with clap::Parser

#[derive(Parser)]
#[command(name = "aetrox")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { path: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path } => {
            println!("⚡ Compiling Aetrox target: {}", path);

            let code = fs::read_to_string(&path).expect("Failed to read .ax source file");
            let source_file = SourceFile::new(&path, &code);
            let mut lexer = Lexer::new(&source_file);

            // 1. Collect all tokens from the lexer into a Vector
            let mut tokens = Vec::new();
            for token_result in lexer {
                match token_result {
                    Ok(t) => tokens.push(t),
                    Err(_) => eprintln!("Lexing Error encountered"),
                }
            }

            println!("🔍 Tokens collected: {:?}", tokens.len());

            // 2. Pass tokens into the Parser
            let mut parser = AstParser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    println!("🌳 AST Parsing Successful!");
                    println!("{:#?}", ast);
                }
                Err(e) => {
                    eprintln!("❌ Parser Error: {}", e);
                }
            }
        }
    }
}