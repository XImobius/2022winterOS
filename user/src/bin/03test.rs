#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
#[no_mangle]
fn main() -> i32{
    let mut a = 1;
    while a != 1024{
        println!("{}", a);
        a = a*2;
    }
    println!("Test end.");
    0
}
