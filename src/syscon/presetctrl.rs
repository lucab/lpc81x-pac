#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SPI0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_RST_NR {
    #[doc = "Assert the SPI0 reset."]
    ASSERT_THE_SPI0_RESE,
    #[doc = "Clear the SPI0 reset."]
    CLEAR_THE_SPI0_RESET,
}
impl SPI0_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SPI0_RST_NR::ASSERT_THE_SPI0_RESE => false,
            SPI0_RST_NR::CLEAR_THE_SPI0_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI0_RST_NR {
        match value {
            false => SPI0_RST_NR::ASSERT_THE_SPI0_RESE,
            true => SPI0_RST_NR::CLEAR_THE_SPI0_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_SPI0_RESE`"]
    #[inline]
    pub fn is_assert_the_spi0_rese(&self) -> bool {
        *self == SPI0_RST_NR::ASSERT_THE_SPI0_RESE
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_SPI0_RESET`"]
    #[inline]
    pub fn is_clear_the_spi0_reset(&self) -> bool {
        *self == SPI0_RST_NR::CLEAR_THE_SPI0_RESET
    }
}
#[doc = "Possible values of the field `SPI1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_RST_NR {
    #[doc = "Assert the SPI1 reset."]
    ASSERT_THE_SPI1_RESE,
    #[doc = "Clear the SPI1 reset."]
    CLEAR_THE_SPI1_RESET,
}
impl SPI1_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SPI1_RST_NR::ASSERT_THE_SPI1_RESE => false,
            SPI1_RST_NR::CLEAR_THE_SPI1_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI1_RST_NR {
        match value {
            false => SPI1_RST_NR::ASSERT_THE_SPI1_RESE,
            true => SPI1_RST_NR::CLEAR_THE_SPI1_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_SPI1_RESE`"]
    #[inline]
    pub fn is_assert_the_spi1_rese(&self) -> bool {
        *self == SPI1_RST_NR::ASSERT_THE_SPI1_RESE
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_SPI1_RESET`"]
    #[inline]
    pub fn is_clear_the_spi1_reset(&self) -> bool {
        *self == SPI1_RST_NR::CLEAR_THE_SPI1_RESET
    }
}
#[doc = "Possible values of the field `UARTFRG_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTFRG_RST_NR {
    #[doc = "Assert the UARTFRG reset."]
    ASSERT_THE_UARTFRG_R,
    #[doc = "Clear the UARTFRG reset."]
    CLEAR_THE_UARTFRG_RE,
}
impl UARTFRG_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UARTFRG_RST_NR::ASSERT_THE_UARTFRG_R => false,
            UARTFRG_RST_NR::CLEAR_THE_UARTFRG_RE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UARTFRG_RST_NR {
        match value {
            false => UARTFRG_RST_NR::ASSERT_THE_UARTFRG_R,
            true => UARTFRG_RST_NR::CLEAR_THE_UARTFRG_RE,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_UARTFRG_R`"]
    #[inline]
    pub fn is_assert_the_uartfrg_r(&self) -> bool {
        *self == UARTFRG_RST_NR::ASSERT_THE_UARTFRG_R
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_UARTFRG_RE`"]
    #[inline]
    pub fn is_clear_the_uartfrg_re(&self) -> bool {
        *self == UARTFRG_RST_NR::CLEAR_THE_UARTFRG_RE
    }
}
#[doc = "Possible values of the field `USART0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0_RST_NR {
    #[doc = "Assert the USART0 reset."]
    ASSERT_THE_USART0_RE,
    #[doc = "Clear the USART0 reset."]
    CLEAR_THE_USART0_RES,
}
impl USART0_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USART0_RST_NR::ASSERT_THE_USART0_RE => false,
            USART0_RST_NR::CLEAR_THE_USART0_RES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART0_RST_NR {
        match value {
            false => USART0_RST_NR::ASSERT_THE_USART0_RE,
            true => USART0_RST_NR::CLEAR_THE_USART0_RES,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_USART0_RE`"]
    #[inline]
    pub fn is_assert_the_usart0_re(&self) -> bool {
        *self == USART0_RST_NR::ASSERT_THE_USART0_RE
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_USART0_RES`"]
    #[inline]
    pub fn is_clear_the_usart0_res(&self) -> bool {
        *self == USART0_RST_NR::CLEAR_THE_USART0_RES
    }
}
#[doc = "Possible values of the field `UART1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_RST_NR {
    #[doc = "Assert the UART reset."]
    ASSERT_THE_UART_RESE,
    #[doc = "Clear the UART1 reset."]
    CLEAR_THE_UART1_RESE,
}
impl UART1_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UART1_RST_NR::ASSERT_THE_UART_RESE => false,
            UART1_RST_NR::CLEAR_THE_UART1_RESE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART1_RST_NR {
        match value {
            false => UART1_RST_NR::ASSERT_THE_UART_RESE,
            true => UART1_RST_NR::CLEAR_THE_UART1_RESE,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_UART_RESE`"]
    #[inline]
    pub fn is_assert_the_uart_rese(&self) -> bool {
        *self == UART1_RST_NR::ASSERT_THE_UART_RESE
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_UART1_RESE`"]
    #[inline]
    pub fn is_clear_the_uart1_rese(&self) -> bool {
        *self == UART1_RST_NR::CLEAR_THE_UART1_RESE
    }
}
#[doc = "Possible values of the field `UART2_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_RST_NR {
    #[doc = "Assert the UART2 reset."]
    ASSERT_THE_UART2_RES,
    #[doc = "Clear the UART2 reset."]
    CLEAR_THE_UART2_RESE,
}
impl UART2_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UART2_RST_NR::ASSERT_THE_UART2_RES => false,
            UART2_RST_NR::CLEAR_THE_UART2_RESE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART2_RST_NR {
        match value {
            false => UART2_RST_NR::ASSERT_THE_UART2_RES,
            true => UART2_RST_NR::CLEAR_THE_UART2_RESE,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_UART2_RES`"]
    #[inline]
    pub fn is_assert_the_uart2_res(&self) -> bool {
        *self == UART2_RST_NR::ASSERT_THE_UART2_RES
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_UART2_RESE`"]
    #[inline]
    pub fn is_clear_the_uart2_rese(&self) -> bool {
        *self == UART2_RST_NR::CLEAR_THE_UART2_RESE
    }
}
#[doc = "Possible values of the field `I2C_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_NR {
    #[doc = "Assert the I2C reset."]
    ASSERT_THE_I2C_RESET,
    #[doc = "Clear the I2C reset."]
    CLEAR_THE_I2C_RESET_,
}
impl I2C_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            I2C_RST_NR::ASSERT_THE_I2C_RESET => false,
            I2C_RST_NR::CLEAR_THE_I2C_RESET_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C_RST_NR {
        match value {
            false => I2C_RST_NR::ASSERT_THE_I2C_RESET,
            true => I2C_RST_NR::CLEAR_THE_I2C_RESET_,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_I2C_RESET`"]
    #[inline]
    pub fn is_assert_the_i2c_reset(&self) -> bool {
        *self == I2C_RST_NR::ASSERT_THE_I2C_RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_I2C_RESET_`"]
    #[inline]
    pub fn is_clear_the_i2c_reset_(&self) -> bool {
        *self == I2C_RST_NR::CLEAR_THE_I2C_RESET_
    }
}
#[doc = "Possible values of the field `MRT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RST_NR {
    #[doc = "Assert the MRT reset."]
    ASSERT_THE_MRT_RESET,
    #[doc = "Clear the MRT reset."]
    CLEAR_THE_MRT_RESET_,
}
impl MRT_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MRT_RST_NR::ASSERT_THE_MRT_RESET => false,
            MRT_RST_NR::CLEAR_THE_MRT_RESET_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRT_RST_NR {
        match value {
            false => MRT_RST_NR::ASSERT_THE_MRT_RESET,
            true => MRT_RST_NR::CLEAR_THE_MRT_RESET_,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_MRT_RESET`"]
    #[inline]
    pub fn is_assert_the_mrt_reset(&self) -> bool {
        *self == MRT_RST_NR::ASSERT_THE_MRT_RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_MRT_RESET_`"]
    #[inline]
    pub fn is_clear_the_mrt_reset_(&self) -> bool {
        *self == MRT_RST_NR::CLEAR_THE_MRT_RESET_
    }
}
#[doc = "Possible values of the field `SCT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RST_NR {
    #[doc = "Assert the SCT reset."]
    ASSERT_THE_SCT_RESET,
    #[doc = "Clear the SCT reset."]
    CLEAR_THE_SCT_RESET_,
}
impl SCT_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SCT_RST_NR::ASSERT_THE_SCT_RESET => false,
            SCT_RST_NR::CLEAR_THE_SCT_RESET_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCT_RST_NR {
        match value {
            false => SCT_RST_NR::ASSERT_THE_SCT_RESET,
            true => SCT_RST_NR::CLEAR_THE_SCT_RESET_,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_SCT_RESET`"]
    #[inline]
    pub fn is_assert_the_sct_reset(&self) -> bool {
        *self == SCT_RST_NR::ASSERT_THE_SCT_RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_SCT_RESET_`"]
    #[inline]
    pub fn is_clear_the_sct_reset_(&self) -> bool {
        *self == SCT_RST_NR::CLEAR_THE_SCT_RESET_
    }
}
#[doc = "Possible values of the field `WKT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_RST_NR {
    #[doc = "Assert the WKT reset."]
    ASSERT_THE_WKT_RESET,
    #[doc = "Clear the WKT reset."]
    CLEAR_THE_WKT_RESET_,
}
impl WKT_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WKT_RST_NR::ASSERT_THE_WKT_RESET => false,
            WKT_RST_NR::CLEAR_THE_WKT_RESET_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKT_RST_NR {
        match value {
            false => WKT_RST_NR::ASSERT_THE_WKT_RESET,
            true => WKT_RST_NR::CLEAR_THE_WKT_RESET_,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_WKT_RESET`"]
    #[inline]
    pub fn is_assert_the_wkt_reset(&self) -> bool {
        *self == WKT_RST_NR::ASSERT_THE_WKT_RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_WKT_RESET_`"]
    #[inline]
    pub fn is_clear_the_wkt_reset_(&self) -> bool {
        *self == WKT_RST_NR::CLEAR_THE_WKT_RESET_
    }
}
#[doc = "Possible values of the field `GPIO_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_RST_NR {
    #[doc = "Assert the GPIO reset."]
    ASSERT_THE_GPIO_RESE,
    #[doc = "Clear the GPIO reset."]
    CLEAR_THE_GPIO_RESET,
}
impl GPIO_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIO_RST_NR::ASSERT_THE_GPIO_RESE => false,
            GPIO_RST_NR::CLEAR_THE_GPIO_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_RST_NR {
        match value {
            false => GPIO_RST_NR::ASSERT_THE_GPIO_RESE,
            true => GPIO_RST_NR::CLEAR_THE_GPIO_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_GPIO_RESE`"]
    #[inline]
    pub fn is_assert_the_gpio_rese(&self) -> bool {
        *self == GPIO_RST_NR::ASSERT_THE_GPIO_RESE
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_GPIO_RESET`"]
    #[inline]
    pub fn is_clear_the_gpio_reset(&self) -> bool {
        *self == GPIO_RST_NR::CLEAR_THE_GPIO_RESET
    }
}
#[doc = "Possible values of the field `FLASH_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RST_NR {
    #[doc = "Assert the flash controller reset."]
    ASSERT_THE_FLASH_CON,
    #[doc = "Clear the flash controller reset."]
    CLEAR_THE_FLASH_CONT,
}
impl FLASH_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLASH_RST_NR::ASSERT_THE_FLASH_CON => false,
            FLASH_RST_NR::CLEAR_THE_FLASH_CONT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_RST_NR {
        match value {
            false => FLASH_RST_NR::ASSERT_THE_FLASH_CON,
            true => FLASH_RST_NR::CLEAR_THE_FLASH_CONT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_FLASH_CON`"]
    #[inline]
    pub fn is_assert_the_flash_con(&self) -> bool {
        *self == FLASH_RST_NR::ASSERT_THE_FLASH_CON
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_FLASH_CONT`"]
    #[inline]
    pub fn is_clear_the_flash_cont(&self) -> bool {
        *self == FLASH_RST_NR::CLEAR_THE_FLASH_CONT
    }
}
#[doc = "Possible values of the field `ACMP_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_RST_NR {
    #[doc = "Assert the analog comparator reset."]
    ASSERT_THE_ANALOG_CO,
    #[doc = "Clear the analog comparator controller reset."]
    CLEAR_THE_ANALOG_COM,
}
impl ACMP_RST_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMP_RST_NR::ASSERT_THE_ANALOG_CO => false,
            ACMP_RST_NR::CLEAR_THE_ANALOG_COM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_RST_NR {
        match value {
            false => ACMP_RST_NR::ASSERT_THE_ANALOG_CO,
            true => ACMP_RST_NR::CLEAR_THE_ANALOG_COM,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT_THE_ANALOG_CO`"]
    #[inline]
    pub fn is_assert_the_analog_co(&self) -> bool {
        *self == ACMP_RST_NR::ASSERT_THE_ANALOG_CO
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_ANALOG_COM`"]
    #[inline]
    pub fn is_clear_the_analog_com(&self) -> bool {
        *self == ACMP_RST_NR::CLEAR_THE_ANALOG_COM
    }
}
#[doc = "Values that can be written to the field `SPI0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_RST_NW {
    #[doc = "Assert the SPI0 reset."]
    ASSERT_THE_SPI0_RESE,
    #[doc = "Clear the SPI0 reset."]
    CLEAR_THE_SPI0_RESET,
}
impl SPI0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI0_RST_NW::ASSERT_THE_SPI0_RESE => false,
            SPI0_RST_NW::CLEAR_THE_SPI0_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the SPI0 reset."]
    #[inline]
    pub fn assert_the_spi0_rese(self) -> &'a mut W {
        self.variant(SPI0_RST_NW::ASSERT_THE_SPI0_RESE)
    }
    #[doc = "Clear the SPI0 reset."]
    #[inline]
    pub fn clear_the_spi0_reset(self) -> &'a mut W {
        self.variant(SPI0_RST_NW::CLEAR_THE_SPI0_RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_RST_NW {
    #[doc = "Assert the SPI1 reset."]
    ASSERT_THE_SPI1_RESE,
    #[doc = "Clear the SPI1 reset."]
    CLEAR_THE_SPI1_RESET,
}
impl SPI1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI1_RST_NW::ASSERT_THE_SPI1_RESE => false,
            SPI1_RST_NW::CLEAR_THE_SPI1_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the SPI1 reset."]
    #[inline]
    pub fn assert_the_spi1_rese(self) -> &'a mut W {
        self.variant(SPI1_RST_NW::ASSERT_THE_SPI1_RESE)
    }
    #[doc = "Clear the SPI1 reset."]
    #[inline]
    pub fn clear_the_spi1_reset(self) -> &'a mut W {
        self.variant(SPI1_RST_NW::CLEAR_THE_SPI1_RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UARTFRG_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTFRG_RST_NW {
    #[doc = "Assert the UARTFRG reset."]
    ASSERT_THE_UARTFRG_R,
    #[doc = "Clear the UARTFRG reset."]
    CLEAR_THE_UARTFRG_RE,
}
impl UARTFRG_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UARTFRG_RST_NW::ASSERT_THE_UARTFRG_R => false,
            UARTFRG_RST_NW::CLEAR_THE_UARTFRG_RE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UARTFRG_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UARTFRG_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UARTFRG_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UARTFRG reset."]
    #[inline]
    pub fn assert_the_uartfrg_r(self) -> &'a mut W {
        self.variant(UARTFRG_RST_NW::ASSERT_THE_UARTFRG_R)
    }
    #[doc = "Clear the UARTFRG reset."]
    #[inline]
    pub fn clear_the_uartfrg_re(self) -> &'a mut W {
        self.variant(UARTFRG_RST_NW::CLEAR_THE_UARTFRG_RE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0_RST_NW {
    #[doc = "Assert the USART0 reset."]
    ASSERT_THE_USART0_RE,
    #[doc = "Clear the USART0 reset."]
    CLEAR_THE_USART0_RES,
}
impl USART0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART0_RST_NW::ASSERT_THE_USART0_RE => false,
            USART0_RST_NW::CLEAR_THE_USART0_RES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the USART0 reset."]
    #[inline]
    pub fn assert_the_usart0_re(self) -> &'a mut W {
        self.variant(USART0_RST_NW::ASSERT_THE_USART0_RE)
    }
    #[doc = "Clear the USART0 reset."]
    #[inline]
    pub fn clear_the_usart0_res(self) -> &'a mut W {
        self.variant(USART0_RST_NW::CLEAR_THE_USART0_RES)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_RST_NW {
    #[doc = "Assert the UART reset."]
    ASSERT_THE_UART_RESE,
    #[doc = "Clear the UART1 reset."]
    CLEAR_THE_UART1_RESE,
}
impl UART1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART1_RST_NW::ASSERT_THE_UART_RESE => false,
            UART1_RST_NW::CLEAR_THE_UART1_RESE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UART reset."]
    #[inline]
    pub fn assert_the_uart_rese(self) -> &'a mut W {
        self.variant(UART1_RST_NW::ASSERT_THE_UART_RESE)
    }
    #[doc = "Clear the UART1 reset."]
    #[inline]
    pub fn clear_the_uart1_rese(self) -> &'a mut W {
        self.variant(UART1_RST_NW::CLEAR_THE_UART1_RESE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART2_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_RST_NW {
    #[doc = "Assert the UART2 reset."]
    ASSERT_THE_UART2_RES,
    #[doc = "Clear the UART2 reset."]
    CLEAR_THE_UART2_RESE,
}
impl UART2_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART2_RST_NW::ASSERT_THE_UART2_RES => false,
            UART2_RST_NW::CLEAR_THE_UART2_RESE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART2_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UART2_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART2_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UART2 reset."]
    #[inline]
    pub fn assert_the_uart2_res(self) -> &'a mut W {
        self.variant(UART2_RST_NW::ASSERT_THE_UART2_RES)
    }
    #[doc = "Clear the UART2 reset."]
    #[inline]
    pub fn clear_the_uart2_rese(self) -> &'a mut W {
        self.variant(UART2_RST_NW::CLEAR_THE_UART2_RESE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_NW {
    #[doc = "Assert the I2C reset."]
    ASSERT_THE_I2C_RESET,
    #[doc = "Clear the I2C reset."]
    CLEAR_THE_I2C_RESET_,
}
impl I2C_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C_RST_NW::ASSERT_THE_I2C_RESET => false,
            I2C_RST_NW::CLEAR_THE_I2C_RESET_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the I2C reset."]
    #[inline]
    pub fn assert_the_i2c_reset(self) -> &'a mut W {
        self.variant(I2C_RST_NW::ASSERT_THE_I2C_RESET)
    }
    #[doc = "Clear the I2C reset."]
    #[inline]
    pub fn clear_the_i2c_reset_(self) -> &'a mut W {
        self.variant(I2C_RST_NW::CLEAR_THE_I2C_RESET_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MRT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RST_NW {
    #[doc = "Assert the MRT reset."]
    ASSERT_THE_MRT_RESET,
    #[doc = "Clear the MRT reset."]
    CLEAR_THE_MRT_RESET_,
}
impl MRT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRT_RST_NW::ASSERT_THE_MRT_RESET => false,
            MRT_RST_NW::CLEAR_THE_MRT_RESET_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _MRT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the MRT reset."]
    #[inline]
    pub fn assert_the_mrt_reset(self) -> &'a mut W {
        self.variant(MRT_RST_NW::ASSERT_THE_MRT_RESET)
    }
    #[doc = "Clear the MRT reset."]
    #[inline]
    pub fn clear_the_mrt_reset_(self) -> &'a mut W {
        self.variant(MRT_RST_NW::CLEAR_THE_MRT_RESET_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RST_NW {
    #[doc = "Assert the SCT reset."]
    ASSERT_THE_SCT_RESET,
    #[doc = "Clear the SCT reset."]
    CLEAR_THE_SCT_RESET_,
}
impl SCT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT_RST_NW::ASSERT_THE_SCT_RESET => false,
            SCT_RST_NW::CLEAR_THE_SCT_RESET_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the SCT reset."]
    #[inline]
    pub fn assert_the_sct_reset(self) -> &'a mut W {
        self.variant(SCT_RST_NW::ASSERT_THE_SCT_RESET)
    }
    #[doc = "Clear the SCT reset."]
    #[inline]
    pub fn clear_the_sct_reset_(self) -> &'a mut W {
        self.variant(SCT_RST_NW::CLEAR_THE_SCT_RESET_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_RST_NW {
    #[doc = "Assert the WKT reset."]
    ASSERT_THE_WKT_RESET,
    #[doc = "Clear the WKT reset."]
    CLEAR_THE_WKT_RESET_,
}
impl WKT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKT_RST_NW::ASSERT_THE_WKT_RESET => false,
            WKT_RST_NW::CLEAR_THE_WKT_RESET_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _WKT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the WKT reset."]
    #[inline]
    pub fn assert_the_wkt_reset(self) -> &'a mut W {
        self.variant(WKT_RST_NW::ASSERT_THE_WKT_RESET)
    }
    #[doc = "Clear the WKT reset."]
    #[inline]
    pub fn clear_the_wkt_reset_(self) -> &'a mut W {
        self.variant(WKT_RST_NW::CLEAR_THE_WKT_RESET_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_RST_NW {
    #[doc = "Assert the GPIO reset."]
    ASSERT_THE_GPIO_RESE,
    #[doc = "Clear the GPIO reset."]
    CLEAR_THE_GPIO_RESET,
}
impl GPIO_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_RST_NW::ASSERT_THE_GPIO_RESE => false,
            GPIO_RST_NW::CLEAR_THE_GPIO_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the GPIO reset."]
    #[inline]
    pub fn assert_the_gpio_rese(self) -> &'a mut W {
        self.variant(GPIO_RST_NW::ASSERT_THE_GPIO_RESE)
    }
    #[doc = "Clear the GPIO reset."]
    #[inline]
    pub fn clear_the_gpio_reset(self) -> &'a mut W {
        self.variant(GPIO_RST_NW::CLEAR_THE_GPIO_RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLASH_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RST_NW {
    #[doc = "Assert the flash controller reset."]
    ASSERT_THE_FLASH_CON,
    #[doc = "Clear the flash controller reset."]
    CLEAR_THE_FLASH_CONT,
}
impl FLASH_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH_RST_NW::ASSERT_THE_FLASH_CON => false,
            FLASH_RST_NW::CLEAR_THE_FLASH_CONT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the flash controller reset."]
    #[inline]
    pub fn assert_the_flash_con(self) -> &'a mut W {
        self.variant(FLASH_RST_NW::ASSERT_THE_FLASH_CON)
    }
    #[doc = "Clear the flash controller reset."]
    #[inline]
    pub fn clear_the_flash_cont(self) -> &'a mut W {
        self.variant(FLASH_RST_NW::CLEAR_THE_FLASH_CONT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMP_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_RST_NW {
    #[doc = "Assert the analog comparator reset."]
    ASSERT_THE_ANALOG_CO,
    #[doc = "Clear the analog comparator controller reset."]
    CLEAR_THE_ANALOG_COM,
}
impl ACMP_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_RST_NW::ASSERT_THE_ANALOG_CO => false,
            ACMP_RST_NW::CLEAR_THE_ANALOG_COM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the analog comparator reset."]
    #[inline]
    pub fn assert_the_analog_co(self) -> &'a mut W {
        self.variant(ACMP_RST_NW::ASSERT_THE_ANALOG_CO)
    }
    #[doc = "Clear the analog comparator controller reset."]
    #[inline]
    pub fn clear_the_analog_com(self) -> &'a mut W {
        self.variant(ACMP_RST_NW::CLEAR_THE_ANALOG_COM)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SPI0 reset control"]
    #[inline]
    pub fn spi0_rst_n(&self) -> SPI0_RST_NR {
        SPI0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SPI1 reset control"]
    #[inline]
    pub fn spi1_rst_n(&self) -> SPI1_RST_NR {
        SPI1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - UART fractional baud rate generator (UARTFRG) reset control"]
    #[inline]
    pub fn uartfrg_rst_n(&self) -> UARTFRG_RST_NR {
        UARTFRG_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - USART0 reset control"]
    #[inline]
    pub fn usart0_rst_n(&self) -> USART0_RST_NR {
        USART0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - U1ART1 reset control"]
    #[inline]
    pub fn uart1_rst_n(&self) -> UART1_RST_NR {
        UART1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - UART2 reset control"]
    #[inline]
    pub fn uart2_rst_n(&self) -> UART2_RST_NR {
        UART2_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - I2C reset control"]
    #[inline]
    pub fn i2c_rst_n(&self) -> I2C_RST_NR {
        I2C_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Multi-rate timer (MRT) reset control"]
    #[inline]
    pub fn mrt_rst_n(&self) -> MRT_RST_NR {
        MRT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SCT reset control"]
    #[inline]
    pub fn sct_rst_n(&self) -> SCT_RST_NR {
        SCT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Self wake-up timer (WKT) reset control"]
    #[inline]
    pub fn wkt_rst_n(&self) -> WKT_RST_NR {
        WKT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - GPIO and GPIO pin interrupt reset control"]
    #[inline]
    pub fn gpio_rst_n(&self) -> GPIO_RST_NR {
        GPIO_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Flash controller reset control"]
    #[inline]
    pub fn flash_rst_n(&self) -> FLASH_RST_NR {
        FLASH_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Analog comparator reset control"]
    #[inline]
    pub fn acmp_rst_n(&self) -> ACMP_RST_NR {
        ACMP_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8191 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SPI0 reset control"]
    #[inline]
    pub fn spi0_rst_n(&mut self) -> _SPI0_RST_NW {
        _SPI0_RST_NW { w: self }
    }
    #[doc = "Bit 1 - SPI1 reset control"]
    #[inline]
    pub fn spi1_rst_n(&mut self) -> _SPI1_RST_NW {
        _SPI1_RST_NW { w: self }
    }
    #[doc = "Bit 2 - UART fractional baud rate generator (UARTFRG) reset control"]
    #[inline]
    pub fn uartfrg_rst_n(&mut self) -> _UARTFRG_RST_NW {
        _UARTFRG_RST_NW { w: self }
    }
    #[doc = "Bit 3 - USART0 reset control"]
    #[inline]
    pub fn usart0_rst_n(&mut self) -> _USART0_RST_NW {
        _USART0_RST_NW { w: self }
    }
    #[doc = "Bit 4 - U1ART1 reset control"]
    #[inline]
    pub fn uart1_rst_n(&mut self) -> _UART1_RST_NW {
        _UART1_RST_NW { w: self }
    }
    #[doc = "Bit 5 - UART2 reset control"]
    #[inline]
    pub fn uart2_rst_n(&mut self) -> _UART2_RST_NW {
        _UART2_RST_NW { w: self }
    }
    #[doc = "Bit 6 - I2C reset control"]
    #[inline]
    pub fn i2c_rst_n(&mut self) -> _I2C_RST_NW {
        _I2C_RST_NW { w: self }
    }
    #[doc = "Bit 7 - Multi-rate timer (MRT) reset control"]
    #[inline]
    pub fn mrt_rst_n(&mut self) -> _MRT_RST_NW {
        _MRT_RST_NW { w: self }
    }
    #[doc = "Bit 8 - SCT reset control"]
    #[inline]
    pub fn sct_rst_n(&mut self) -> _SCT_RST_NW {
        _SCT_RST_NW { w: self }
    }
    #[doc = "Bit 9 - Self wake-up timer (WKT) reset control"]
    #[inline]
    pub fn wkt_rst_n(&mut self) -> _WKT_RST_NW {
        _WKT_RST_NW { w: self }
    }
    #[doc = "Bit 10 - GPIO and GPIO pin interrupt reset control"]
    #[inline]
    pub fn gpio_rst_n(&mut self) -> _GPIO_RST_NW {
        _GPIO_RST_NW { w: self }
    }
    #[doc = "Bit 11 - Flash controller reset control"]
    #[inline]
    pub fn flash_rst_n(&mut self) -> _FLASH_RST_NW {
        _FLASH_RST_NW { w: self }
    }
    #[doc = "Bit 12 - Analog comparator reset control"]
    #[inline]
    pub fn acmp_rst_n(&mut self) -> _ACMP_RST_NW {
        _ACMP_RST_NW { w: self }
    }
}
