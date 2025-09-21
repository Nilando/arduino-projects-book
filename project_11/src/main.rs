#![no_std]
#![no_main]

use core::fmt::Write;
use arduino_hal::{delay_ms, Adc};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use panic_halt as _;
use ag_lcd::{LcdDisplay, Blink, Cursor};
use heapless::String;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[arduino_hal::entry]
fn main() -> ! {
     let peripherals = arduino_hal::Peripherals::take().unwrap();
     let pins = arduino_hal::pins!(peripherals);
     let delay = arduino_hal::Delay::new();
     let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);
     let mut adc = Adc::new(peripherals.ADC, Default::default());
    
     let rs = pins.d12.into_output().downgrade();
     let en = pins.d10.into_output().downgrade();
     let d4 = pins.d5.into_output().downgrade();
     let d5 = pins.d4.into_output().downgrade();
     let d6 = pins.d3.into_output().downgrade();
     let d7 = pins.d2.into_output().downgrade();

     let button = pins.d13;
    
     let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
         .with_half_bus(d4, d5, d6, d7)
         .with_blink(Blink::Off)
         .with_cursor(Cursor::Off)
         .with_lines(ag_lcd::Lines::TwoLines)
         .with_cols(16)
         .build();

    delay_ms(100);
    let mut floating_pin = pins.a0.into_analog_input(&mut adc);
    let noise = adc.read_blocking(&mut floating_pin);

    // Use noise as a seed
    ufmt::uwriteln!(&mut serial, "seed: {}", noise).unwrap_infallible();
    let mut rng = SmallRng::seed_from_u64(noise as u64);

    let mut state = MagicState::WaitingToPlay;
    loop {
        match state {
            MagicState::WaitingToPlay => {
                lcd.set_position(0, 0);
                delay_ms(10);
                lcd.print("Ask and I will");
                delay_ms(10);
                lcd.set_position(0, 1);
                delay_ms(10);
                lcd.print("tell. ");
                delay_ms(10);

                loop {
                    if button.is_high() {
                        lcd.clear();
                        delay_ms(10);
                        lcd.set_position(0, 0);
                        delay_ms(10);
                        lcd.print("One moment...");
                        delay_ms(2000);
                        state = MagicState::Shaking;
                        break;
                    }
                }
            }
            MagicState::Shaking => {
                for _ in 0..400 {
                    lcd.print("Shaking!!!!");
                    delay_ms(10);
                }
                state = MagicState::DisplayingAnswer;
            }
            MagicState::DisplayingAnswer => {
                let n: u8 = rng.gen_range(0..10);

                let msg = match n {
                    0 => "Yes.",
                    1 => "No.",
                    2 => "Perhaps.",
                    3 => "Ask again.",
                    4 => "Probably.",
                    5 => "Definitely not.",
                    6 => "Absolutely",
                    7 => "In time.",
                    8 => "Be patient.",
                    9 => "Unclear",
                    _ => {"wtf"}
                };
                lcd.clear();
                delay_ms(10);
                lcd.set_position(0, 0);
                delay_ms(10);
                lcd.print(msg);
                delay_ms(2000);
                state = MagicState::WaitingToPlay;
            }
        }
    }
}


enum MagicState {
    WaitingToPlay,
    Shaking,
    DisplayingAnswer
}
