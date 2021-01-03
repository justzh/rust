/// `fmt::Debug` hardly looks compact and clear, so it is often advantageous
/// to customize the output appearance. This is done by manually implementing
/// `fmt::Display`, which uses the `{}` print marker. Implementing it like this:

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct name `Structure` that contains an `i32`.
#[derive(Debug)]
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
	// This trait requires `fmt` with this exact signature.
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Write strictly the first element into the supplied output
		// stream: `f`. Returns `fmt::Result` which indeicates whether the
		// operation succeeded or failed. Note that `write!` uses syntax which
		// is very similar to `println!`.
		write!(f, "{}", self.0)
	}
}

fn main() {
	let structure = Structure(0);

	println!("Display: {}", structure);
	println!("Debug: {:?}", structure);
}