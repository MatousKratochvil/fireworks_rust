use crate::romans::expression::*;

pub struct TensExpression {}
impl<'a> Expression<'a> for TensExpression {
    fn one(&self) -> &'a str {
        "X"
    }
    fn four(&self) -> &'a str {
        "XL"
    }
    fn five(&self) -> &'a str {
        "L"
    }
    fn nine(&self) -> &'a str {
        "XC"
    }
    fn multiplier(&self) -> u64 {
        10
    }
}
