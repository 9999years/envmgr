use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionEl<T> {
    #[serde(flatten)]
    value: T,

    #[serde(alias = "if", default)]
    when: ConditionWrapper,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionWrapper {
    Bool(bool),
    UntaggedOr(Vec<Condition>),
    Tagged(Condition),
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
    /// True when a given environment variable is non-empty
    Var(String),
    /// Trivial always-true condition
    True,
    /// Trivial always-false condition
    False,
}

impl Default for ConditionWrapper {
    fn default() -> Self {
        Self::Tagged(Condition::True)
    }
}

impl From<bool> for Condition {
    fn from(val: bool) -> Self {
        match val {
            true => Condition::True,
            false => Condition::False,
        }
    }
}

impl From<ConditionWrapper> for Condition {
    fn from(wrapper: ConditionWrapper) -> Self {
        match wrapper {
            ConditionWrapper::UntaggedOr(conds) => Self::Or(conds),
            ConditionWrapper::Tagged(cond) => cond,
            ConditionWrapper::Bool(val) => val.into(),
        }
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition::True
    }
}
