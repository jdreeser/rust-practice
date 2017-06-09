// reference for getting used to rust
use std::fmt; // use a library

#[derive(Debug)]  // derive a trait implementation
struct Name(i32); // define a struct

impl fmt::Display for Name { // manually implement a trait
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // define a function
		write!(f, "{}", self.0) // call a macro, return value
	}
}

#[derive(Debug)]
struct Named { // define a struct with named member variables
	x: i32,    // define signed integer
	list: Vec<i32>, // define vector
}

fn main() { // define main function
	let name = Name(3); // declare struct variable
	let named = Named { x: 3, list: vec![1, 2, 3] };
	println!("{:?}", name);
	println!("{:?}", named);

	let mil = 1_000_000u32; // suffix type annotation
	let mil: u32 = 1_000_000; // type annotation
}