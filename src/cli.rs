use crate::verbs::Verb;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "axl",
    version,
    about = "Stack-aware workflow driver",
    long_about = "AXL deletes stack-specific muscle memory. \
Run dev/build/test/etc. in any repo and AXL maps them to the right commands."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start the development server / app for the current project
    Dev,
    /// Build production artifacts
    Build,
    /// Run the project's test suite
    Test,
    /// Clean local build artifacts
    Clean,
    /// Fully reset the environment (node_modules, caches, etc.)
    Reset,
    /// Open the project in the default GUI (editor/IDE)
    Open,
    /// Tail/watch logs as defined by the stack
    Logs,
    /// Install the project locally (e.g., cargo install --path .)
    Install,
    /// Show the resolved stack, detection reason, and command table
    Info,
    /// Print only the detection details for debugging
    Detect,
    /// List recently used projects from the registry
    Recent,
    /// Re-run the last verb for the last or named project
    Resume {
        /// Optional fuzzy name/path to resume
        project: Option<String>,
    },
    /// Show the path (or cd command) for a recent project
    Switch {
        /// Optional fuzzy name/path to switch
        project: Option<String>,
        /// Only print the raw path (for scripts)
        #[arg(long)]
        path_only: bool,
    },
    /// Scaffold an axl.toml with defaults for this stack
    Init {
        /// Overwrite an existing file
        #[arg(long)]
        force: bool,
    },
    /// Run diagnostics: missing tools, unmapped verbs, etc.
    Doctor,
    /// Show the installed AXL version
    Version,
}

impl Commands {
    pub fn verb(&self) -> Option<Verb> {
        use Commands::*;
        match self {
            Dev => Some(Verb::Dev),
            Build => Some(Verb::Build),
            Test => Some(Verb::Test),
            Clean => Some(Verb::Clean),
            Reset => Some(Verb::Reset),
            Open => Some(Verb::Open),
            Logs => Some(Verb::Logs),
            Install => Some(Verb::Install),
            _ => None,
        }
    }
}
