#![no_std]
#![no_main]

use arduino_hal::Pins;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();                                        
    let pins: Pins = arduino_hal::pins!(dp);                                    
    let mut led_pins = [
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),

        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
    ];

    let mut x = 0;
    loop {
        if x == 255 {
            x = 0;
        } else {
            x += 1;
        }

        for (i, pin) in led_pins.iter_mut().enumerate() {
            if (x & (1 << i)) != 0 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }

        arduino_hal::delay_ms(500);
    }
}
