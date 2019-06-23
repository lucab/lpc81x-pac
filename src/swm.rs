#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS"]
    pub pinassign0: PINASSIGN0,
    #[doc = "0x04 - Pin assign register 1. Assign movable functions U0_SCLC, U1_TXD, U1_RXD"]
    pub pinassign1: PINASSIGN1,
    #[doc = "0x08 - Pin assign register 2. Assign movable functions U2_TXD, U2_RXD"]
    pub pinassign2: PINASSIGN2,
    #[doc = "0x0c - Pin assignregister 3. Assign movable function SPI0_SCK"]
    pub pinassign3: PINASSIGN3,
    #[doc = "0x10 - Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL, SPI1_SCK"]
    pub pinassign4: PINASSIGN4,
    #[doc = "0x14 - Pin assign register 5. Assign movable functions SPI1_MOSI, SPI1_MISO, SPI1_SSEL, CTIN_0"]
    pub pinassign5: PINASSIGN5,
    #[doc = "0x18 - Pin assign register 6. Assign movable functions CTIN_1, CTIN_2, CTIN_3, CTOUT_0"]
    pub pinassign6: PINASSIGN6,
    #[doc = "0x1c - Pin assign egister 7. Assign movable functions CTOUT_1, CTOUT_2, CTOUT_3, I2C_SDA"]
    pub pinassign7: PINASSIGN7,
    #[doc = "0x20 - Pin assign register 8. Assign movable functions I2C_SCL, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
    pub pinassign8: PINASSIGN8,
    _reserved0: [u8; 412usize],
    #[doc = "0x1c0 - Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP"]
    pub pinenable0: PINENABLE0,
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS"]
pub struct PINASSIGN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS"]
pub mod pinassign0;
#[doc = "Pin assign register 1. Assign movable functions U0_SCLC, U1_TXD, U1_RXD"]
pub struct PINASSIGN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 1. Assign movable functions U0_SCLC, U1_TXD, U1_RXD"]
pub mod pinassign1;
#[doc = "Pin assign register 2. Assign movable functions U2_TXD, U2_RXD"]
pub struct PINASSIGN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 2. Assign movable functions U2_TXD, U2_RXD"]
pub mod pinassign2;
#[doc = "Pin assignregister 3. Assign movable function SPI0_SCK"]
pub struct PINASSIGN3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assignregister 3. Assign movable function SPI0_SCK"]
pub mod pinassign3;
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL, SPI1_SCK"]
pub struct PINASSIGN4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL, SPI1_SCK"]
pub mod pinassign4;
#[doc = "Pin assign register 5. Assign movable functions SPI1_MOSI, SPI1_MISO, SPI1_SSEL, CTIN_0"]
pub struct PINASSIGN5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 5. Assign movable functions SPI1_MOSI, SPI1_MISO, SPI1_SSEL, CTIN_0"]
pub mod pinassign5;
#[doc = "Pin assign register 6. Assign movable functions CTIN_1, CTIN_2, CTIN_3, CTOUT_0"]
pub struct PINASSIGN6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 6. Assign movable functions CTIN_1, CTIN_2, CTIN_3, CTOUT_0"]
pub mod pinassign6;
#[doc = "Pin assign egister 7. Assign movable functions CTOUT_1, CTOUT_2, CTOUT_3, I2C_SDA"]
pub struct PINASSIGN7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign egister 7. Assign movable functions CTOUT_1, CTOUT_2, CTOUT_3, I2C_SDA"]
pub mod pinassign7;
#[doc = "Pin assign register 8. Assign movable functions I2C_SCL, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
pub struct PINASSIGN8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 8. Assign movable functions I2C_SCL, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
pub mod pinassign8;
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP"]
pub struct PINENABLE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP"]
pub mod pinenable0;
