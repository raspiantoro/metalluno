use super::pin::{DPin, D13};

pub struct LED {
    pin: D13,
}

impl LED {
    pub fn new() -> Self {
        Self { pin: D13 {} }
    }

    pub fn take(&self) -> Option<Self> {
        if unsafe { super::LED_IS_TAKEN } {
            None
        } else {
            unsafe { super::LED_IS_TAKEN = true };

            let d13 = D13::take().unwrap();
            d13.into_output();

            Option::Some(Self { pin: d13 })
        }
    }

    pub fn turn_on(&self) {
        if unsafe { super::LED_IS_TAKEN } {
            self.pin.set_high();
        }
    }

    pub fn turn_off(&self) {
        if unsafe { super::LED_IS_TAKEN } {
            self.pin.set_low();
        }
    }
}
