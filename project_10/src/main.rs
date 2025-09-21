#![no_std]
#![no_main]


use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm};
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
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let pot_pin = pins.a0.into_analog_input(&mut adc);
    let direction_button = pins.d13;
    let power_button = pins.d12;

    let mut ctrl_pin1 = pins.d2.into_output();
    let mut ctrl_pin2 = pins.d3.into_output();
    let mut enable_pin = pins.d9.into_output().into_pwm(&timer1);

    enable_pin.enable();

    let mut motor_direction = true;
    let mut motor_on = false;

    enable_pin.set_duty(0);

    let mut prev_power_state = false;
    let mut prev_direction = true;
    loop {
        delay_ms(20);

        let power_state = power_button.is_high();
        let direction = direction_button.is_high();
        let speed = pot_pin.analog_read(&mut adc)/4;

        //ufmt::uwriteln!(&mut serial, "speed: {}", speed).unwrap_infallible();
        //ufmt::uwriteln!(&mut serial, "motor_on: {}", motor_on).unwrap_infallible();
        //ufmt::uwriteln!(&mut serial, "motor_dir: {}", motor_direction).unwrap_infallible();

        if power_state && (power_state != prev_power_state) {
            motor_on = !motor_on;
        }

        if direction && (direction != prev_direction) {
            motor_direction = !motor_direction;
        }

        if motor_direction {
            ctrl_pin1.set_high();
            ctrl_pin2.set_low();
        } else {
            ctrl_pin1.set_low();
            ctrl_pin2.set_high();
        }

        if motor_on {
            enable_pin.set_duty(speed as u8);
        } else {
            enable_pin.set_duty(0);
        }

        prev_power_state = power_state;
        prev_direction = direction;
    }
}
