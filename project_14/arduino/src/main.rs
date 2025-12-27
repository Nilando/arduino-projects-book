#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _; // halts on panic

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    loop {
        ufmt::uwriteln!(&mut serial, "0123456789").unwrap();

        arduino_hal::delay_ms(500);
    }
}
