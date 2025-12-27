#![no_std]
#![no_main]

use arduino_hal::{delay_ms, delay_us, Adc};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);
    let mut adc = Adc::new(peripherals.ADC, Default::default());

    // INPUTS
    let piezo = pins.a0.into_analog_input(&mut adc);
    let button = pins.d2;

    // OUTPUTS
    let mut red_led = pins.d13.into_output();
    let mut green_led = pins.d12.into_output();
    let mut yello_led = pins.d11.into_output();
    pins.d9.into_output();
    let mut lock_state = LockState::Locked;

    let tc1 = peripherals.TC1;
    tc1.icr1.write(|w| w.bits(4999));
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

    let knock_pattern: [bool; 8] = [
        true,
        false,
        true,
        false,
        true,
        true,
        false,
        false,
    ];
    loop {
        match lock_state {
            LockState::Locked => {
                let mut prev_knocks = [false; 8];

                red_led.set_high();
                yello_led.set_low();
                green_led.set_low();

                tc1.ocr1a.write(|w| w.bits(150 as u16));
                
                loop {
                    let mut max = 0;

                    red_led.set_high();
                    for _ in 0..500 {
                        let piezo_reading = piezo.analog_read(&mut adc);

                        max = max.max(piezo_reading);
                        delay_ms(1);
                    }

                    red_led.set_low();
                    delay_ms(500);

                    prev_knocks.rotate_left(1);
                    *prev_knocks.last_mut().unwrap() = max > 300;

                    ufmt::uwriteln!(&mut serial, "PREV: {:?}", prev_knocks).unwrap_infallible();
                    ufmt::uwriteln!(&mut serial, "PATT: {:?}", knock_pattern).unwrap_infallible();
                    if prev_knocks == knock_pattern {
                        lock_state = LockState::Unlocking;
                        break;
                    }
                }
            }
            LockState::Open => {
                red_led.set_low();
                yello_led.set_low();
                green_led.set_high();
                tc1.ocr1a.write(|w| w.bits(600 as u16));
                loop {
                    if button.is_high() {
                        green_led.set_low();
                        for _ in 0..15 {
                            yello_led.toggle();
                            delay_ms(100);
                        }
                        lock_state = LockState::Locked;
                        break;
                    }
                }
            }
            LockState::Unlocking => {
                red_led.set_low();
                yello_led.set_low();
                green_led.set_low();

                for _ in 0..15 {
                    yello_led.toggle();
                    delay_ms(100);
                }

                lock_state = LockState::Open;
            }
        }
    }
}

enum LockState {
    Locked,
    Open,
    Unlocking
}
