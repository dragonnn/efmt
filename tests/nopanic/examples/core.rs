#![no_main]
#![no_std]

use sfmt::uwrite;

use common::W;

#[no_mangle]
fn _start(s: &str, c: char) {
    uwrite!(&mut W, "{}", s).unwrap();
    uwrite!(&mut W, "{:20}", s).unwrap();
    uwrite!(&mut W, "{:<20}", s).unwrap();
    uwrite!(&mut W, "{:>20}", s).unwrap();
    uwrite!(&mut W, "{:^20}", s).unwrap();
    uwrite!(&mut W, "{:0^20}", s).unwrap();

    uwrite!(&mut W, "{}", c).unwrap();
    uwrite!(&mut W, "{:20}", c).unwrap();
    uwrite!(&mut W, "{:<20}", c).unwrap();
    uwrite!(&mut W, "{:>20}", c).unwrap();
    uwrite!(&mut W, "{:^20}", c).unwrap();
    uwrite!(&mut W, "{:0^20}", c).unwrap();
}
