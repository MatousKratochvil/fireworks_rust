use std::io::*;
mod collection;
mod romans;

fn main() -> Result<()> {
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;

	let buff_str = buffer.as_str().trim();
	let mut context = romans::context::Context::new(buff_str);

	let expressions: Vec<Box<dyn romans::expression::Expression>> = vec![
		Box::new(romans::thousands::ThousandsExpression {}),
		Box::new(romans::hundreds::HundredsExpression {}),
		Box::new(romans::tens::TensExpression {}),
		Box::new(romans::ones::OnesExpression {}),
	];

	expressions
		.into_iter()
		.for_each(|expression| expression.interpret(&mut context));

	println!("{} = {}", buffer.as_str().trim(), context.print());

	let mut collection = collection::Collection::<isize>::new();

	for i in 0..100 {
		collection.attach(i);
	}

	collection.accept_mut(|&mut x| x * x);
	collection.accept(|&x| println!("{}", x));

	Ok(())
}
