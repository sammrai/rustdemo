// https://www.udemy.com/course/rust-os-system/


// mod vars;
// mod stack_healp;
// mod ownership;
// mod generics;
// mod lifetime;
// mod structs;
// mod enums;
// mod traits;
extern crate lib_demo;

mod errror_handling;
mod unit_test;

fn main() {
    // println!("Hello, world!");
    // stack_healp::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    errror_handling::run();
    // vars::run();
    // vars::sub_a::func_a();
    // vars::sub_b::func_b();
    lib_demo::print_random_number();
}