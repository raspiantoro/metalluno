#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate metalluno;

use avr_delay::delay_ms;
use metalluno::hal::gpio::{pin_b, pin_d, port_b, port_d};

#[no_mangle]
pub extern "C" fn main() {
    let portb = port_b::Parts::new();
    let portd = port_d::Parts::new();

    let pb2 = pin_b::PB2::new();
    let pd7 = pin_d::PD7::new();

    // set pin as output
    portb.pin3.into_output();
    portb.pin5.into_output();
    portd.pin6.into_output();
    pb2.into_output();
    pd7.into_output();

    loop {
        // set high
        portb.pin3.set_high();
        portb.pin5.set_high();
        portd.pin6.set_high();
        pb2.set_high();
        pd7.set_high();
        delay_ms(5000);

        // set low
        portb.pin3.set_low();
        portb.pin5.set_low();
        portd.pin6.set_low();
        pb2.set_low();
        pd7.set_low();
        delay_ms(500);
    }
}
