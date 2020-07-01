use crate::romans::expression::*;

pub struct OnesExpression {}
impl<'a> Expression<'a> for OnesExpression {
    fn one(&self) -> &'a str {
        "I"
    }
    fn four(&self) -> &'a str {
        "IV"
    }
    fn five(&self) -> &'a str {
        "V"
    }
    fn nine(&self) -> &'a str {
        "IX"
    }
    fn multiplier(&self) -> u64 {
        1
    }
}
