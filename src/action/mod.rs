use std::fmt::Display;

use derive_more::{AsRef, Display};

pub mod pnpm;
pub mod yarn;

#[derive(Debug, Display)]
#[display(fmt = "{name}{}", "self.version_fmt()")]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
}

impl Package {
    fn version_fmt(&self) -> String {
        self.version.map(|a| format!("@{a}")).unwrap_or_default()
    }
}

#[derive(AsRef, Debug)]
pub struct Packages(Vec<Package>);

impl Display for Packages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

#[derive(Debug, Display)]
pub enum Operation {
    Synchronize(OpSync),
    Query(OpQuery),
}

#[derive(Debug, Display)]
pub enum OpSync {
    #[display(fmt = "-S {_0}")]
    Install(Packages),
    #[display(fmt = "-Si {_0}")]
    Info(Packages),
    #[display(fmt = "-Ss {_0}")]
    Search(Package),
    #[display(fmt = "-Sy")]
    RefreshPkgDb,
    #[display(fmt = "-Syu")]
    Upgrade,
}

#[derive(Debug, Display)]
pub enum OpQuery {
    #[display(fmt = "-Q")]
    ShowAll,
    #[display(fmt = "-Q {_0}")]
    SimpleInfo(Packages),
    #[display(fmt = "-Qi {_0}")]
    FullInfo(Packages),
    #[display(fmt = "-Qs {_0}")]
    Search(Package),
}
