#![no_std]
#![no_main]

extern crate panic_halt; // breakpoint on `rust_begin_unwind` to catches panics

use core::cell::RefCell;

use cortex_m_rt::entry;
use cortex_m::{iprintln, interrupt::Mutex, peripheral::NVIC};

// use stm32f3_discovery::stm32f3xx_hal::{
use stm32f3xx_hal::{
    prelude::*,
    pac,
    gpio,
    interrupt,
    gpio::{Output, Input, PushPull, Edge},
    delay::Delay,
    pwm::tim1,
    time::rate::*,
};
use max7219::*;

type LedPin = gpio::gpioe::PE12<Output<PushPull>>;
static LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));
type ButtonPin = gpio::gpioa::PA0<Input>;
static BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Configure peripherals
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut rcc = device_periphs.RCC.constrain();
    let mut syscfg = device_periphs.SYSCFG.constrain(&mut rcc.apb2);

    let mut core_periphs = cortex_m::Peripherals::take().unwrap();
    let stim = &mut core_periphs.ITM.stim[0];
    let mut flash = device_periphs.FLASH.constrain();
    let mut exti = device_periphs.EXTI;
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

    // MAX7219 display
    let mut gpioc = device_periphs.GPIOC.split(&mut rcc.ahb);
    let sck = gpioc.pc10.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    // let miso = gpioc.pc11.into_af_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);
    let cs = gpioc.pc13.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let data = gpioc.pc12.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let mut display = MAX7219::from_pins(1, data, cs, sck).unwrap();
    display.power_on().unwrap();
    display.write_str(0, b"01234567", 0b00100000).unwrap();
    display.set_intensity(0, 0x1).unwrap();

    // LED
    let mut gpioe = device_periphs.GPIOE.split(&mut rcc.ahb);
    // let mut led = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).into_active_high_switch();
    // led.off().unwrap();
    let mut led = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led.toggle().unwrap();
    // Move ownership of led to the global LED
    cortex_m::interrupt::free(|cs| *LED.borrow(cs).borrow_mut() = Some(led));

    // Sound
    // https://flowdsp.io/blog/stm32f3-02-dac-dma/
    // https://electronics.stackexchange.com/questions/405362/stm32f103-dma-with-pwm-repeating-values
    // https://stackoverflow.com/questions/63016570/how-to-make-dma-work-for-changing-the-duty-cycle-of-a-pwm-port-using-rust#63142402
    // https://github.com/antoinevg/stm32f3-rust-examples

    // Buttons / GPIO interrupt
    // let button = gpioa.pa2.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr).into_active_low_switch();
    let mut user_button = gpioa.pa0.into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr);
    syscfg.select_exti_interrupt_source(&user_button);
    user_button.trigger_on_edge(&mut exti, Edge::Rising);
    user_button.enable_interrupt(&mut exti);
    let interrupt_num = user_button.interrupt(); // hal::pac::Interrupt::EXTI0
    // Move ownership to global BUTTON
    cortex_m::interrupt::free(|cs| *BUTTON.borrow(cs).borrow_mut() = Some(user_button));

    unsafe {
        NVIC::unmask(interrupt_num)
    };

    iprintln!(stim, "Begin!");

    loop {
        delay.delay_ms(500u16);
        display.write_raw(0, &[1u8,2u8,3u8,4u8,5u8,6u8,7u8,8u8]).unwrap();
        
        // match button.is_active() {
        //     Ok(true) => {
        //         iprintln!(stim, "on");
        //         led.on().ok();
        //     }
        //     Ok(false) => {
        //         iprintln!(stim, "off");
        //         led.off().ok();
        //     }
        //     Err(_) => {
        //         panic!("Failed to read start button state");
        //     }
        // }
    }
}

// Button pressed interrupt
// The exti# maps to the pin number that is being used as an external interrupt.
// See page 295 of the stm32f303 reference manual for explanation:
// http://www.st.com/resource/en/reference_manual/dm00043574.pdf
//
// This may be called more than once per button press from the user since the button may not be debounced.
#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        LED.borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .toggle()
            .unwrap();
        BUTTON.borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt();
    })
}
