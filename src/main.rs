use anyhow::Result;
use clap::Parser;

mod profile;

#[derive(Parser, Debug)]
#[command(name = "tink", about = "Generate launch profiles")]
enum Args {
    Add(profile::FnAdd),
    Replace(profile::FnReplace),
}

fn main() -> Result<()> {
    match Args::parse() {
        Args::Add(cmd) => cmd.run(),
        Args::Replace(cmd) => cmd.run(),
    }
}
