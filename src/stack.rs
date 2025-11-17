use crate::verbs::Verb;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stack {
    Bun,
    Node,
    Gradle,
    Rust,
    Flutter,
    Melos,
    Bazel,
    Robin,
    Python,
    Generic,
}

impl Stack {
    pub fn label(self) -> &'static str {
        match self {
            Stack::Bun => "Bun",
            Stack::Node => "Node.js",
            Stack::Gradle => "Gradle / Android",
            Stack::Rust => "Rust",
            Stack::Flutter => "Flutter",
            Stack::Melos => "Melos",
            Stack::Bazel => "Bazel",
            Stack::Robin => "Robin",
            Stack::Python => "Python",
            Stack::Generic => "Generic",
        }
    }

    pub fn keyword(self) -> &'static str {
        match self {
            Stack::Bun => "bun",
            Stack::Node => "node",
            Stack::Gradle => "gradle",
            Stack::Rust => "rust",
            Stack::Flutter => "flutter",
            Stack::Melos => "melos",
            Stack::Bazel => "bazel",
            Stack::Robin => "robin",
            Stack::Python => "python",
            Stack::Generic => "generic",
        }
    }

    pub fn default_command(self, verb: Verb) -> Option<DefaultCommand> {
        use Verb::*;

        let cmd = match (self, verb) {
            (Stack::Bun, Dev) => Some("bun run dev"),
            (Stack::Bun, Build) => Some("bun run build"),
            (Stack::Bun, Test) => Some("bun run test"),
            (Stack::Bun, Clean) => Some("rm -rf node_modules bun.lockb"),
            (Stack::Bun, Reset) => Some("rm -rf node_modules bun.lockb && bun install"),
            (Stack::Bun, Open) => Some(default_open_command()),
            (Stack::Bun, Logs) => Some("bun run dev -- --verbose"),

            (Stack::Node, Dev) => Some("npm run dev"),
            (Stack::Node, Build) => Some("npm run build"),
            (Stack::Node, Test) => Some("npm test"),
            (Stack::Node, Clean) => Some("rm -rf node_modules dist"),
            (Stack::Node, Reset) => Some("rm -rf node_modules package-lock.json && npm install"),
            (Stack::Node, Open) => Some(default_open_command()),
            (Stack::Node, Logs) => Some("npm run logs"),

            (Stack::Gradle, Dev) => Some("./gradlew :app:installDebug"),
            (Stack::Gradle, Build) => Some("./gradlew assemble"),
            (Stack::Gradle, Test) => Some("./gradlew test"),
            (Stack::Gradle, Clean) => Some("./gradlew clean"),
            (Stack::Gradle, Reset) => Some("./gradlew clean && ./gradlew --stop"),
            (Stack::Gradle, Open) => Some(default_open_command()),
            (Stack::Gradle, Logs) => Some("adb logcat"),

            (Stack::Rust, Dev) => Some("cargo run"),
            (Stack::Rust, Build) => Some("cargo build"),
            (Stack::Rust, Test) => Some("cargo test"),
            (Stack::Rust, Clean) => Some("cargo clean"),
            (Stack::Rust, Reset) => Some("cargo clean && cargo fetch"),
            (Stack::Rust, Open) => Some(default_open_command()),
            (Stack::Rust, Logs) => Some("cargo test -- --nocapture"),

            (Stack::Flutter, Dev) => Some("flutter run"),
            (Stack::Flutter, Build) => Some("flutter build"),
            (Stack::Flutter, Test) => Some("flutter test"),
            (Stack::Flutter, Clean) => Some("flutter clean"),
            (Stack::Flutter, Reset) => Some("flutter clean && flutter pub get"),
            (Stack::Flutter, Open) => Some(default_open_command()),
            (Stack::Flutter, Logs) => Some("flutter logs"),

            (Stack::Melos, Dev) => Some("melos run dev"),
            (Stack::Melos, Build) => Some("melos run build"),
            (Stack::Melos, Test) => Some("melos run test"),
            (Stack::Melos, Clean) => Some("melos clean"),
            (Stack::Melos, Reset) => Some("melos clean && melos bootstrap"),
            (Stack::Melos, Open) => Some(default_open_command()),
            (Stack::Melos, Logs) => Some("melos run logs"),

            (Stack::Bazel, Dev) => Some("bazel run //..."),
            (Stack::Bazel, Build) => Some("bazel build //..."),
            (Stack::Bazel, Test) => Some("bazel test //..."),
            (Stack::Bazel, Clean) => Some("bazel clean"),
            (Stack::Bazel, Reset) => Some("bazel clean --expunge"),
            (Stack::Bazel, Open) => Some(default_open_command()),
            (Stack::Bazel, Logs) => Some("bazel test //... --test_output=all"),

            (Stack::Robin, Dev) => Some("robin dev:start"),
            (Stack::Robin, Build) => Some("robin build"),
            (Stack::Robin, Test) => Some("robin test"),
            (Stack::Robin, Clean) => Some("robin clean"),
            (Stack::Robin, Reset) => Some("robin reset"),
            (Stack::Robin, Open) => Some(default_open_command()),
            (Stack::Robin, Logs) => Some("robin dev:logs"),

            (Stack::Python, Dev) => Some("python -m app"),
            (Stack::Python, Build) => Some("python -m build"),
            (Stack::Python, Test) => Some("pytest"),
            (Stack::Python, Clean) => Some("rm -rf .venv __pycache__ build dist"),
            (Stack::Python, Reset) => Some(
                "rm -rf .venv && python -m venv .venv && . ./.venv/bin/activate && pip install -r requirements.txt",
            ),
            (Stack::Python, Open) => Some(default_open_command()),
            (Stack::Python, Logs) => Some("tail -f logs/*.log"),

            (Stack::Generic, Open) => Some(default_open_command()),
            _ => None,
        }?;

        let requires = match self {
            Stack::Bun => vec!["bun"],
            Stack::Node => vec!["node", "npm"],
            Stack::Gradle => vec!["java", "gradle"],
            Stack::Rust => vec!["cargo"],
            Stack::Flutter => vec!["flutter"],
            Stack::Melos => vec!["melos"],
            Stack::Bazel => vec!["bazel"],
            Stack::Robin => vec!["robin"],
            Stack::Python => vec!["python"],
            Stack::Generic => vec![],
        }
        .into_iter()
        .map(String::from)
        .collect();

        Some(DefaultCommand {
            cmd: cmd.to_string(),
            requires,
        })
    }
}

fn default_open_command() -> &'static str {
    if cfg!(target_os = "macos") {
        "open ."
    } else if cfg!(target_os = "windows") {
        "explorer ."
    } else {
        "xdg-open ."
    }
}

pub struct DefaultCommand {
    pub cmd: String,
    pub requires: Vec<String>,
}

pub struct Detection {
    pub stack: Stack,
    pub reason: DetectionReason,
}

pub enum DetectionReason {
    Config(String),
    Marker { path: String },
    Fallback,
}

impl DetectionReason {
    pub fn describe(&self) -> String {
        match self {
            DetectionReason::Config(path) => format!("stack set via {}", path),
            DetectionReason::Marker { path } => format!("found {path}"),
            DetectionReason::Fallback => "no markers found (generic fallback)".to_string(),
        }
    }
}

impl fmt::Display for DetectionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.describe())
    }
}

struct Marker {
    stack: Stack,
    path: &'static str,
}

const MARKERS: &[Marker] = &[
    Marker {
        stack: Stack::Node,
        path: "package.json",
    },
    Marker {
        stack: Stack::Bun,
        path: "bun.lockb",
    },
    Marker {
        stack: Stack::Gradle,
        path: "gradlew",
    },
    Marker {
        stack: Stack::Rust,
        path: "Cargo.toml",
    },
    Marker {
        stack: Stack::Flutter,
        path: "pubspec.yaml",
    },
    Marker {
        stack: Stack::Melos,
        path: "melos.yaml",
    },
    Marker {
        stack: Stack::Bazel,
        path: "BUILD.bazel",
    },
    Marker {
        stack: Stack::Robin,
        path: ".robin.json",
    },
    Marker {
        stack: Stack::Python,
        path: "pyproject.toml",
    },
    Marker {
        stack: Stack::Generic,
        path: ".git",
    },
];

pub fn detect_stack(
    dir: &Path,
    override_stack: Option<Stack>,
    config_path: Option<&Path>,
) -> Detection {
    if let Some(stack) = override_stack {
        return Detection {
            stack,
            reason: DetectionReason::Config(
                config_path
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| dir.join("axl.toml").display().to_string()),
            ),
        };
    }

    for marker in MARKERS {
        if dir.join(marker.path).exists() {
            return Detection {
                stack: marker.stack,
                reason: DetectionReason::Marker {
                    path: marker.path.into(),
                },
            };
        }
    }

    Detection {
        stack: Stack::Generic,
        reason: DetectionReason::Fallback,
    }
}
