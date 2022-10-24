#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;

// About to reach 8kHz audio-rate with this one...

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use mcp49xx::{Mcp49xx, Command, Channel, MODE_0, marker::{Resolution12Bit, DualChannel, Buffered}};
    use stm32f4xx_hal::{gpio::{PC13, Output, PushPull, OpenDrain, Pin, Alternate, NoPin}, spi::{NoMiso, Spi, Spi1}, pac::SPI1};
    use systick_monotonic::*;
    use stm32f4xx_hal::{pac::Peripherals, prelude::_stm32f4xx_hal_gpio_GpioExt, rcc::RccExt, prelude::_fugit_RateExtU32, time::MegaHertz, timer::{Delay, SysDelay, SysTimerExt}};

    const TICKS: u32 = 4000; 

    #[shared]
    struct Shared {}

    type SpiType = Spi<SPI1, (Pin<'A',5, Alternate<5>>, NoPin, Pin<'A',7, Alternate<5>>)>;

    #[local]
    struct Local {
        state: u16,
        spi: SpiType,
        dac: Mcp49xx<Pin<'A',1, Output>, SpiType, Resolution12Bit, DualChannel, Buffered>,
        cmd: Command
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<TICKS>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) { hprintln!("init!");
        // Setup LED
        let mut gpioc = cx.device.GPIOC.split();
        let led = gpioc
            .pc13
            .into_open_drain_output();

        let mono = Systick::new(cx.core.SYST, 32_000_000);
        let when = monotonics::now() + 200.millis();

        let gpioa = cx.device.GPIOA.split();

        let rcc = cx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
        let sck = gpioa.pa5.into_alternate();
        let mosi = gpioa.pa7.into_alternate().internal_pull_up(true);
        let mut chip_select = gpioa.pa1.into_push_pull_output();

        let mut spi = Spi::new(
            cx.device.SPI1,
            (sck, NoMiso {}, mosi),
            MODE_0,
            20.MHz(),
            &clocks,
        );

        chip_select.set_high();

        let mut dac = Mcp49xx::new_mcp4922(chip_select);
        let cmd = Command::default();
        let cmd = cmd.channel(Channel::Ch0).buffered();
        dac.send(&mut spi, cmd.value(100)).unwrap();

        blink::spawn_at(when, when).unwrap();

        hprintln!("done init!");
        (Shared {}, Local {state: 0, spi, dac, cmd}, init::Monotonics(mono))
    }

    #[task(local = [state, spi, dac, cmd])]
    fn blink(cx: blink::Context, instant: fugit::TimerInstantU64<TICKS> ) {
        // hprintln!("{:?} --------- {:?}",instant, cx.local.state);
        cx.local.dac.send(cx.local.spi, cx.local.cmd.value(*cx.local.state)).unwrap();
        *cx.local.state += 400;
        if(*cx.local.state > 4000){
            *cx.local.state = 0;
        }
        let next_instant = instant + fugit::Duration:: <u64,1,TICKS> ::from_ticks(1);
        blink::spawn_at(next_instant, next_instant).unwrap();
    }

}