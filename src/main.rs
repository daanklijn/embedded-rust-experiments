#![no_main]
#![no_std]

// use panic_halt as _;
use cortex_m_rt::entry;

mod blink;


#[entry]
fn main() -> ! {
    blink::start();
}