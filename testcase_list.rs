/// Implementing `fmt::Display` for a structure where the elements must be handled
/// sequentially is tricky. The problem is that each `write!` generates a
/// `fmt::Result`. Proper handling of this requires dealing with *all* the results.
/// Rust provides the `?`? operator for exactly this purpose.
///
/// Using `?` on `write!` looks like this:
