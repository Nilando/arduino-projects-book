#![no_std]
#![no_main]


use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::{delay_ms, Adc, Pins};
use panic_halt as _;
use arduino_hal::prelude::*;


const INTERVAL: u16 = i16::MAX as u16 / 5;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = Adc::new(dp.ADC, Default::default());
    // Notice this is not a pwm pin. We generate frequency on via delay.
    let input_pin = pins.d8;
    let mut l1 = pins.d2.into_output().downgrade();
    let mut l2 = pins.d3.into_output().downgrade();
    let mut l3 = pins.d4.into_output().downgrade();
    let mut l4 = pins.d5.into_output().downgrade();
    let mut l5: Pin<Output, Dynamic> = pins.d6.into_output().downgrade();


    // loop forever
    //   map the pval to the range of 500-1600hz
    //
    //   use this value to set the tone of the piezo
    let mut i = 0i32;
    loop {
        if input_pin.is_high() {
            if i < 0 {
                i = 0;
            }
            
            if i != i16::MAX.into() {
                i += 1;
            }
        } else {
            if i > 0 {
                i = 0;
            }

            if i != i16::MIN.into() {
                i -= 1;
            }
        }



        if i < 0 {
            display_leds([
                &mut l1,
                &mut l2,
                &mut l3,
                &mut l4,
                &mut l5,
            ], (i * -1) as u32);
        } else {
            display_leds([
                &mut l5,
                &mut l4,
                &mut l3,
                &mut l2,
                &mut l1,
            ], i as u32);
        }
    }
}


fn display_leds(pins: [&mut Pin<Output, Dynamic>; 5], v: u32) {
    for i in 0..5 {
        if v > (INTERVAL as u32 * (i + 1)) {
            pins[i as usize].set_high()
        } else {
            pins[i as usize].set_low()
        }
    }
}
