use crate1::print_me;

mod module1;
use module1::say_hello;

fn main() {
    print_me("Word!");
    say_hello();
}