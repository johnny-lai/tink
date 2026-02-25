use anyhow::{bail, Result};
use clap::Args;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

#[derive(Args, Debug)]
pub(crate) struct Profile {
    /// Language to select the debug adapter (go, rust)
    #[arg(short, long)]
    language: String,

    /// Display name for the profile (defaults to "Debug <program>")
    #[arg(short = 'n', long)]
    label: Option<String>,

    /// Program and its arguments (everything after --)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    program_args: Vec<String>,
}

#[derive(Args, Debug)]
pub(crate) struct FnAdd {
    #[command(flatten)]
    profile: Profile,
}

impl FnAdd {
    pub fn run(&self) -> Result<()> {
        if self.profile.program_args.is_empty() {
            bail!("no program specified")
        }

        let mut zed_debug = ZedDebug::load()?;
        zed_debug.add(&self.profile)?;
        zed_debug.save()
    }
}

#[derive(Args, Debug)]
pub(crate) struct FnReplace {
    #[command(flatten)]
    profile: Profile,
}

impl FnReplace {
    pub fn run(&self) -> Result<()> {
        if self.profile.program_args.is_empty() {
            bail!("no program specified")
        }

        let mut zed_debug = ZedDebug::load()?;
        zed_debug.replace(&self.profile);
        zed_debug.save()
    }
}

pub(crate) struct ZedDebug {
    profiles: Vec<Value>,
}

impl ZedDebug {
    pub fn load() -> Result<ZedDebug> {
        let debug_path = Path::new(".zed/debug.json");
        if debug_path.exists() {
            let content = fs::read_to_string(debug_path)?;
            let profiles: Vec<Value> = serde_json::from_str(&content)?;
            Ok(ZedDebug { profiles })
        } else {
            Ok(ZedDebug { profiles: Vec::new() })
        }
    }

    fn to_json(profile: &Profile) -> Value {
        let program = &profile.program_args[0];
        let args: Vec<&String> = profile.program_args[1..].iter().collect();

        let default_label = format!("Debug {program}");
        let label = profile.label.as_deref().unwrap_or(&default_label);

        let mut value = json!({
            "label": label,
            "adapter": Self::language_to_adapter(&profile.language),
            "request": "launch",
            "program": program,
        });

        if !args.is_empty() {
            value["args"] = json!(args);
        }

        value
    }

    fn language_to_adapter(language: &str) -> Option<&'static str> {
        match language.to_lowercase().as_str() {
            "go" => Some("Go"),
            "rust" => Some("CodeLLDB"),
            "c" | "cpp" | "c++" => Some("CodeLLDB"),
            "python" => Some("Debugpy"),
            "javascript" | "js" => Some("JavaScript"),
            "typescript" | "ts" => Some("JavaScript"),
            "php" => Some("PHP"),
            _ => None,
        }
    }

    pub fn add(&mut self, profile: &Profile) -> Result<()> {
        let value = Self::to_json(profile);

        let label = value["label"].as_str().unwrap_or_default();
        if self.profiles.iter().any(|p| p["label"].as_str() == Some(label)) {
            bail!("profile with label '{}' already exists", label);
        }

        self.profiles.push(value);
        Ok(())
    }

    pub fn replace(&mut self, profile: &Profile) {
        let value = Self::to_json(profile);
        let label = value["label"].as_str().unwrap_or_default();
        match self.profiles.iter().position(|p| p["label"].as_str() == Some(label)) {
            Some(i) => self.profiles[i] = value,
            None => self.profiles.push(value),
        }
    }

    pub fn save(&self) -> Result<()> {
        let debug_path = Path::new(".zed/debug.json");
        if let Some(parent) = debug_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.profiles)?;
        fs::write(debug_path, content)?;
        Ok(())
    }
}
