#![no_std]
#![no_main]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm};
use arduino_hal::{Adc, Pins};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();                                        
    let pins: Pins = arduino_hal::pins!(dp);                                    
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = Adc::new(dp.ADC, Default::default());
    let mut red_led = pins.d11.into_output().into_pwm(&timer2);
    let mut blue_led = pins.d10.into_output().into_pwm(&timer1);
    let mut green_led = pins.d9.into_output().into_pwm(&timer1);
    let red_receptor = pins.a5.into_analog_input(&mut adc);
    let blue_receptor = pins.a4.into_analog_input(&mut adc);
    let green_receptor = pins.a3.into_analog_input(&mut adc);

    red_led.enable();
    blue_led.enable();
    green_led.enable();
    
    loop {
        // read raw values from the receptor pins
        let raw_red = red_receptor.analog_read(&mut adc);
        arduino_hal::delay_ms(5);
        let raw_blue = blue_receptor.analog_read(&mut adc);
        arduino_hal::delay_ms(5);
        let raw_green = green_receptor.analog_read(&mut adc);
        arduino_hal::delay_ms(5);

        ufmt::uwriteln!(&mut serial, "RED: {}", raw_red / 4).unwrap_infallible();
        ufmt::uwriteln!(&mut serial, "BLUE: {}", raw_blue / 4).unwrap_infallible();
        ufmt::uwriteln!(&mut serial, "GREEN: {}", raw_green / 4).unwrap_infallible();

        // convert raw values from 0-1023 => 0-255
        // PWM "write" to each of the led pins
        red_led.set_duty((raw_red / 4) as u8);
        blue_led.set_duty((raw_blue / 4) as u8);
        green_led.set_duty((raw_green / 4) as u8);

        arduino_hal::delay_ms(500);
    }
}
