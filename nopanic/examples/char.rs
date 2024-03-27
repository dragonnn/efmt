#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(c: char) {
    uwrite!(&mut W, "{}", c).unwrap();
    uwrite!(&mut W, "{:20}", c).unwrap();
    uwrite!(&mut W, "{:<20}", c).unwrap();
    uwrite!(&mut W, "{:>20}", c).unwrap();
    uwrite!(&mut W, "{:^20}", c).unwrap();
    uwrite!(&mut W, "{:0^20}", c).unwrap();
    // TODO
    // uwrite!(&mut W, "{:?}", c).unwrap();
}
