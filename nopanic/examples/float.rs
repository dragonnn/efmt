#![no_main]
#![no_std]

use ufmt::uwrite;

use common::W;

#[no_mangle]
fn _start(a: f32, b: f64) {
    uwrite!(&mut W, "{}", a).unwrap();
    uwrite!(&mut W, "{}", b).unwrap();
    uwrite!(&mut W, "{:10.2}", a).unwrap();
    uwrite!(&mut W, "{:10.2}", b).unwrap();
    uwrite!(&mut W, "{:<10.2}", a).unwrap();
    uwrite!(&mut W, "{:<10.2}", b).unwrap();
    uwrite!(&mut W, "{:>10.2}", a).unwrap();
    uwrite!(&mut W, "{:>10.2}", b).unwrap();
    uwrite!(&mut W, "{:^10.2}", a).unwrap();
    uwrite!(&mut W, "{:^10.2}", b).unwrap();
    uwrite!(&mut W, "{:0^10.2}", b).unwrap();
}
