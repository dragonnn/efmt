#![no_main]
#![no_std]

use defmt::trace;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::pac::{CorePeripherals, DWT};

#[allow(unused)]
use tfmt;
#[allow(unused)]
use core::{fmt, convert::Infallible, ptr, fmt::Write};

struct PseudoRandom { 
    state: u64,
    inc: u64,
}

impl PseudoRandom {
    fn new() -> Self {
        PseudoRandom {
            state: 0x853c49e6748fea9b, 
            inc: 0xda3e39cb94b95bdb, 
        }
    }

    fn next(&mut self) -> u32 {
        let oldstate = self.state;

        // Advance internal state
        self.state = oldstate.wrapping_mul(6364136223846793005).wrapping_add(self.inc | 1);

        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted = ((oldstate >> 18) ^ oldstate) >> 27;
        let rot = (oldstate >> 59) as isize;
        return (xorshifted >> rot  | (xorshifted << ((-rot) & 31))) as u32;    
    }
}

#[entry]
fn main() -> ! {
    let mut cp = CorePeripherals::take().unwrap();
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();
    
    let mut u32_ = 7_u32;
    let mut f32_ = 0.0_f32;
    let mut random = PseudoRandom::new();

    for _ in 1..20 {
        let mut s = heapless::String::<100>::new();
        let before = DWT::cycle_count();

        //tfmt::uwrite!(&mut s, "{}", u32_).unwrap();
        //core::write!(&mut s, "{}", u32_).unwrap();

        //tfmt::uwrite!(&mut s, "{:^20}", u32_).unwrap();
        //core::write!(&mut s, "{:^20}", u32_).unwrap();

        //tfmt::uwrite!(&mut s, "{} {} {}", u32_ as u8, u32_ as u16, u32_).unwrap();
        core::write!(&mut s, "{} {} {}", u32_ as u8, u32_ as u16, u32_).unwrap();

        //tfmt::uwrite!(&mut s, "{:#x}", u32_).unwrap();
        //core::write!(&mut s, "{:#x}", u32_).unwrap();

        //tfmt::uwrite!(&mut s, "{:5.3}", f32_).unwrap();
        //core::write!(&mut s, "{:5.3}", f32_).unwrap();

        let after = DWT::cycle_count();
        let delta = after.wrapping_sub(before);

        trace!("{},{},{},{}", s.as_str(), delta, u32_, f32_);

        u32_ = random.next();
        f32_ = (u32_ as f32) * 1e-9;
    }
    loop {}
}
