use std::fmt::{self, Display, Formatter};

use clap::ArgEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, ArgEnum)]
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
        let str_repr = match self {
            ClippyFlavor::Pedantic => "pedantic",
            ClippyFlavor::Development => "development",
            ClippyFlavor::Prototype => "prototype",
        };

        write!(f, "{}", str_repr)
    }
}
