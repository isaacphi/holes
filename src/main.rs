#![no_std]
#![no_main]

extern crate panic_halt; // breakpoint on `rust_begin_unwind` to catches panics

use cortex_m_rt::entry;
use cortex_m::{iprintln};

use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::{pwm::tim1, time::rate::*};

use stm32f3_discovery::switch_hal::{IntoSwitch, InputSwitch, OutputSwitch};

#[entry]
fn main() -> ! {
    // Configure peripherals
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut rcc = device_periphs.RCC.constrain();

    let mut core_periphs = cortex_m::Peripherals::take().unwrap();
    let stim = &mut core_periphs.ITM.stim[0];
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // PWM
    // Set the resolution of our duty cycle to 9000 and our period to
    // 50Hz.
    let mut gpioa = device_periphs.GPIOA.split(&mut rcc.ahb);
    let (mut c1_no_pins, _, _, _) = tim1(device_periphs.TIM1, 9000, 50.Hz(), &clocks);
    let pa8 = gpioa.pa8.into_af6_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let mut ch1 = c1_no_pins.output_to_pa8(pa8);
    ch1.set_duty(ch1.get_max_duty() / 2);
    ch1.enable();
    
    // LED
    let mut gpioe = device_periphs.GPIOE.split(&mut rcc.ahb);
    let mut led = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).into_active_high_switch();
    led.off().unwrap();

    // Start button
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
                panic!("Failed to read start button state");
            }
        }
    }
}
