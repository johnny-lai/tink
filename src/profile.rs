use anyhow::{bail, Result};
use clap::{Args, ValueEnum};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

#[derive(ValueEnum, Debug, Clone, Default)]
pub(crate) enum Target {
    #[default]
    Zed,
    Vscode,
}

#[derive(Args, Debug)]
pub(crate) struct Profile {
    /// Language to select the debug adapter (go, rust, c, cpp, python, js, ts, php)
    #[arg(short, long)]
    language: String,

    /// Display name for the profile (defaults to "Debug <program>")
    #[arg(short = 'n', long)]
    label: Option<String>,

    /// Target editor [default: zed]
    #[arg(short, long, value_enum, default_value_t = Target::Zed)]
    target: Target,

    /// Program and its arguments (everything after --)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    program_args: Vec<String>,
}

trait DebugEditor: Sized {
    fn load() -> Result<Self>;
    fn add(&mut self, profile: &Profile) -> Result<()>;
    fn replace(&mut self, profile: &Profile);
    fn save(&self) -> Result<()>;
}

fn do_add<T: DebugEditor>(profile: &Profile) -> Result<()> {
    let mut editor = T::load()?;
    editor.add(profile)?;
    editor.save()
}

fn do_replace<T: DebugEditor>(profile: &Profile) -> Result<()> {
    let mut editor = T::load()?;
    editor.replace(profile);
    editor.save()
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
        match self.profile.target {
            Target::Zed => do_add::<ZedDebug>(&self.profile),
            Target::Vscode => do_add::<VscodeDebug>(&self.profile),
        }
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
        match self.profile.target {
            Target::Zed => do_replace::<ZedDebug>(&self.profile),
            Target::Vscode => do_replace::<VscodeDebug>(&self.profile),
        }
    }
}

// --- Zed ---

struct ZedDebug {
    profiles: Vec<Value>,
}

impl DebugEditor for ZedDebug {
    fn load() -> Result<ZedDebug> {
        let path = Path::new(".zed/debug.json");
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let profiles: Vec<Value> = serde_json::from_str(&content)?;
            Ok(ZedDebug { profiles })
        } else {
            Ok(ZedDebug { profiles: Vec::new() })
        }
    }

    fn add(&mut self, profile: &Profile) -> Result<()> {
        let value = Self::to_json(profile);
        let label = value["label"].as_str().unwrap_or_default();
        if self.profiles.iter().any(|p| p["label"].as_str() == Some(label)) {
            bail!("profile with label '{}' already exists", label);
        }
        self.profiles.push(value);
        Ok(())
    }

    fn replace(&mut self, profile: &Profile) {
        let value = Self::to_json(profile);
        let label = value["label"].as_str().unwrap_or_default();
        match self.profiles.iter().position(|p| p["label"].as_str() == Some(label)) {
            Some(i) => self.profiles[i] = value,
            None => self.profiles.push(value),
        }
    }

    fn save(&self) -> Result<()> {
        let path = Path::new(".zed/debug.json");
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, serde_json::to_string_pretty(&self.profiles)?)?;
        Ok(())
    }
}

impl ZedDebug {
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
            "rust" | "c" | "cpp" | "c++" => Some("CodeLLDB"),
            "python" => Some("Debugpy"),
            "javascript" | "js" | "typescript" | "ts" => Some("JavaScript"),
            "php" => Some("PHP"),
            _ => None,
        }
    }
}

// --- VSCode ---

struct VscodeDebug {
    configurations: Vec<Value>,
}

impl DebugEditor for VscodeDebug {
    fn load() -> Result<VscodeDebug> {
        let path = Path::new(".vscode/launch.json");
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let json: Value = serde_json::from_str(&content)?;
            let configurations = json["configurations"].as_array().cloned().unwrap_or_default();
            Ok(VscodeDebug { configurations })
        } else {
            Ok(VscodeDebug { configurations: Vec::new() })
        }
    }

    fn add(&mut self, profile: &Profile) -> Result<()> {
        let value = Self::to_json(profile);
        let name = value["name"].as_str().unwrap_or_default();
        if self.configurations.iter().any(|p| p["name"].as_str() == Some(name)) {
            bail!("configuration with name '{}' already exists", name);
        }
        self.configurations.push(value);
        Ok(())
    }

    fn replace(&mut self, profile: &Profile) {
        let value = Self::to_json(profile);
        let name = value["name"].as_str().unwrap_or_default();
        match self.configurations.iter().position(|p| p["name"].as_str() == Some(name)) {
            Some(i) => self.configurations[i] = value,
            None => self.configurations.push(value),
        }
    }

    fn save(&self) -> Result<()> {
        let path = Path::new(".vscode/launch.json");
        fs::create_dir_all(path.parent().unwrap())?;
        let content = json!({
            "version": "0.2.0",
            "configurations": self.configurations,
        });
        fs::write(path, serde_json::to_string_pretty(&content)?)?;
        Ok(())
    }
}

impl VscodeDebug {
    fn to_json(profile: &Profile) -> Value {
        let program = &profile.program_args[0];
        let args: Vec<&String> = profile.program_args[1..].iter().collect();
        let default_label = format!("Debug {program}");
        let label = profile.label.as_deref().unwrap_or(&default_label);

        let mut value = json!({
            "name": label,
            "type": Self::language_to_type(&profile.language),
            "request": "launch",
            "program": program,
        });
        if !args.is_empty() {
            value["args"] = json!(args);
        }
        value
    }

    fn language_to_type(language: &str) -> Option<&'static str> {
        match language.to_lowercase().as_str() {
            "go" => Some("go"),
            "rust" | "c" | "cpp" | "c++" => Some("lldb"),
            "python" => Some("debugpy"),
            "javascript" | "js" | "typescript" | "ts" => Some("node"),
            "php" => Some("php"),
            _ => None,
        }
    }
}
