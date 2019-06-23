#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration for pin PIO0_17"]
    pub pio0_17: PIO0_17,
    #[doc = "0x04 - I/O configuration for pin PIO0_13"]
    pub pio0_13: PIO0_13,
    #[doc = "0x08 - I/O configuration for pin PIO0_12"]
    pub pio0_12: PIO0_12,
    #[doc = "0x0c - I/O configuration for pin PIO0_5/RESET"]
    pub pio0_5: PIO0_5,
    #[doc = "0x10 - I/O configuration for pin PIO0_4"]
    pub pio0_4: PIO0_4,
    #[doc = "0x14 - I/O configuration for pin PIO0_3/SWCLK"]
    pub pio0_3: PIO0_3,
    #[doc = "0x18 - I/O configuration for pin PIO0_2/SWDIO"]
    pub pio0_2: PIO0_2,
    #[doc = "0x1c - I/O configuration for pin PIO0_11. This is the pin configuration for the true open-drain pin."]
    pub pio0_11: PIO0_11,
    #[doc = "0x20 - I/O configuration for pin PIO0_10. This is the pin configuration for the true open-drain pin."]
    pub pio0_10: PIO0_10,
    #[doc = "0x24 - I/O configuration for pin PIO0_16"]
    pub pio0_16: PIO0_16,
    #[doc = "0x28 - I/O configuration for pin PIO0_15"]
    pub pio0_15: PIO0_15,
    #[doc = "0x2c - I/O configuration for pin PIO0_1/ACMP_I1/CLKIN"]
    pub pio0_1: PIO0_1,
    _reserved0: [u8; 4usize],
    #[doc = "0x34 - I/O configuration for pin PIO0_9/XTALOUT"]
    pub pio0_9: PIO0_9,
    #[doc = "0x38 - I/O configuration for pin PIO0_8/XTALIN"]
    pub pio0_8: PIO0_8,
    #[doc = "0x3c - I/O configuration for pin PIO0_7"]
    pub pio0_7: PIO0_7,
    #[doc = "0x40 - I/O configuration for pin PIO0_6/VDDCMP"]
    pub pio0_6: PIO0_6,
    #[doc = "0x44 - I/O configuration for pin PIO0_0/ACMP_I0"]
    pub pio0_0: PIO0_0,
    #[doc = "0x48 - I/O configuration for pin PIO0_14"]
    pub pio0_14: PIO0_14,
}
#[doc = "I/O configuration for pin PIO0_17"]
pub struct PIO0_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_17"]
pub mod pio0_17;
#[doc = "I/O configuration for pin PIO0_13"]
pub struct PIO0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_13"]
pub mod pio0_13;
#[doc = "I/O configuration for pin PIO0_12"]
pub struct PIO0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_12"]
pub mod pio0_12;
#[doc = "I/O configuration for pin PIO0_5/RESET"]
pub struct PIO0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_5/RESET"]
pub mod pio0_5;
#[doc = "I/O configuration for pin PIO0_4"]
pub struct PIO0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_4"]
pub mod pio0_4;
#[doc = "I/O configuration for pin PIO0_3/SWCLK"]
pub struct PIO0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_3/SWCLK"]
pub mod pio0_3;
#[doc = "I/O configuration for pin PIO0_2/SWDIO"]
pub struct PIO0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_2/SWDIO"]
pub mod pio0_2;
#[doc = "I/O configuration for pin PIO0_11. This is the pin configuration for the true open-drain pin."]
pub struct PIO0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_11. This is the pin configuration for the true open-drain pin."]
pub mod pio0_11;
#[doc = "I/O configuration for pin PIO0_10. This is the pin configuration for the true open-drain pin."]
pub struct PIO0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_10. This is the pin configuration for the true open-drain pin."]
pub mod pio0_10;
#[doc = "I/O configuration for pin PIO0_16"]
pub struct PIO0_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_16"]
pub mod pio0_16;
#[doc = "I/O configuration for pin PIO0_15"]
pub struct PIO0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_15"]
pub mod pio0_15;
#[doc = "I/O configuration for pin PIO0_1/ACMP_I1/CLKIN"]
pub struct PIO0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_1/ACMP_I1/CLKIN"]
pub mod pio0_1;
#[doc = "I/O configuration for pin PIO0_9/XTALOUT"]
pub struct PIO0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_9/XTALOUT"]
pub mod pio0_9;
#[doc = "I/O configuration for pin PIO0_8/XTALIN"]
pub struct PIO0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_8/XTALIN"]
pub mod pio0_8;
#[doc = "I/O configuration for pin PIO0_7"]
pub struct PIO0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_7"]
pub mod pio0_7;
#[doc = "I/O configuration for pin PIO0_6/VDDCMP"]
pub struct PIO0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_6/VDDCMP"]
pub mod pio0_6;
#[doc = "I/O configuration for pin PIO0_0/ACMP_I0"]
pub struct PIO0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_0/ACMP_I0"]
pub mod pio0_0;
#[doc = "I/O configuration for pin PIO0_14"]
pub struct PIO0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_14"]
pub mod pio0_14;
