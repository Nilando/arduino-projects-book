#![no_std]
#![no_main]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm};
use arduino_hal::{Adc, Pins};
use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let potentiometer = pins.a5.into_analog_input(&mut adc);
    // Important because this sets the bit in the DDR register!
    pins.d9.into_output();

    // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
    // - Each count increases the duty-cycle by 4us.
    // - Use OC1A which is connected to D9 of the Arduino Uno.
    let tc1 = dp.TC1;
    tc1.icr1.write(|w| w.bits(4999));
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

    loop {
        let pot_val = potentiometer.analog_read(&mut adc);
        let min_duty = 140.0;
        let max_duty = 620.0;
        let duty = ((max_duty - min_duty) * (pot_val as f32 / 1023.0)) + min_duty;
        ufmt::uwriteln!(&mut serial, "POT: {}", pot_val).unwrap_infallible();
        ufmt::uwriteln!(&mut serial, "DUTY: {}", duty as u16).unwrap_infallible();
        // 0 - 1023 => 140 - 620

        // 100 counts => 0.4ms
        // 700 counts => 2.8ms
        tc1.ocr1a.write(|w| w.bits(duty as u16));
        arduino_hal::delay_ms(20);
    }}
