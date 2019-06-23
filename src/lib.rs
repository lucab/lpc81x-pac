#![doc = "Peripheral access API for LPC800 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn SPI0();
    fn SPI1();
    fn UART0();
    fn UART1();
    fn UART2();
    fn I2C();
    fn SCT();
    fn MRT();
    fn CMP();
    fn WDT();
    fn BOD();
    fn FLASH_IRQ();
    fn WKT();
    fn PININT0();
    fn PININT1();
    fn PININT2();
    fn PININT3();
    fn PININT4();
    fn PININT5();
    fn PININT6();
    fn PININT7();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _reserved: 0 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C },
    Vector { _handler: SCT },
    Vector { _handler: MRT },
    Vector { _handler: CMP },
    Vector { _handler: WDT },
    Vector { _handler: BOD },
    Vector {
        _handler: FLASH_IRQ,
    },
    Vector { _handler: WKT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PININT0 },
    Vector { _handler: PININT1 },
    Vector { _handler: PININT2 },
    Vector { _handler: PININT3 },
    Vector { _handler: PININT4 },
    Vector { _handler: PININT5 },
    Vector { _handler: PININT6 },
    Vector { _handler: PININT7 },
];
#[doc = r" Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - SPI0"]
    SPI0,
    #[doc = "1 - SPI1"]
    SPI1,
    #[doc = "3 - UART0"]
    UART0,
    #[doc = "4 - UART1"]
    UART1,
    #[doc = "5 - UART2"]
    UART2,
    #[doc = "8 - I2C"]
    I2C,
    #[doc = "9 - SCT"]
    SCT,
    #[doc = "10 - MRT"]
    MRT,
    #[doc = "11 - CMP"]
    CMP,
    #[doc = "12 - WDT"]
    WDT,
    #[doc = "13 - BOD"]
    BOD,
    #[doc = "14 - FLASH_IRQ"]
    FLASH_IRQ,
    #[doc = "15 - WKT"]
    WKT,
    #[doc = "24 - PININT0"]
    PININT0,
    #[doc = "25 - PININT1"]
    PININT1,
    #[doc = "26 - PININT2"]
    PININT2,
    #[doc = "27 - PININT3"]
    PININT3,
    #[doc = "28 - PININT4"]
    PININT4,
    #[doc = "29 - PININT5"]
    PININT5,
    #[doc = "30 - PININT6"]
    PININT6,
    #[doc = "31 - PININT7"]
    PININT7,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SPI0 => 0,
            Interrupt::SPI1 => 1,
            Interrupt::UART0 => 3,
            Interrupt::UART1 => 4,
            Interrupt::UART2 => 5,
            Interrupt::I2C => 8,
            Interrupt::SCT => 9,
            Interrupt::MRT => 10,
            Interrupt::CMP => 11,
            Interrupt::WDT => 12,
            Interrupt::BOD => 13,
            Interrupt::FLASH_IRQ => 14,
            Interrupt::WKT => 15,
            Interrupt::PININT0 => 24,
            Interrupt::PININT1 => 25,
            Interrupt::PININT2 => 26,
            Interrupt::PININT3 => 27,
            Interrupt::PININT4 => 28,
            Interrupt::PININT5 => 29,
            Interrupt::PININT6 => 30,
            Interrupt::PININT7 => 31,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdt::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &wwdt::RegisterBlock {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "Multi-Rate Timer (MRT)"]
pub struct MRT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MRT {}
impl MRT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mrt::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for MRT {
    type Target = mrt::RegisterBlock;
    fn deref(&self) -> &mrt::RegisterBlock {
        unsafe { &*MRT::ptr() }
    }
}
#[doc = "Multi-Rate Timer (MRT)"]
pub mod mrt;
#[doc = "Self wake-up timer (WKT)"]
pub struct WKT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WKT {}
impl WKT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wkt::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for WKT {
    type Target = wkt::RegisterBlock;
    fn deref(&self) -> &wkt::RegisterBlock {
        unsafe { &*WKT::ptr() }
    }
}
#[doc = "Self wake-up timer (WKT)"]
pub mod wkt;
#[doc = "Switch matrix (SWM)"]
pub struct SWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWM {}
impl SWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swm::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for SWM {
    type Target = swm::RegisterBlock;
    fn deref(&self) -> &swm::RegisterBlock {
        unsafe { &*SWM::ptr() }
    }
}
#[doc = "Switch matrix (SWM)"]
pub mod swm;
#[doc = "Power Management Unit (PMU)"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    fn deref(&self) -> &pmu::RegisterBlock {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "Power Management Unit (PMU)"]
pub mod pmu;
#[doc = "Analog comparator"]
pub struct CMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP {}
impl CMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for CMP {
    type Target = cmp::RegisterBlock;
    fn deref(&self) -> &cmp::RegisterBlock {
        unsafe { &*CMP::ptr() }
    }
}
#[doc = "Analog comparator"]
pub mod cmp;
#[doc = "Flash controller"]
pub struct FLASHCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHCTRL {}
impl FLASHCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flashctrl::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for FLASHCTRL {
    type Target = flashctrl::RegisterBlock;
    fn deref(&self) -> &flashctrl::RegisterBlock {
        unsafe { &*FLASHCTRL::ptr() }
    }
}
#[doc = "Flash controller"]
pub mod flashctrl;
#[doc = "I/O configuration (IOCON)"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iocon::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    fn deref(&self) -> &iocon::RegisterBlock {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "I/O configuration (IOCON)"]
pub mod iocon;
#[doc = "System configuration (SYSCON)"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscon::RegisterBlock {
        1074036736 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &syscon::RegisterBlock {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "System configuration (SYSCON)"]
pub mod syscon;
#[doc = "I2C-bus interface"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c::RegisterBlock {
        1074069504 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    fn deref(&self) -> &i2c::RegisterBlock {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "I2C-bus interface"]
pub mod i2c;
#[doc = "SPI"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074102272 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "SPI"]
pub mod spi0;
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074118656 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "USART"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074167808 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074184192 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check (CRC) engine"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check (CRC) engine"]
pub mod crc;
#[doc = "State Configurable Timer (SCT)"]
pub struct SCT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT {}
impl SCT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sct::RegisterBlock {
        1342193664 as *const _
    }
}
impl Deref for SCT {
    type Target = sct::RegisterBlock;
    fn deref(&self) -> &sct::RegisterBlock {
        unsafe { &*SCT::ptr() }
    }
}
#[doc = "State Configurable Timer (SCT)"]
pub mod sct;
#[doc = "General Purpose I/O port (GPIO)"]
pub struct GPIO_PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORT {}
impl GPIO_PORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_port::RegisterBlock {
        2684354560 as *const _
    }
}
impl Deref for GPIO_PORT {
    type Target = gpio_port::RegisterBlock;
    fn deref(&self) -> &gpio_port::RegisterBlock {
        unsafe { &*GPIO_PORT::ptr() }
    }
}
#[doc = "General Purpose I/O port (GPIO)"]
pub mod gpio_port;
#[doc = "Pin interrupt and pattern match engine"]
pub struct PIN_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIN_INT {}
impl PIN_INT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pin_int::RegisterBlock {
        2684370944 as *const _
    }
}
impl Deref for PIN_INT {
    type Target = pin_int::RegisterBlock;
    fn deref(&self) -> &pin_int::RegisterBlock {
        unsafe { &*PIN_INT::ptr() }
    }
}
#[doc = "Pin interrupt and pattern match engine"]
pub mod pin_int;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "MRT"]
    pub MRT: MRT,
    #[doc = "WKT"]
    pub WKT: WKT,
    #[doc = "SWM"]
    pub SWM: SWM,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "CMP"]
    pub CMP: CMP,
    #[doc = "FLASHCTRL"]
    pub FLASHCTRL: FLASHCTRL,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "SCT"]
    pub SCT: SCT,
    #[doc = "GPIO_PORT"]
    pub GPIO_PORT: GPIO_PORT,
    #[doc = "PIN_INT"]
    pub PIN_INT: PIN_INT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            WWDT: WWDT {
                _marker: PhantomData,
            },
            MRT: MRT {
                _marker: PhantomData,
            },
            WKT: WKT {
                _marker: PhantomData,
            },
            SWM: SWM {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            CMP: CMP {
                _marker: PhantomData,
            },
            FLASHCTRL: FLASHCTRL {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            SCT: SCT {
                _marker: PhantomData,
            },
            GPIO_PORT: GPIO_PORT {
                _marker: PhantomData,
            },
            PIN_INT: PIN_INT {
                _marker: PhantomData,
            },
        }
    }
}
