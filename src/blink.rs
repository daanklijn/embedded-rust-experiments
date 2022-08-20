use cortex_m_semihosting::hprintln;

pub(crate) 

extern crate panic_halt;
use cortex_m;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use stm32f4xx_hal::{self, stm32, prelude::*};

pub fn start() -> ! {
    let peripherals = stm32::Peripherals::take().unwrap();

    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    // use pc13 for stm32f411
    let gpioc = peripherals.GPIOC.split();
    let gpioa = peripherals.GPIOA.split();

    let mut led1 = gpioc.pc15.into_push_pull_output();
    let mut led2 = gpioa.pa1.into_push_pull_output();
    let mut led3 = gpioa.pa4.into_push_pull_output();
    let mut led4 = gpioa.pa7.into_push_pull_output();

    // Reset and clock control
    let rcc = peripherals.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();

    // System Timer
    let mut delay = stm32f4xx_hal::delay::Delay::new(core_peripherals.SYST, clocks);

    let mut delay_ms: u32 = 200;
    let mut i = 1;


    loop {
        led1.set_high().unwrap();
        led2.set_high().unwrap();
        led3.set_high().unwrap();
        led4.set_high().unwrap();
        match i{
            1=>led1.set_low().unwrap(),
            2=>led2.set_low().unwrap(),
            3=>led3.set_low().unwrap(),
            4=>led4.set_low().unwrap(),
            _=>i=0
        }
        i+=1;

        delay.delay_ms(delay_ms);
        hprintln!("Blink!").unwrap();
    }
}
