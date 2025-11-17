use crate::verbs::Verb;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct ProjectConfig {
    path: PathBuf,
    raw: Option<RawConfig>,
}

impl ProjectConfig {
    pub fn load(root: &Path) -> Result<Self> {
        let path = root.join("axl.toml");
        if !path.exists() {
            return Ok(Self { path, raw: None });
        }

        let content =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let raw: RawConfig =
            toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
        Ok(Self {
            path,
            raw: Some(raw),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn has_file(&self) -> bool {
        self.raw.is_some()
    }

    pub fn project_name(&self) -> Option<&str> {
        self.raw
            .as_ref()
            .and_then(|raw| raw.project.as_ref())
            .and_then(|p| p.name.as_deref())
    }

    pub fn stack_override(&self) -> Option<crate::stack::Stack> {
        self.raw
            .as_ref()
            .and_then(|raw| raw.project.as_ref())
            .and_then(|p| p.stack)
    }

    pub fn command_for(&self, verb: Verb) -> Option<CommandSpec> {
        self.raw
            .as_ref()
            .and_then(|raw| raw.commands.get(verb.as_str()))
            .map(CommandSpec::from)
    }
}

#[derive(Clone, Debug, Deserialize)]
struct RawConfig {
    #[serde(default)]
    project: Option<ProjectSection>,
    #[serde(flatten)]
    commands: HashMap<String, CommandEntry>,
}

#[derive(Clone, Debug, Deserialize)]
struct ProjectSection {
    name: Option<String>,
    stack: Option<crate::stack::Stack>,
}

#[derive(Clone, Debug, Deserialize)]
struct CommandEntry {
    cmd: String,
    #[serde(default)]
    requires: Vec<String>,
    #[serde(default)]
    env: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct CommandSpec {
    pub cmd: String,
    pub requires: Vec<String>,
    pub env: HashMap<String, String>,
}

impl From<&CommandEntry> for CommandSpec {
    fn from(entry: &CommandEntry) -> Self {
        CommandSpec {
            cmd: entry.cmd.clone(),
            requires: entry.requires.clone(),
            env: entry.env.clone(),
        }
    }
}
