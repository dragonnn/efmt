#![no_std]

use core::{convert::Infallible, ptr};

use panic_never as _;
use efmt::uWrite;

pub struct W;

impl uWrite for W {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
        s.as_bytes().iter().for_each(|b| unsafe {
            let _ = ptr::read_volatile(b);
        });

        Ok(())
    }
}
