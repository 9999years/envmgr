use super::de::{self, Condition};

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionEl<T> {
    pub value: T,
    pub when: Condition,
}

impl<T> From<T> for ConditionEl<T> {
    fn from(t: T) -> Self {
        ConditionEl {
            value: t,
            when: Default::default(),
        }
    }
}

impl<T> Into<Condition> for ConditionEl<T> {
    fn into(self) -> Condition {
        self.when
    }
}
