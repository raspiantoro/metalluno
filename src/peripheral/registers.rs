use core::ptr::{read_volatile, write_volatile};

pub type Address = *mut u8;

pub struct Register(pub Address);

impl Register {
    pub fn write(&self, f: impl FnOnce(u8) -> u8) {
        // unsafe { write_volatile(self.address, read_volatile(self.address) | masks) }
        unsafe {
            let addr = read_volatile(self.0);
            write_volatile(self.0, f(addr));
        }
    }

    pub fn get(&self) -> *mut u8 {
        self.0
    }
}
