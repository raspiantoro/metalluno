use crate::peripheral::Register;
use core::ptr::{read_volatile, write_volatile};

trait Port {
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

macro_rules! gpio {
    ($port_mod: ident, $pin_mod: ident, $dd: expr, $ddr: expr, $register: ty ,[$(($pin_struct: ident, $pin_field: ident, $masks: expr),)+]) => {
        pub mod $port_mod {
            use super::{$pin_mod, Port};
            use crate::peripheral::{Register, Address};

            pub struct Parts{
                $(
                    pub $pin_field: $pin_mod::$pin_struct,
                )+
            }

            impl Parts{
                pub fn new() -> Self{
                    Self{
                        $(
                            $pin_field: $pin_mod::$pin_struct::new(),
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
            use crate::peripheral::{Register, Address};

            $(
                pub struct $pin_struct{
                    dd: Register,
                    ddr: Register,
                    masks: u8
                }

                impl $pin_struct{
                    pub fn new() -> Self{
                        Self{
                            dd: $dd,
                            ddr: $ddr,
                            masks: $masks,
                        }
                    }

                    pub fn into_output(&self) -> &Self{
                        self.ddr.write(|addr| addr | self.masks);
                        self
                    }

                    pub fn into_input(&self) -> &Self{
                        self.ddr.write(|addr| addr & !self.masks);
                        self
                    }

                    pub fn set_high(&self) -> &Self{
                        self.dd.write(|addr| addr | self.masks);
                        self
                    }

                    pub fn set_low(&self) -> &Self{
                        self.dd.write(|addr| addr & !self.masks);
                        self
                    }
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
