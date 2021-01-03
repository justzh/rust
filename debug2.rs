// So `fmt::Debug` definitely makes this printable but sacrifices some elegance.
// Rust also provides "pretty printing" with {:#?}.
#[derive(Debug)]
struct Person<'a> {
	name: &'a str,
	age: u8
}

fn main() {
	let name = "Peter";
	let age = 27;
	let peter = Person {name, age};

	// Pretty print
	println!("{:#?}", peter);
}

// One can manually implement `fmt::Display` to control the display.