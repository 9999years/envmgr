use std::collections::HashMap;

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
    pub path: String,
    pub when: Condition,
}
