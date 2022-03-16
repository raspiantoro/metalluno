#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate baremetal_rust;

use avr_delay::delay_ms;
use baremetal_rust::hal::gpio::{pin, port};

#[no_mangle]
pub extern "C" fn main() {
    let port_b = port::PortB::new();

    let pin_2 = pin::P2::new();

    // set pin as output
    port_b.p3.into_output();
    port_b.p5.into_output();
    pin_2.into_output();

    loop {
        // set pin 5 high
        port_b.p3.set_high();
        port_b.p5.set_high();
        pin_2.set_high();
        delay_ms(5000);

        // set pin 5 low
        port_b.p3.set_low();
        port_b.p5.set_low();
        pin_2.set_low();
        delay_ms(500);
    }
}
