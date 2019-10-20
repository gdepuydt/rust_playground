use crate1::print_me;
use crate1::window;
/// run using the command: 
/// ```
/// cargo run --example example1
/// ```
///    (from crate root)
fn main() {
    println!("print word from example1.rs");
    print_me("example1");
    window::println();
}
