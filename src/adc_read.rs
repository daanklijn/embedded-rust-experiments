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
    let pa3 = gpioa.pa5.into_analog();

    loop {
        let sample = adc.convert(&pa3, SampleTime::Cycles_3);
        let millivolts = adc.sample_to_millivolts(sample);
        hprintln!("read adc: {}mV", millivolts);

        delay.delay_ms(delay_ms);
    }
}
