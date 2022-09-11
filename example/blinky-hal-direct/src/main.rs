#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate utils;

use atmel328p::hal::gpio::port_b;
use avr_delay::delay_ms;

#[no_mangle]
pub extern "C" fn main() {
    let portb = port_b::Parts::new();

    // set pin as output
    // arduino uno builtin led is using pin 5 on port b
    portb.pin5.into_output();

    loop {
        // set high
        portb.pin5.set_high();
        delay_ms(500);

        // set low
        portb.pin5.set_low();
        delay_ms(500);
    }
}
