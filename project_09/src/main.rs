#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let button = pins.d13;
    let mut output = pins.d7.into_output();

    loop {
        if button.is_high() {
            output.set_high();
        } else {
            output.set_low();
        }
    }
}
