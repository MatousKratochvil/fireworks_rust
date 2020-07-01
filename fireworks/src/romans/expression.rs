use crate::romans::context::*;

pub trait Expression<'a> {
    fn interpret(&self, context: &mut Context) {
        if context.get_romans().len() == 0 {
            return;
        }

        if context.get_romans().starts_with(self.nine()) {
            context.add_number(9 * self.multiplier());
            context.split(self.nine().len());
        } else if context.get_romans().starts_with(self.five()) {
            context.add_number(5 * self.multiplier());
            context.split(self.five().len());
        } else if context.get_romans().starts_with(self.four()) {
            context.add_number(4 * self.multiplier());
            context.split(self.four().len());
        }
        while context.get_romans().starts_with(self.one()) {
            context.add_number(1 * self.multiplier());
            context.split(self.one().len());
        }
    }

    fn one(&self) -> &'a str;
    fn four(&self) -> &'a str;
    fn five(&self) -> &'a str;
    fn nine(&self) -> &'a str;
    fn multiplier(&self) -> u64;
}
