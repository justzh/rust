// All types which want to use `std::fmt` formatting `traits` require an
// implementation to be printable. Automatic implmentations are only provided
// for types such as in the `std` library. All others *must* be manually
// implemented somehow.

// The `fmt::Debug` `trait` makes this very straightforward. *All* types can
// `derive` (automatically create) the `fmt:Debug` implementation. This is not
// true for `fmt::Display` which must be manually implemented.

// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintabe(i32);

// All `std` library types automatically are printable with {:?} too:

// Derive the `fmt:Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
	// Printing with `{:?}` is similar to with `{}`.
	println!("There are {:?} months in a year.", 12);
	println!("{1} {0} is the {actor} name.",
			 "Slater",
			 "Christian",
			 actor="actor's");

	// `Structure` is printable!
	println!("Now {:?} will print!", Structure(3));

	// The problem with `derive` is there is no control over how
	// the results look. What if I want this to just show a `7`?
	println!("Now {:?} will print!", Deep(Structure(7)));
}

