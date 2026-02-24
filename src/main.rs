use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "tink", about = "Generate launch profiles")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        /// Language to select the debug adapter (go, rust)
        #[arg(short, long)]
        language: String,

        /// Program and its arguments (everything after --)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        program_args: Vec<String>,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Add {
            language: _,
            program_args,
        } => {
            if program_args.is_empty() {
                eprintln!("Error: no program specified. Use -- <program> [args...]");
                process::exit(1);
            }
        }
    }
}
