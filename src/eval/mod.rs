use std::collections::HashMap;

use color_eyre::eyre;

use super::data::*;

mod condition;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VarResult {
    vars: HashMap<String, String>,
}

impl From<HashMap<String, String>> for VarResult {
    fn from(vars: HashMap<String, String>) -> Self {
        Self { vars }
    }
}

trait Eval {
    type Output;

    fn eval(self) -> Self::Output;
}

impl Eval for EnvConfig {
    type Output = eyre::Result<VarResult>;
    fn eval(self) -> Self::Output {
        Ok(self
            .env
            .into_iter()
            .map(|(var, cfg)| Ok((var, cfg.eval()?)))
            .filter_map(
                |res: Result<(String, Option<String>), eyre::Report>| match res {
                    Ok((_, None)) => None,
                    Ok((var, Some(cfg))) => Some(Ok((var, cfg))),
                    Err(err) => Some(Err(err)),
                },
            )
            .collect::<Result<HashMap<_, _>, _>>()?
            .into())
    }
}

impl Eval for VarConfig {
    type Output = eyre::Result<Option<String>>;
    fn eval(self) -> Self::Output {
        todo!()
    }
}
