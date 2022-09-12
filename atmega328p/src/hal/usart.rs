use crate::peripheral::register::{Address, Register};

// Data register empty flag, bit 5 of UCSR0A
const UDRE0: u8 = 1 << 5;

// Receive Complete flag, bit 7 of UCSR0A
const RXC0: u8 = 1 << 7;

// USART baud rate register 0
struct UBRR0 {
    _high: Register,
    _low: Register,
}

impl UBRR0 {
    fn new(ubrr_val: u16) -> Self {
        let hreg = Register(0xC5 as Address);
        let lreg = Register(0xC4 as Address);

        hreg.write(|_| (ubrr_val >> 8) as u8);
        lreg.write(|_| ubrr_val as u8);

        Self {
            _high: hreg,
            _low: lreg,
        }
    }
}

// USART control and status register 0
pub struct UCSR0 {
    // UCSR0A
    pub a: Register,

    // UCSR0B
    pub b: Register,

    // UCSR0C
    pub c: Register,
}

impl UCSR0 {
    fn new() -> Self {
        Self {
            a: Register(0xC0 as Address),
            b: Register(0xC1 as Address),
            c: Register(0xC2 as Address),
        }
    }
}

type UDR0 = Register;

pub struct USART0 {
    _baud_rate_register: UBRR0,
    data_register: UDR0, // UDR0
    control_register: UCSR0,
}

impl USART0 {
    pub fn new(baud_rate: u64, freq: u64) -> Self {
        let ubrr_val = ((freq / (16 * baud_rate)) - 1u64) as u16;
        Self {
            _baud_rate_register: UBRR0::new(ubrr_val),
            data_register: Register(0xC6 as Address),
            control_register: UCSR0::new(),
        }
    }

    pub fn init(&mut self, f: impl FnOnce(&mut UCSR0)) {
        f(&mut self.control_register)
    }

    pub fn send(&mut self, c: char) {
        unsafe {
            while (core::ptr::read_volatile(self.control_register.a.get()) & UDRE0) == 0 {}
            self.data_register.write(|_| c as u8);
        }
    }

    pub fn write(&mut self, s: &str) {
        for c in s.chars() {
            self.send(c);
        }
    }

    pub fn writeln(&mut self, s: &str) {
        self.write(s);
        let new_line = "\n";
        self.write(new_line);
    }

    pub fn recv(&mut self) -> char {
        unsafe {
            while (core::ptr::read_volatile(self.control_register.a.get()) & RXC0) == 0 {}
            core::ptr::read_volatile(self.data_register.get() as *const char)
        }
    }

    pub fn readln(&mut self) -> [u8; 100] {
        let mut cs: [u8; 100] = [0; 100];
        let mut counter: usize = 0;

        loop {
            let c = self.recv() as u8;

            if c == 10 {
                break;
            }

            cs[counter] = c;
            counter += 1;
        }

        cs
    }
}
