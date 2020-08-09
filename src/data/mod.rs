use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use serde::Deserialize;

mod de;
pub use de::Condition;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "de::EnvConfig")]
pub struct EnvConfig {
    pub env: HashMap<String, VarConfig>,
    pub tests: HashMap<String, Condition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarConfig {
    pub sep: String,
    pub paths: Vec<DirEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirEntry {
    pub path: ShellPath,
    pub when: Condition,
}

/// A shell path for glob expansions, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct ShellPath(String);

impl Deref for ShellPath {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ShellPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for ShellPath {
    fn from(path: String) -> Self {
        ShellPath(path)
    }
}

impl Into<String> for ShellPath {
    fn into(self) -> String {
        self.0
    }
}

impl Into<PathBuf> for ShellPath {
    fn into(self) -> PathBuf {
        PathBuf::from(self.0)
    }
}
