use crate::romans::expression::*;

pub struct HundredsExpression {}
impl<'a> Expression<'a> for HundredsExpression {
    fn one(&self) -> &'a str {
        "C"
    }
    fn four(&self) -> &'a str {
        "CD"
    }
    fn five(&self) -> &'a str {
        "D"
    }
    fn nine(&self) -> &'a str {
        "CM"
    }
    fn multiplier(&self) -> u64 {
        100
    }
}
