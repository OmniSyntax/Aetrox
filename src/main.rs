// use clap::{Parser, Subcommand};
// use std::fs;

// // Bring your modular lexer components into scope
// mod lexer;
// use lexer::{SourceFile, Lexer};

// #[derive(Parser)]
// #[command(name = "aetrox")]
// #[command(about = "The Aetrox High-Performance Programming Language Compiler", version = "0.1.0")]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Compile and run an Aetrox source file
//     Run {
//         /// Path to the .ax source file
//         path: String,
//     },
// }

// fn main() {
//     let cli = Cli::parse();

//     match cli.command {
//         Commands::Run { path } => {
//             println!("⚡ Compiling Aetrox target: {}", path);

//             // 1. Read the .ax file from disk
//             let code = fs::read_to_string(&path).expect("Failed to read .ax source file");
            
//             // 2. Wrap it inside your SourceFile structure for error tracking
//             let source_file = SourceFile::new(&path, &code);

//             // 3. Initialize the modular Lexer
//             let lexer = Lexer::new(&source_file);

//             println!("🔍 Scanning tokens...");
            
//             // 4. Iterate through tokens and print results
//             for token in lexer {
//                 match token {
//                     Ok(t) => println!("Token: {:?}", t),
//                     Err(_) => eprintln!("Lexing Error: Invalid syntax token encountered"),
//                 }
//             }

//             // TODO: Pass tokens into Parser next
//         }
//     }
// }

use clap::{Parser, Subcommand};
use std::fs;

mod lexer;
use lexer::{SourceFile, Lexer};

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
            let lexer = Lexer::new(&source_file);

            println!("🔍 Scanning tokens...");
            for token in lexer {
                match token {
                    Ok(t) => println!("Token: {:?}", t),
                    Err(_) => eprintln!("Lexing Error: Invalid syntax token encountered"),
                }
            }
        }
    }
}













// cargo run -- run examples/hello.ax