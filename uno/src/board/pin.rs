use atmega328p::hal::gpio::{
    pin_b::{PB0, PB1, PB2, PB3, PB4, PB5},
    pin_d::{PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7},
    Pin,
};

use super::D13_IS_TAKEN;

pub trait DPin {
    type PinType: atmega328p::hal::gpio::Pin;
    const PIN_VAL: Self::PinType;

    fn into_output(&self) {
        Self::PIN_VAL.into_output();
    }

    fn into_input(&self) {
        Self::PIN_VAL.into_input();
    }

    fn set_high(&self) {
        Self::PIN_VAL.set_high();
    }

    fn set_low(&self) {
        Self::PIN_VAL.set_low();
    }
}

pub struct Digital {
    pub d13: D13,
}

impl Digital {
    pub fn take() -> Option<Self> {
        if unsafe { super::DIGITAL_IS_TAKEN } {
            None
        } else {
            unsafe { super::DIGITAL_IS_TAKEN = true }
            Option::Some(Self { d13: D13 {} })
        }
    }

    fn steal() -> Option<Self> {
        if unsafe { super::DIGITAL_IS_TAKEN } {
            None
        } else {
            Option::Some(Self { d13: D13 {} })
        }
    }
}

pub struct Pins {
    pub digital: Digital,
}

impl Pins {
    pub fn take() -> Option<Self> {
        if unsafe { super::PINS_IS_TAKEN } {
            None
        } else {
            unsafe { super::PINS_IS_TAKEN = true }
            Option::Some(Self {
                digital: Digital::steal()?,
            })
        }
    }
}

macro_rules! pin {
    (Digital, [$(($struct_name: ident, $type: ty, $val: expr),)+]) => {
        $(
            pub struct $struct_name{}

            impl DPin for $struct_name{
                type PinType = $type;
                const PIN_VAL: Self::PinType = $val;
            }
        )+
    };
}

pin!(
    Digital,
    [
        (D0, PD0, PD0 {}),
        (D1, PD1, PD1 {}),
        (D2, PD2, PD2 {}),
        (D3, PD3, PD3 {}),
        (D4, PD4, PD4 {}),
        (D5, PD5, PD5 {}),
        (D6, PD6, PD6 {}),
        (D7, PD7, PD7 {}),
        (D8, PB0, PB0 {}),
        (D9, PB1, PB1 {}),
        (D10, PB2, PB2 {}),
        (D11, PB3, PB3 {}),
        (D12, PB4, PB4 {}),
        (D13, PB5, PB5 {}),
    ]
);

impl D13 {
    pub fn take() -> Option<Self> {
        if unsafe { D13_IS_TAKEN } {
            None
        } else {
            unsafe { D13_IS_TAKEN = true }
            Option::Some(Self {})
        }
    }
}
