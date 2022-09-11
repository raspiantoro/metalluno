use crate::peripheral::register::Register;

pub trait Pin {
    const DD: Register;
    const DDR: Register;
    const MASKS: u8;

    fn into_output(&self) -> &Self {
        Self::DDR.write(|addr| addr | Self::MASKS);
        self
    }

    fn into_input(&self) -> &Self {
        Self::DDR.write(|addr| addr & !Self::MASKS);
        self
    }

    fn set_high(&self) -> &Self {
        Self::DD.write(|addr| addr | Self::MASKS);
        self
    }

    fn set_low(&self) -> &Self {
        Self::DD.write(|addr| addr & !Self::MASKS);
        self
    }
}

macro_rules! gpio {
    ($port_mod: ident, $pin_mod: ident, $dd: expr, $ddr: expr, $register: ty ,[$(($pin_struct: ident, $pin_field: ident, $masks: expr),)+]) => {
        pub mod $port_mod {
            use super::$pin_mod;
            use crate::peripheral::register::{Register, Address};
            use crate::peripheral::port::Port;

            pub struct Parts{
                $(
                    pub $pin_field: $pin_mod::$pin_struct,
                )+
            }

            impl Parts{
                pub fn new() -> Self{
                    Self{
                        $(
                            $pin_field: $pin_mod::$pin_struct{},
                        )+
                    }
                }
            }

            impl Port for Parts{
                const DD: $register = $dd;
                const DDR: $register = $ddr;
            }



        }

        pub mod $pin_mod{
            use crate::peripheral::register::{Register, Address};
            use super::Pin;

            $(
                pub struct $pin_struct{}

                impl Pin for $pin_struct{
                    const DD: $register = $dd;
                    const DDR: $register = $ddr;
                    const MASKS: u8 = $masks;
                }
            )+
        }


    };
}

gpio!(
    port_b,
    pin_b,
    Register(0x25 as Address),
    Register(0x24 as Address),
    Register,
    [
        (PB0, pin0, (1 << 0)),
        (PB1, pin1, (1 << 1)),
        (PB2, pin2, (1 << 2)),
        (PB3, pin3, (1 << 3)),
        (PB4, pin4, (1 << 4)),
        (PB5, pin5, (1 << 5)),
        (PB6, pin6, (1 << 6)),
        (PB7, pin7, (1 << 7)),
    ]
);

gpio!(
    port_c,
    pin_c,
    Register(0x28 as Address),
    Register(0x27 as Address),
    Register,
    [
        (PC0, pin0, (1 << 0)),
        (PC1, pin1, (1 << 1)),
        (PC2, pin2, (1 << 2)),
        (PC3, pin3, (1 << 3)),
        (PC4, pin4, (1 << 4)),
        (PC5, pin5, (1 << 5)),
        (PC6, pin6, (1 << 6)),
    ]
);

gpio!(
    port_d,
    pin_d,
    Register(0x2B as Address),
    Register(0x2A as Address),
    Register,
    [
        (PD0, pin0, (1 << 0)),
        (PD1, pin1, (1 << 1)),
        (PD2, pin2, (1 << 2)),
        (PD3, pin3, (1 << 3)),
        (PD4, pin4, (1 << 4)),
        (PD5, pin5, (1 << 5)),
        (PD6, pin6, (1 << 6)),
        (PD7, pin7, (1 << 7)),
    ]
);
