// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        // `self.0' means the 0th part of Structure
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex(f32, f32);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}

impl fmt::Binary for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "beans B)")
    }
}

fn main() {
    let cnum = Complex(3.3, 7.2);
    println!("Display: {}", cnum);
    println!("Debug: {:?}", cnum);
    println!("Complex in binary: {:b}", cnum);
}
