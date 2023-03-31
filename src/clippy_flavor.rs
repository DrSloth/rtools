use std::fmt::{self, Display, Formatter};

use clap::ValueEnum;

/// Enumeration describing how harsh clippy should be
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, ValueEnum)]
pub enum ClippyFlavor {
    /// The pedantic style, this will be used by CI/CD for main branches in projects
    Pedantic,
    /// Rather pedantic style for local development/other branches. This is less pedantic than
    /// pedantic, but pedantic will be used for release/deployment builds (the main branch)
    #[default]
    Development,
    /// Unpedantic style usable for small scripts (meh) or prototypes
    Prototype,
}

impl Display for ClippyFlavor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let str_repr = match *self {
            Self::Pedantic => "pedantic",
            Self::Development => "development",
            Self::Prototype => "prototype",
        };

        write!(f, "{}", str_repr)
    }
}
