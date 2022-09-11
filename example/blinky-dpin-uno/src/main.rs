#![feature(llvm_asm)]
#![no_std]
#![no_main]

use avr_delay::delay_ms;
use uno::board::{
    pin::{DPin, Digital},
    Board,
};

extern crate utils;

#[no_mangle]
pub extern "C" fn main() {
    let board = Board::take().unwrap();

    // set digital pin 13 as output pin
    // arduino uno builtin led is using digital pin 13
    board.pins.digital.d13.into_output();

    let digital = Digital::take().unwrap();

    loop {
        // set high
        digital.d13.set_high();
        delay_ms(5000);

        // set low
        digital.d13.set_low();
        delay_ms(100);
    }
}
