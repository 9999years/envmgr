use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathsOrVals<T> {
    #[serde(alias = "dir", alias = "directory", alias = "file")]
    Path(T),
    #[serde(alias = "dirs", alias = "directories", alias = "files")]
    Paths(Vec<T>),
    #[serde(alias = "val")]
    Value(T),
    #[serde(alias = "vals")]
    Values(Vec<T>),
}

impl<T> PathsOrVals<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            PathsOrVals::Path(t) | PathsOrVals::Value(t) => vec![t],
            PathsOrVals::Paths(ts) | PathsOrVals::Values(ts) => ts,
        }
    }

    pub fn is_path(&self) -> bool {
        matches!(self, PathsOrVals::Path(_) | PathsOrVals::Paths(_))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, PathsOrVals::Value(_) | PathsOrVals::Values(_))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathOrVal<T> {
    #[serde(alias = "dir", alias = "directory", alias = "file")]
    Path(T),
    #[serde(alias = "val")]
    Value(T),
}

impl<T> PathOrVal<T> {
    pub fn into_inner(self) -> T {
        match self {
            PathOrVal::Path(t) | PathOrVal::Value(t) => t,
        }
    }

    pub fn is_path(&self) -> bool {
        matches!(self, PathOrVal::Path(_))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, PathOrVal::Value(_))
    }
}
