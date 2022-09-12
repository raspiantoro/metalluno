#![feature(llvm_asm)]
#![no_std]
#![no_main]

use atmega328p::hal::usart::USART0;
use avr_delay::delay_ms;

extern crate utils;

// Transmitter enable flag, bit 3 of UCSR0B
const TXEN0: u8 = 1 << 3;

// Character Size masks, to set bit 1 on USCR0C
const UCSZ00: u8 = 1 << 1;

// Character Size masks, to set bit 2 on USCR0C
const UCSZ01: u8 = 1 << 2;

// Character Size masks for USCR0C bit 2:1
// will be use with UCSZ02 on UCSR0B, to define character set
const FRAME_FORMAT: u8 = UCSZ00 | UCSZ01;

#[no_mangle]
pub extern "C" fn main() {
    let baud_rate = 9600u64;
    let freq = 16_000_000u64;
    let mut usart0 = USART0::new(baud_rate, freq);

    usart0.init(|ucsr| {
        ucsr.b.write(|addr| addr | TXEN0);
        ucsr.c.write(|addr| addr | FRAME_FORMAT);
    });

    let word = "Hello from METALLUNO-DIRECT!!!";

    loop {
        usart0.writeln(word);
        delay_ms(500);
    }
}
