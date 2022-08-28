pub(crate) extern crate panic_halt;

use mcp49xx::{Command, Mcp49xx, MODE_0, Channel};
use stm32f4xx_hal::{
    pac::Peripherals,
    prelude::_fugit_RateExtU32,
    prelude::_stm32f4xx_hal_gpio_GpioExt,
    rcc::RccExt,
    spi::{NoMiso, Spi},
};

pub fn start() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();

    let gpioa = peripherals.GPIOA.split();

    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();

    let sck = gpioa.pa5.into_alternate();
    let mosi = gpioa.pa7.into_alternate().internal_pull_up(true);
    let mut chip_select = gpioa.pa1.into_push_pull_output();

    let mut spi = Spi::new(
        peripherals.SPI1,
        (sck, NoMiso {}, mosi),
        MODE_0,
        1.MHz(),
        &clocks,
    );

    chip_select.set_high();

    let mut dac = Mcp49xx::new_mcp4922(chip_select);
    let cmd = Command::default();
    let cmd = cmd.channel(Channel::Ch0).buffered();


    // let mut position = 0;
    let mut value: u16 = 0;
    loop {
        value += 100;
        if value > 4095{
            value =0;
        }
        dac.send(&mut spi, cmd.value(value)).unwrap();
    }
}
