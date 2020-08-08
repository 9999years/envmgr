use std::collections::HashMap;

use super::data::*;
use super::eyre;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VarResult {
    vars: HashMap<String, String>,
}

trait Eval {
    type Output;

    fn eval(self) -> Self::Output;
}

impl Eval for EnvConfig {
    type Output = eyre::Result<VarResult>;
    fn eval(self) -> Self::Output {
        Ok(Default::default())
    }
}
