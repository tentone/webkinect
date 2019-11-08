/*
use std::io;

const DERP: f32 = 1e1;

fn factorial(n: i64) -> i64
{
	if n == 1
	{
		return 1;
	} else if n == 0 { return 1; }

	return n * factorial(n - 1);
}

fn main()
{
	let s = String::from("xixi");
	{
		let s = String::from("derp");
	}

	println!("{}", s);
	return;

	println!("Guess the number!");

	let x = 5;
	let y = 10;
	println!("x = {} and y = {}", x, y);

	println!("{}", DERP);

	let mut guess: String = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("{}", guess);

	let spaces = "   ";
	let _spaces_len: usize = spaces.len();

	let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

	let mut i: usize = 0;
	while i < months.len()
	{
		println!("{}", months[i]);
		i += 1;
	}

	let x = 5;
	let y = {
		let x = 3;
		x + 1
	};

	println!("The value of y is: {}, x is: {}", y, x);
	print!("{}", factorial(8));
}
*/
