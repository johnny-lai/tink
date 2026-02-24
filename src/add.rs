use anyhow::{bail, Result};
use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct FnAdd {
    /// Language to select the debug adapter (go, rust)
    #[arg(short, long)]
    language: String,

    /// Program and its arguments (everything after --)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    program_args: Vec<String>,
}

impl FnAdd {
    pub fn run(&self) -> Result<()> {
        if self.program_args.is_empty() {
            bail!("no program specified")
        }

        println!("add: {self:?}");
        Ok(())
    }
}
