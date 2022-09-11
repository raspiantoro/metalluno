use self::{led::LED, pin::Pins};

pub mod led;
pub mod pin;

static mut BOARD_IS_TAKEN: bool = false;
static mut PINS_IS_TAKEN: bool = false;
static mut DIGITAL_IS_TAKEN: bool = false;
static mut LED_IS_TAKEN: bool = false;
static mut D13_IS_TAKEN: bool = false;

pub struct Board {
    pub pins: Pins,
    pub led: LED,
}

impl Board {
    pub fn take() -> Option<Self> {
        if unsafe { BOARD_IS_TAKEN } {
            None
        } else {
            unsafe { BOARD_IS_TAKEN = true };
            Option::Some(Self {
                pins: steal::pins()?,
                led: LED::new(),
            })
        }
    }
}

mod steal {
    use super::pin::{Digital, Pins, D13};

    pub fn pins() -> Option<Pins> {
        if unsafe { super::PINS_IS_TAKEN } {
            None
        } else {
            Option::Some(Pins {
                digital: digital()?,
            })
        }
    }

    pub fn digital() -> Option<Digital> {
        if unsafe { super::DIGITAL_IS_TAKEN } {
            None
        } else {
            Option::Some(Digital { d13: D13 {} })
        }
    }
}
