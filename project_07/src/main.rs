#![no_std]
#![no_main]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::Adc;
use panic_halt as _;

const FREQUENCIES: [u16; 4] = [
    262,
    294,
    330,
    349,
];

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = Adc::new(dp.ADC, Default::default());
    // Notice this is not a pwm pin. We generate frequency on via delay.
    let mut output_pin = pins.d8.into_output(); 
    let input_pin = pins.a0.into_analog_input(&mut adc);

    loop {
        let input = input_pin.analog_read(&mut adc);

        let freq_hz = 
        if 1020 <= input {
            FREQUENCIES[3]
        } else if 1000 <= input {
            FREQUENCIES[2]
        } else if 500 <= input {
            FREQUENCIES[1]
        } else if 5 <= input {
            FREQUENCIES[0]
        } else {
            output_pin.set_low();
            continue;
        };
             
        let half_period_us = 1_000_000 / (freq_hz * 2) as u32;

        for _ in 0..freq_hz / 16 {
            output_pin.toggle();
            arduino_hal::delay_us(half_period_us);
        }
    }
}
