use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use super::*;

impl From<HashMap<String, ConditionEl<VarConfig>>> for VarMap {
    fn from(map: HashMap<String, ConditionEl<VarConfig>>) -> Self {
        Self(map)
    }
}

impl Into<HashMap<String, ConditionEl<VarConfig>>> for VarMap {
    fn into(self) -> HashMap<String, ConditionEl<VarConfig>> {
        self.0
    }
}

impl Deref for VarMap {
    type Target = HashMap<String, ConditionEl<VarConfig>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VarMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<ConditionEl<ShellPath>>> for VarConfig {
    fn from(paths: Vec<ConditionEl<ShellPath>>) -> Self {
        Self {
            sep: super::de::default_var_sep(),
            paths,
        }
    }
}

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
