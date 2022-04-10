#![no_std]
#![no_main]

extern crate panic_halt; // breakpoint on `rust_begin_unwind` to catches panics

use cortex_m_rt::entry;
use cortex_m::{iprintln};

use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::delay::Delay;

use stm32f3_discovery::switch_hal::{IntoSwitch, InputSwitch, OutputSwitch};

#[entry]
fn main() -> ! {
    // Configure peripherals
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_control_clock = device_periphs.RCC.constrain();

    let mut core_periphs = cortex_m::Peripherals::take().unwrap();
    let stim = &mut core_periphs.ITM.stim[0];
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_control_clock.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // LED
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_control_clock.ahb);
    let mut led =
            gpioe
            .pe12
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
            .into_active_high_switch();
    led.off().unwrap();

    // Start button
    let mut gpioa = device_periphs.GPIOA.split(&mut reset_control_clock.ahb);
    let button = gpioa.pa2.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr).into_active_low_switch();

    iprintln!(stim, "Begin!");

    loop {
        delay.delay_ms(50u16);
        match button.is_active() {
            Ok(true) => {
                iprintln!(stim, "on");
                led.on().ok();
            }
            Ok(false) => {
                iprintln!(stim, "off");
                led.off().ok();
            }
            Err(_) => {
                panic!("Failed to read button state");
            }
        }
    }
}
