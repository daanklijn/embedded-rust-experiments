use cortex_m_semihosting::hprintln;

pub(crate) 

extern crate panic_halt;
use cortex_m;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use stm32f4xx_hal::{pac::Peripherals, prelude::_stm32f4xx_hal_gpio_GpioExt, rcc::RccExt, prelude::_fugit_RateExtU32, time::MegaHertz, timer::{Delay, SysDelay, SysTimerExt}, adc::{Adc, config::{AdcConfig, SampleTime}}};

pub fn start() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioc = peripherals.GPIOC.split();
    let gpioa = peripherals.GPIOA.split();

    let mut led1 = gpioc.pc15.into_push_pull_output();
    let mut led2 = gpioa.pa1.into_push_pull_output();
    let mut led3 = gpioa.pa4.into_push_pull_output();
    let mut led4 = gpioa.pa7.into_push_pull_output();

    let rcc = peripherals.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();

    let mut delay = core_peripherals.SYST.delay(&clocks);

    let mut delay_ms: u16 = 200;

    let mut adc = Adc::adc1(peripherals.ADC1, true, AdcConfig::default());
    let pa5 = gpioa.pa5.into_analog();

    let mut i = 1;
    let mut a: i32 = 1;

    loop {
        led1.set_high();
        led2.set_high();
        led3.set_high();
        led4.set_high();
        match i{
            1=>led1.set_low(),
            2=>led2.set_low(),
            3=>led3.set_low(),
            4=>led4.set_low(),
            _=>a=a*-1
        }
        i+=a;
        let sample = adc.convert(&pa5, SampleTime::Cycles_3);
        let millivolts = adc.sample_to_millivolts(sample);

        // Shift the bits to make the range more usable.
        delay_ms = millivolts >> 4;

        delay.delay_ms(delay_ms);
    }
}
