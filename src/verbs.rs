use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Verb {
    Dev,
    Build,
    Test,
    Clean,
    Reset,
    Open,
    Logs,
    Install,
}

impl Verb {
    pub const ALL: [Verb; 8] = [
        Verb::Dev,
        Verb::Build,
        Verb::Test,
        Verb::Clean,
        Verb::Reset,
        Verb::Open,
        Verb::Logs,
        Verb::Install,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Verb::Dev => "dev",
            Verb::Build => "build",
            Verb::Test => "test",
            Verb::Clean => "clean",
            Verb::Reset => "reset",
            Verb::Open => "open",
            Verb::Logs => "logs",
            Verb::Install => "install",
        }
    }
}

impl Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Verb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Verb::Dev),
            "build" => Ok(Verb::Build),
            "test" => Ok(Verb::Test),
            "clean" => Ok(Verb::Clean),
            "reset" => Ok(Verb::Reset),
            "open" => Ok(Verb::Open),
            "logs" => Ok(Verb::Logs),
            "install" => Ok(Verb::Install),
            other => Err(format!("unknown verb '{other}'")),
        }
    }
}
