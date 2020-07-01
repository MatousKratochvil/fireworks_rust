use crate::romans::expression::*;

pub struct ThousandsExpression {}
impl<'a> Expression<'a> for ThousandsExpression {
    fn one(&self) -> &'a str {
        "M"
    }
    fn four(&self) -> &'a str {
        " "
    }
    fn five(&self) -> &'a str {
        " "
    }
    fn nine(&self) -> &'a str {
        " "
    }
    fn multiplier(&self) -> u64 {
        1000
    }
}
