use std::env;

use color_eyre::{
    eyre::{self, eyre, WrapErr},
    Section, SectionExt,
};
use tracing::instrument;

use super::Eval;
use crate::data::Condition;

impl Eval for Condition {
    type Output = eyre::Result<bool>;

    #[instrument]
    fn eval(self) -> Self::Output {
        match self {
            Condition::Or(conds) => {
                for res in conds.into_iter().map(Eval::eval) {
                    let evaluated = res.wrap_err("Evaluating 'or' conditional")?;
                    if evaluated {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            Condition::And(conds) => {
                for res in conds.into_iter().map(Eval::eval) {
                    let evaluated = res.wrap_err("Evaluating 'and' conditional")?;
                    if !evaluated {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Condition::Host(_) => Err(eyre!("eval for Condition::Host unimplemented")),
            Condition::HostRe(_) => Err(eyre!("eval for Condition::HostRe unimplemented")),
            Condition::OsFamily(_) => Err(eyre!("eval for Condition::OsFamily unimplemented")),
            Condition::Os(_) => Err(eyre!("eval for Condition::Os unimplemented")),
            Condition::Arch(_) => Err(eyre!("eval for Condition::Arch unimplemented")),
            Condition::Var(var) => match env::var(&var).map(|var| var.is_empty()) {
                Ok(is_empty) => Ok(is_empty),
                Err(env::VarError::NotPresent) => Ok(false),
                Err(err) => Err(err)
                    .wrap_err("Fetching environment variable")
                    .with_section(move || var.header("Variable name")),
            },
            Condition::True => Ok(true),
            Condition::False => Ok(false),
        }
    }
}
