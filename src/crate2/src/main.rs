use crate1::print_me;

use crate3::{
    crate_3,
    useless,
};

mod module2;
use module2::module_2;
/// This crate calls a crate within this workspace
fn main() {
    println!("hello from crate 2");
    print_me("crate2");
    module_2();
    crate_3();
    useless::uselessness();
}
