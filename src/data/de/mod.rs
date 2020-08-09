use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wyz::conv::Conv;

mod condition;
mod impls;
mod one_or_more;
mod path_or_val;
mod utils;
pub use condition::*;
pub use impls::*;
pub use one_or_more::*;
pub use path_or_val::*;
pub use utils::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvConfig {
    env: OneOrMore<EnvMap>,
    tests: HashMap<String, Condition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvMap {
    Block(ConditionEl<Block>),
    Map(VarMap),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Block {
    block: Box<EnvMap>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VarMap(pub HashMap<String, VarConfig>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VarConfig {
    SingleString(String),
    Entries(OneOrMore<DirEntry>),
    Full(FullVarConfig),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FullVarConfig {
    #[serde(default = "default_var_sep")]
    sep: String,
    #[serde(flatten)]
    paths: PathsOrVals<ConditionEl<DirEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DirEntry {
    Plain(String),
    Conditional(Box<ConditionEl<PathOrVal<String>>>),
}
