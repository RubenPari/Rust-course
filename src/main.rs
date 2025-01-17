mod collatz_length;
mod fibonacci;

use collatz_length::collatz_length;
use fibonacci::fibonacci;

fn main() {
    // Exercise 1
    println!("{}", fibonacci(10));

    // Exercise 2
    println!("{}", collatz_length(11));
}
