mod collatz_length;
mod fibonacci;
mod transpose_matrix;

use collatz_length::collatz_length;
use fibonacci::fibonacci;
use transpose_matrix::transpose_matrix;

fn main() {
    // Exercise 1
    println!("{}", fibonacci(10));

    // Exercise 2
    println!("{}", collatz_length(11));

    // Exercise 3
    println!(
        "{:?}",
        transpose_matrix([[101, 102, 103], [201, 202, 203], [301, 302, 303]])
    );
}
