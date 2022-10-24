#![no_main]
#![no_std]

// use panic_halt as _;
use cortex_m_rt::entry;

mod blink;
mod adc_read;
mod adc_double_read;
mod blink_w_speed_control;
mod spi_dac;
mod rtic_spi_dac;
// mod timer_blink;


// #[entry]
// fn main() -> ! {
//     // blink::start();
//     // adc_read::start();
//     // blink_w_speed_control::start();
//     // adc_double_read::start();
//     spi_dac::start();
//     // timer_blink::app::init();
// }
