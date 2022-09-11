use super::register::Register;
use core::ptr::{read_volatile, write_volatile};

pub trait Port {
    // Port Data Register Address
    const DD: Register;

    // Port Data Direction Register Address
    const DDR: Register;

    fn get_dd(&self) -> Register {
        Self::DD
    }

    fn get_ddr(&self) -> Register {
        Self::DDR
    }

    // Updating Port Data Register state
    fn update_dd_state(&self, f: impl FnOnce(u8) -> u8) {
        unsafe {
            let addr = read_volatile(Self::DD.get());
            write_volatile(Self::DD.get(), f(addr));
        }
    }

    // Updating Port Data Direction Register state
    fn update_ddr_state(&self, f: impl FnOnce(u8) -> u8) {
        unsafe {
            let addr = read_volatile(Self::DDR.get());
            write_volatile(Self::DDR.get(), f(addr));
        }
    }
}
