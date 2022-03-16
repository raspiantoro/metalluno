use crate::peripheral::registers::Register;
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
    ($port_name: ident, $dd: expr, $ddr: expr, $register: ty ,[$(($pin_struct: ident, $pin_field: ident, $masks: expr),)+]) => {
        pub mod port {
            use super::{pin, Port};
            use crate::peripheral::registers::{Register, Address};

            pub struct $port_name{
                $(
                    pub $pin_field: pin::$pin_struct,
                )+
            }

            impl $port_name{
                pub fn new() -> Self{
                    Self{
                        $(
                            $pin_field: pin::$pin_struct::new(),
                        )+
                    }
                }
            }

            impl Port for $port_name{
                const DD: $register = $dd;
                const DDR: $register = $ddr;
            }



        }

        pub mod pin{
            use crate::peripheral::registers::{Register, Address};

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
    PortB,
    Register(0x25 as Address),
    Register(0x24 as Address),
    Register,
    [
        (P0, p0, (1 << 0)),
        (P1, p1, (1 << 1)),
        (P2, p2, (1 << 2)),
        (P3, p3, (1 << 3)),
        (P4, p4, (1 << 4)),
        (P5, p5, (1 << 5)),
        (P6, p6, (1 << 6)),
        (P7, p7, (1 << 7)),
    ]
);
