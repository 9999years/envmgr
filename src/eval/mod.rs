use std::collections::HashMap;

use color_eyre::eyre;

use super::data::*;

mod condition;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vars {
    vars: HashMap<String, String>,
}

impl From<HashMap<String, String>> for Vars {
    fn from(vars: HashMap<String, String>) -> Self {
        Self { vars }
    }
}

pub trait Eval {
    type Output;

    fn eval(self) -> Self::Output;
}

pub trait EvalWithEnv {
    type Output;
    type Environment;

    fn eval_with_env(self, env: &mut Self::Environment) -> Self::Output;
}

impl<E: Eval> EvalWithEnv for E {
    type Output = <Self as Eval>::Output;
    type Environment = ();
    fn eval_with_env(self, env: &mut Self::Environment) -> Self::Output {
        self.eval()
    }
}

impl Eval for EnvConfig {
    type Output = eyre::Result<Vars>;
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
        let mut components: Vec<String> = Vec::with_capacity(self.paths.len());
        for path in self.paths {
            if let Some(component) = path.eval()? {
                components.push(component);
            }
        }
        Ok(Some(components.join(&self.sep)))
    }
}

impl Eval for DirEntry {
    type Output = eyre::Result<Option<String>>;
    fn eval(self) -> Self::Output {
        Ok(if self.when.eval()? {
            Some(self.path.into())
        } else {
            None
        })
    }
}

impl EvalWithEnv for ShellPath {
    type Output;
    type Environment;
    fn eval_with_env(self, env: &mut Self::Environment) -> Self::Output {
        todo!()
    }
}
