#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use stm32f4xx_hal::gpio::{PC13, Output, PushPull, OpenDrain};
    use systick_monotonic::*;
    use stm32f4xx_hal::{pac::Peripherals, prelude::_stm32f4xx_hal_gpio_GpioExt, rcc::RccExt, prelude::_fugit_RateExtU32, time::MegaHertz, timer::{Delay, SysDelay, SysTimerExt}};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        state: bool,
        led: PC13<Output<OpenDrain> > ,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init!");
        // Setup LED
        let mut gpioc = cx.device.GPIOC.split();
        let led = gpioc
            .pc13
            .into_open_drain_output();

        let mono = Systick::new(cx.core.SYST, 36_000_000);
        let when = monotonics::now() + 200.millis();
        blink::spawn_at(when, when).unwrap();

        hprintln!("done init!");
        (Shared {}, Local {state: false, led}, init::Monotonics(mono))
    }

    #[task(local = [state, led])]
    fn blink(cx: blink::Context, instant: fugit::TimerInstantU64<1000> ) {
        // hprintln!("foo(scheduled = {:?}",instant);
        if *cx.local.state {
            // hprintln!("blink off");
            cx.local.led.set_low();
            *cx.local.state = false;
        } else {
            // hprintln!("blink on");
            cx.local.led.set_high();
            *cx.local.state = true;
        }
        let next_instant = instant + 20.millis();
        blink::spawn_at(next_instant, next_instant).unwrap();
    }

}