#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    // Example: Use pin 2 as send, pin 3 as receive
    let mut send = pins.d4.into_output();
    let mut recv = pins.d2.into_output();
    let mut led = pins.d10.into_output();

    loop {
        send.set_low();
        recv.set_low();

        delay_ms(20);

        let recv_input = recv.into_floating_input();

        // ----- CHARGE & TIMING PHASE -----
        // Drive send HIGH to start charging the receive node through the resistor
        send.set_high();

        // Time how many loop iterations until recv reads HIGH
        let mut count: u32 = 0;
        let max_count: u32 = 200_000; // large timeout for 1M resistor
        while recv_input.is_low() && count < max_count {
            count = count.wrapping_add(1);
        }

        // Optional: set send LOW again to prepare for next cycle
        send.set_low();

        if count > 20 {
            led.set_high();
        } else {
            led.set_low();
        }
        // Report the raw count (proportional to capacitance)
        ufmt::uwriteln!(&mut serial, "cap_count: {}\r", count).ok();

        // Convert recv input back into an output for the next discharge
        recv = recv_input.into_output();

        // Short pause between readings
        delay_ms(50);
    }
}
