use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wyz::conv::Conv;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvConfig {
    env: VarEntries,
}

impl From<EnvConfig> for super::EnvConfig {
    fn from(cfg: EnvConfig) -> Self {
        Self {
            env: cfg.env.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum VarEntries {
    Map(VarEntry),
    Vec(Vec<VarEntry>),
}

impl Into<HashMap<String, super::VarConfig>> for VarEntries {
    fn into(self) -> HashMap<String, super::VarConfig> {
        match self {
            VarEntries::Map(vars) => vars
                .into_iter()
                .map(|(var, cfg)| (var, cfg.conv::<super::VarConfig>()))
                .collect(),
            VarEntries::Vec(groups) => groups
                .into_iter()
                .flat_map(|vars| {
                    vars.into_iter()
                        .map(|(var, cfg)| (var, cfg.conv::<super::VarConfig>()))
                })
                .collect(),
        }
    }
}

type VarEntry = HashMap<String, VarConfigWrapper>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VarConfigWrapper {
    Plain(Vec<DirEntry>),
    Wrapped(VarConfig),
}

impl From<VarConfigWrapper> for super::VarConfig {
    fn from(config: VarConfigWrapper) -> Self {
        match config {
            VarConfigWrapper::Plain(paths) => Self {
                sep: default_var_sep(),
                paths: paths
                    .into_iter()
                    .map(|entry| entry.conv::<super::DirEntry>())
                    .collect(),
            },
            VarConfigWrapper::Wrapped(cfg) => cfg.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VarConfig {
    #[serde(default = "default_var_sep")]
    sep: String,
    #[serde(
        alias = "vals",
        alias = "values",
        alias = "directories",
        alias = "dirs",
        alias = "files"
    )]
    paths: Vec<DirEntry>,
}

impl From<VarConfig> for super::VarConfig {
    fn from(cfg: VarConfig) -> Self {
        Self {
            sep: cfg.sep,
            paths: cfg
                .paths
                .into_iter()
                .map(|entry| entry.conv::<super::DirEntry>())
                .collect(),
        }
    }
}

fn default_var_sep() -> String {
    if cfg!(windows) { ";" } else { ":" }.to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DirEntry {
    Plain(String),
    Conditional(ConditionalDirEntry),
}

impl From<DirEntry> for super::DirEntry {
    fn from(entry: DirEntry) -> Self {
        match entry {
            DirEntry::Plain(path) => Self {
                path,
                when: Default::default(),
            },
            DirEntry::Conditional(ConditionalDirEntry { path, when }) => Self {
                path,
                when: when.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalDirEntry {
    #[serde(
        alias = "val",
        alias = "value",
        alias = "dir",
        alias = "directory",
        alias = "file"
    )]
    path: String,

    #[serde(alias = "if")]
    when: ConditionWrapper,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionWrapper {
    UntaggedOr(Vec<Condition>),
    Tagged(Condition),
}

impl From<ConditionWrapper> for Condition {
    fn from(wrapper: ConditionWrapper) -> Self {
        match wrapper {
            ConditionWrapper::UntaggedOr(conds) => Self::Or(conds),
            ConditionWrapper::Tagged(cond) => cond,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    /// Compound condition true when any of the given conditions is true
    #[serde(alias = "any")]
    Or(Vec<Condition>),
    /// Compound condition true when all of the given conditions are true
    #[serde(alias = "all")]
    And(Vec<Condition>),
    /// True when the hostname matches a glob according to [`globset`'s
    /// syntax][globset]
    /// [globset]: https://docs.rs/globset/0.4.5/globset/#syntax
    #[serde(alias = "hostname", alias = "host_glob", alias = "hostname_glob")]
    Host(String),
    /// True when the hostname matches a regular expression
    #[serde(alias = "hostname_re")]
    HostRe(String),
    /// Glob match against the operating system family, `windows` or `unix`
    #[serde(alias = "os_family_glob", alias = "family", alias = "family_glob")]
    OsFamily(String),
    /// Glob match against the operating system
    #[serde(alias = "os_glob")]
    Os(String),
    /// Glob match against the CPU architeture
    #[serde(alias = "architecture")]
    Arch(String),
    /// Trivial always-true condition
    True,
    /// Trivial always-false condition
    False,
}

impl Default for Condition {
    fn default() -> Self {
        Condition::True
    }
}
