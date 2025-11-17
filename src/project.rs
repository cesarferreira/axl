use crate::config::{CommandSpec, ProjectConfig};
use crate::stack::{Detection, Stack, detect_stack};
use crate::verbs::Verb;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pub struct ProjectContext {
    pub root: PathBuf,
    pub stack: Stack,
    pub detection: Detection,
    pub config: ProjectConfig,
    project_name: String,
}

impl ProjectContext {
    pub fn from_current_dir() -> Result<Self> {
        let cwd = env::current_dir().context("reading current directory")?;
        Self::from_path(&cwd)
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let root = path
            .canonicalize()
            .with_context(|| format!("canonicalizing {}", path.display()))?;
        let config = ProjectConfig::load(&root)?;
        let override_stack = config.stack_override();
        let detection = detect_stack(&root, override_stack, Some(config.path()));
        let stack = detection.stack;
        let project_name = config
            .project_name()
            .map(|s| s.to_string())
            .or_else(|| {
                root.file_name()
                    .and_then(|name| name.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| root.display().to_string());

        Ok(Self {
            root,
            stack,
            detection,
            config,
            project_name,
        })
    }

    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    pub fn has_config(&self) -> bool {
        self.config.has_file()
    }

    pub fn config_path(&self) -> &Path {
        self.config.path()
    }

    pub fn resolved_command(&self, verb: Verb) -> Option<ResolvedCommand> {
        if let Some(spec) = self.config.command_for(verb) {
            return Some(ResolvedCommand::new(verb, spec, CommandOrigin::Config));
        }

        let default = self.stack.default_command(verb)?;
        let spec = CommandSpec {
            cmd: default.cmd,
            requires: default.requires,
            env: HashMap::new(),
        };
        Some(ResolvedCommand::new(
            verb,
            spec,
            CommandOrigin::Default(self.stack),
        ))
    }
}

pub struct ResolvedCommand {
    pub verb: Verb,
    pub cmd: String,
    pub requires: Vec<String>,
    pub env: HashMap<String, String>,
    pub origin: CommandOrigin,
}

impl ResolvedCommand {
    fn new(verb: Verb, spec: CommandSpec, origin: CommandOrigin) -> Self {
        Self {
            verb,
            cmd: spec.cmd,
            requires: spec.requires,
            env: spec.env,
            origin,
        }
    }

    pub fn origin_label(&self) -> String {
        self.origin.label()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CommandOrigin {
    Config,
    Default(Stack),
}

impl CommandOrigin {
    pub fn label(self) -> String {
        match self {
            CommandOrigin::Config => "axl.toml".into(),
            CommandOrigin::Default(stack) => format!("{} defaults", stack.label()),
        }
    }
}
