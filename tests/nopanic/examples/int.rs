#![no_main]
#![no_std]

use sfmt::uwrite;

use common::W;

#[no_mangle]
fn _start(a: u8, b: u16, c: u32, d: usize, e: u64, f: u128) {
    uwrite!(&mut W, "{}", a).unwrap();
    uwrite!(&mut W, "{}", b).unwrap();
    uwrite!(&mut W, "{}", c).unwrap();
    uwrite!(&mut W, "{}", d).unwrap();
    uwrite!(&mut W, "{}", e).unwrap();
    uwrite!(&mut W, "{}", f).unwrap();

    uwrite!(&mut W, "{:20}", a).unwrap();
    uwrite!(&mut W, "{:20}", b).unwrap();
    uwrite!(&mut W, "{:20}", c).unwrap();
    uwrite!(&mut W, "{:20X}", d).unwrap();
    uwrite!(&mut W, "{:20}", e).unwrap();
    uwrite!(&mut W, "{:20}", f).unwrap();

    uwrite!(&mut W, "{:<20}", a).unwrap();
    uwrite!(&mut W, "{:<20}", b).unwrap();
    uwrite!(&mut W, "{:<20}", c).unwrap();
    uwrite!(&mut W, "{:<20x}", d).unwrap();
    uwrite!(&mut W, "{:<20}", e).unwrap();
    uwrite!(&mut W, "{:<20}", f).unwrap();

    uwrite!(&mut W, "{:>20}", a).unwrap();
    uwrite!(&mut W, "{:>20}", b).unwrap();
    uwrite!(&mut W, "{:>20}", c).unwrap();
    uwrite!(&mut W, "{:>20}", d).unwrap();
    uwrite!(&mut W, "{:>20X}", e).unwrap();
    uwrite!(&mut W, "{:>20}", f).unwrap();

    uwrite!(&mut W, "{:^20x}", a).unwrap();
    uwrite!(&mut W, "{:^20}", b).unwrap();
    uwrite!(&mut W, "{:^20}", c).unwrap();
    uwrite!(&mut W, "{:^20}", d).unwrap();
    uwrite!(&mut W, "{:^20}", e).unwrap();
    uwrite!(&mut W, "{:^20}", f).unwrap();

    uwrite!(&mut W, "{:0^20}", f).unwrap();
}

#[no_mangle]
fn _start2(a: i8, b: i16, c: i32, d: isize, e: i64, f: i128) {
    uwrite!(&mut W, "{}", a).unwrap();
    uwrite!(&mut W, "{}", b).unwrap();
    uwrite!(&mut W, "{}", c).unwrap();
    uwrite!(&mut W, "{}", d).unwrap();
    uwrite!(&mut W, "{}", e).unwrap();
    uwrite!(&mut W, "{}", f).unwrap();

    uwrite!(&mut W, "{:20}", a).unwrap();
    uwrite!(&mut W, "{:20}", b).unwrap();
    uwrite!(&mut W, "{:20}", c).unwrap();
    uwrite!(&mut W, "{:20}", d).unwrap();
    uwrite!(&mut W, "{:20}", e).unwrap();
    uwrite!(&mut W, "{:20}", f).unwrap();

    uwrite!(&mut W, "{:<20}", a).unwrap();
    uwrite!(&mut W, "{:<20}", b).unwrap();
    uwrite!(&mut W, "{:<20}", c).unwrap();
    uwrite!(&mut W, "{:<20}", d).unwrap();
    uwrite!(&mut W, "{:<20}", e).unwrap();
    uwrite!(&mut W, "{:<20}", f).unwrap();

    uwrite!(&mut W, "{:>20}", a).unwrap();
    uwrite!(&mut W, "{:>20}", b).unwrap();
    uwrite!(&mut W, "{:>20}", c).unwrap();
    uwrite!(&mut W, "{:>20}", d).unwrap();
    uwrite!(&mut W, "{:>20}", e).unwrap();
    uwrite!(&mut W, "{:>20}", f).unwrap();

    uwrite!(&mut W, "{:^20x}", a).unwrap();
    uwrite!(&mut W, "{:^20X}", b).unwrap();
    uwrite!(&mut W, "{:^20x}", c).unwrap();
    uwrite!(&mut W, "{:^20X}", d).unwrap();
    uwrite!(&mut W, "{:^20x}", e).unwrap();
    uwrite!(&mut W, "{:^20X}", f).unwrap();

    uwrite!(&mut W, "{:0^20}", f).unwrap();
}
