use anyhow::Result;
use clap::Parser;

mod add;

#[derive(Parser, Debug)]
#[command(name = "tink", about = "Generate launch profiles")]
enum Args {
    Add(add::FnAdd),
}

fn main() -> Result<()> {
    match Args::parse() {
        Args::Add(cmd) => cmd.run(),
    }
}
