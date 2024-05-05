#![no_std]
#![no_main]

// use bcm2835_lpa::{self as bcm2835, GPIO};
use core::panic::PanicInfo;

mod bsp;
use bsp::gpio::{BlinkSpeed, GPIO};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}

#[no_mangle]
#[link_section = ".text.main"]
pub extern "C" fn main() -> ! {
    GPIO::set_output(49);
    loop {
        // GPIO::act_blink(BlinkSpeed::FastBlink, 10);
        // GPIO::act_blink(BlinkSpeed::SlowBlink, 10);
        GPIO::set(49);
    }
}
