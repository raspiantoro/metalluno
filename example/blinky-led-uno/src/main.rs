#![feature(llvm_asm)]
#![no_std]
#![no_main]

use avr_delay::delay_ms;
use uno::board::Board;

extern crate utils;

#[no_mangle]
pub extern "C" fn main() {
    let board = Board::take().unwrap();
    let led = board.led.take().unwrap();

    loop {
        led.turn_on();
        delay_ms(5000);

        led.turn_off();
        delay_ms(100);
    }
}
