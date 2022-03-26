#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// Need stm32f3xx_hal::prelude::* otherwise
//   'Error(corex-m-rt): The interrupt vectors are missing`
#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
#[warn(unused_imports)]

use cortex_m_rt::entry;
use cortex_m::{iprintln, Peripherals};

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();
    let stim = &mut p.ITM.stim[0];

    iprintln!(stim, "Begin");

    loop {}
}
