
use core::cell::RefCell;

use cortex_m_semihosting::hprintln;

pub(crate) 

extern crate panic_halt;
use cortex_m::{self, interrupt::Mutex};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use stm32f4xx_hal::{pac::{Peripherals, TIM2}, prelude::*, rcc::RccExt, prelude::_fugit_RateExtU32, time::MegaHertz, timer::{Delay, SysDelay, SysTimerExt, TimerExt, Event, CounterMs}, interrupt, gpio::{Pin, OpenDrain, Output}};

type LedType = Pin<'C', 13, Output<OpenDrain>>;

static G_TIM: Mutex<RefCell<Option<CounterMs<TIM2>>>> = Mutex::new(RefCell::new(None));
static G_LED: Mutex<RefCell<Option<LedType>>> = Mutex::new(RefCell::new(None));


pub fn start() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut syscfg = peripherals.SYSCFG.constrain();

    let gpioc = peripherals.GPIOC.split();
    let mut led1 = gpioc.pc13.into_open_drain_output();

    // Reset and clock control
    let rcc = peripherals.RCC.constrain();

    // let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // System Timer
    let mut delay = core_peripherals.SYST.delay(&clocks);

    let mut delay_ms: u32 = 200;
    let mut i = 1;
    let mut seconds: i32 = 1;

    let mut timer = peripherals.TIM2.counter_ms(&clocks);
    timer.start(2000.millis());
    timer.listen(Event::Update);

    unsafe{
        cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
    }

    cortex_m::interrupt::free(|cs| {
        G_TIM.borrow(cs).replace(Some(timer));
        G_LED.borrow(cs).replace(Some(led1));
    });

    loop {
        delay.delay_ms(delay_ms);
        hprintln!("No blink!");
    }
}

#[interrupt]
fn TIM2() {
    hprintln!("Blink from timer!");
    cortex_m::interrupt::free(|cs| {
        let mut led = G_LED.borrow(cs).borrow_mut();
        led.as_mut().unwrap().toggle();

        let mut timer = G_TIM.borrow(cs).borrow_mut();
        timer.as_mut().unwrap().clear_interrupt(Event::Update);
    });
}
