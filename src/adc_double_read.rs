use cortex_m_semihosting::hprintln;

pub(crate) 

extern crate panic_halt;
use cortex_m;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use stm32f4xx_hal::{pac::Peripherals, prelude::_stm32f4xx_hal_gpio_GpioExt, rcc::RccExt, prelude::_fugit_RateExtU32, time::MegaHertz, timer::{Delay, SysDelay, SysTimerExt}, adc::{Adc, config::{AdcConfig, SampleTime}}};

pub fn start() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();

    let gpioa = peripherals.GPIOA.split();
    
    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
    let mut delay = core_peripherals.SYST.delay(&clocks);
    let mut delay_ms: u16 = 200;

    let mut adc = Adc::adc1(peripherals.ADC1, true, AdcConfig::default());
    let pa5 = gpioa.pa5.into_analog();
    let pa3 = gpioa.pa3.into_analog();

    loop {
        let sample3 = adc.convert(&pa3, SampleTime::Cycles_3);
        let millivolts3 = adc.sample_to_millivolts(sample3);
        let sample5 = adc.convert(&pa5, SampleTime::Cycles_3);
        let millivolts5 = adc.sample_to_millivolts(sample5);
        hprintln!("double read adc: {}mV, {}mV", millivolts3, millivolts5);

        delay.delay_ms(delay_ms);
    }
}
