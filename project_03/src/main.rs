#![no_std]
#![no_main]

use arduino_hal::{adc, Pins, pac::ADC};
use arduino_hal::{Adc, Peripherals};
use arduino_hal::port::mode;
use panic_halt as _;
use arduino_hal::prelude::*;

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

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = Adc::new(dp.ADC, Default::default());
    let temp_pin = pins.a0.into_analog_input(&mut adc);

    let mut x = 0u8;

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).unwrap_infallible();

    loop {
        let sensor_value = temp_pin.analog_read(&mut adc);
        let volts = (sensor_value as f32 / 1024.0) * 5.0;
        let temp = ((volts - 0.5) * 100.0) as usize;
        let x = temp;

        for (i, pin) in led_pins.iter_mut().enumerate() {
            if (x & (1 << i)) != 0 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }

        ufmt::uwriteln!(&mut serial, "TEMP: {}", temp).unwrap_infallible();

        arduino_hal::delay_ms(10000);
    }
}
