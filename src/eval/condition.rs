use color_eyre::{
    eyre::{self, WrapErr},
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
            Condition::Host(_) => unimplemented!(),
            Condition::HostRe(_) => unimplemented!(),
            Condition::OsFamily(_) => unimplemented!(),
            Condition::Os(_) => unimplemented!(),
            Condition::Arch(_) => unimplemented!(),
            Condition::True => Ok(true),
            Condition::False => Ok(false),
        }
    }
}
