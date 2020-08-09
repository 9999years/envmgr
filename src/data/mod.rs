use std::collections::HashMap;

use serde::Deserialize;

mod de;
mod impls;
pub use impls::*;
mod condition;
pub use condition::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "de::EnvConfig")]
pub struct EnvConfig {
    pub env: Vec<ConditionEl<VarMap>>,
    pub tests: HashMap<String, Condition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarMap(pub HashMap<String, ConditionEl<VarConfig>>);

#[derive(Debug, Clone, PartialEq)]
pub struct VarConfig {
    pub sep: String,
    pub paths: Vec<ConditionEl<ShellPath>>,
}

/// A shell path for glob expansions, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct ShellPath(String);
