#![no_main]
#![no_std]

// use panic_halt as _;
use cortex_m_rt::entry;

mod blink;
mod adc_read;
mod blink_w_speed_control;


#[entry]
fn main() -> ! {
    // blink::start();
    // adc_read::start();
    blink_w_speed_control::start();
}