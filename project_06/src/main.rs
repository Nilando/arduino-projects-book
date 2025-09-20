#![no_std]
#![no_main]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::Adc;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = Adc::new(dp.ADC, Default::default());
    // Notice this is not a pwm pin. We generate frequency on via delay.
    let mut output_pin = pins.d8.into_output(); 
    let photo_receptor = pins.a0.into_analog_input(&mut adc);
    let mut max_pval = 0u16;
    let mut min_pval = !0u16;

    // loop for 5 seconds
    //  capture the lowest and highest values from the photoreceptor
    let mili_delay = 10;
    for _ in (0..5000).step_by(mili_delay) {
        let pval = photo_receptor.analog_read(&mut adc);
        if pval < min_pval {
            min_pval = pval;
        }

        if max_pval < pval {
            max_pval = pval;
        }

        arduino_hal::delay_ms(mili_delay as u32);

        ufmt::uwriteln!(&mut serial, "{} :: {}", max_pval, min_pval).unwrap_infallible();
    }

    // loop forever
    //   map the pval to the range of 500-1600hz
    //
    //   use this value to set the tone of the piezo
    let max_freq_hz = 1600;
    let min_freq_hz = 500;
    loop {
        let pval = photo_receptor.analog_read(&mut adc);
        let clamped_pval = pval.clamp(min_pval, max_pval);
        let normalized_pval = (clamped_pval - min_pval) / (max_pval - min_pval);
        let freq_hz = ((max_freq_hz - min_freq_hz) * normalized_pval) + min_freq_hz;
        let half_period_us = 1_000_000 / (freq_hz * 2) as u32;

        ufmt::uwriteln!(&mut serial, "freq: {}", freq_hz).unwrap_infallible();

        // freq_hz / 4 == times to play the frequency for 250milis
        // "* 2" b/c each iteration is a half period
        for _ in 0..(freq_hz / 4) * 2 {
            output_pin.toggle();
            arduino_hal::delay_us(half_period_us);
        }
    }
}
