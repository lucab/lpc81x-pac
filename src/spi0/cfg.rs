#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `Enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    DISABLED_THE_SPI_IS,
    #[doc = "Enabled. The SPI is enabled for operation."]
    ENABLED_THE_SPI_IS_,
}
impl ENABLER {
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
            ENABLER::DISABLED_THE_SPI_IS => false,
            ENABLER::ENABLED_THE_SPI_IS_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::DISABLED_THE_SPI_IS,
            true => ENABLER::ENABLED_THE_SPI_IS_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THE_SPI_IS`"]
    #[inline]
    pub fn is_disabled_the_spi_is(&self) -> bool {
        *self == ENABLER::DISABLED_THE_SPI_IS
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_SPI_IS_`"]
    #[inline]
    pub fn is_enabled_the_spi_is_(&self) -> bool {
        *self == ENABLER::ENABLED_THE_SPI_IS_
    }
}
#[doc = "Possible values of the field `Master`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTERR {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE_THE_SPI_,
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE_THE_SPI,
}
impl MASTERR {
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
            MASTERR::SLAVE_MODE_THE_SPI_ => false,
            MASTERR::MASTER_MODE_THE_SPI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASTERR {
        match value {
            false => MASTERR::SLAVE_MODE_THE_SPI_,
            true => MASTERR::MASTER_MODE_THE_SPI,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE_THE_SPI_`"]
    #[inline]
    pub fn is_slave_mode_the_spi_(&self) -> bool {
        *self == MASTERR::SLAVE_MODE_THE_SPI_
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE_THE_SPI`"]
    #[inline]
    pub fn is_master_mode_the_spi(&self) -> bool {
        *self == MASTERR::MASTER_MODE_THE_SPI
    }
}
#[doc = "Possible values of the field `LSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFR {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD_DATA_IS_TR,
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE_DATA_IS_TRA,
}
impl LSBFR {
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
            LSBFR::STANDARD_DATA_IS_TR => false,
            LSBFR::REVERSE_DATA_IS_TRA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSBFR {
        match value {
            false => LSBFR::STANDARD_DATA_IS_TR,
            true => LSBFR::REVERSE_DATA_IS_TRA,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_DATA_IS_TR`"]
    #[inline]
    pub fn is_standard_data_is_tr(&self) -> bool {
        *self == LSBFR::STANDARD_DATA_IS_TR
    }
    #[doc = "Checks if the value of the field is `REVERSE_DATA_IS_TRA`"]
    #[inline]
    pub fn is_reverse_data_is_tra(&self) -> bool {
        *self == LSBFR::REVERSE_DATA_IS_TRA
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the frame (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE_THE_SPI_CAPT,
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the frame (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE_THE_SPI_CHA,
}
impl CPHAR {
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
            CPHAR::CHANGE_THE_SPI_CAPT => false,
            CPHAR::CAPTURE_THE_SPI_CHA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::CHANGE_THE_SPI_CAPT,
            true => CPHAR::CAPTURE_THE_SPI_CHA,
        }
    }
    #[doc = "Checks if the value of the field is `CHANGE_THE_SPI_CAPT`"]
    #[inline]
    pub fn is_change_the_spi_capt(&self) -> bool {
        *self == CPHAR::CHANGE_THE_SPI_CAPT
    }
    #[doc = "Checks if the value of the field is `CAPTURE_THE_SPI_CHA`"]
    #[inline]
    pub fn is_capture_the_spi_cha(&self) -> bool {
        *self == CPHAR::CAPTURE_THE_SPI_CHA
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "Low. The rest state of the clock (between frames) is low."]
    LOW_THE_REST_STATE_,
    #[doc = "High. The rest state of the clock (between frames) is high."]
    HIGH_THE_REST_STATE,
}
impl CPOLR {
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
            CPOLR::LOW_THE_REST_STATE_ => false,
            CPOLR::HIGH_THE_REST_STATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::LOW_THE_REST_STATE_,
            true => CPOLR::HIGH_THE_REST_STATE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_THE_REST_STATE_`"]
    #[inline]
    pub fn is_low_the_rest_state_(&self) -> bool {
        *self == CPOLR::LOW_THE_REST_STATE_
    }
    #[doc = "Checks if the value of the field is `HIGH_THE_REST_STATE`"]
    #[inline]
    pub fn is_high_the_rest_state(&self) -> bool {
        *self == CPOLR::HIGH_THE_REST_STATE
    }
}
#[doc = "Possible values of the field `Loop`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
}
impl LOOPR {
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
            LOOPR::DISABLED_ => false,
            LOOPR::ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPR {
        match value {
            false => LOOPR::DISABLED_,
            true => LOOPR::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == LOOPR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == LOOPR::ENABLED_
    }
}
#[doc = "Possible values of the field `SPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOLR {
    #[doc = "Low. The SSEL pin is active low. The value in the SSEL fields of the RXDAT, TXDATCTL, and TXCTL registers related to SSEL is not inverted relative to the pins."]
    LOW_THE_SSEL_PIN_IS,
    #[doc = "High. The SSEL pin is active high. The value in the SSEL fields of the RXDAT, TXDATCTL, and TXCTL registers related to SSEL is inverted relative to the pins."]
    HIGH_THE_SSEL_PIN_I,
}
impl SPOLR {
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
            SPOLR::LOW_THE_SSEL_PIN_IS => false,
            SPOLR::HIGH_THE_SSEL_PIN_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPOLR {
        match value {
            false => SPOLR::LOW_THE_SSEL_PIN_IS,
            true => SPOLR::HIGH_THE_SSEL_PIN_I,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_THE_SSEL_PIN_IS`"]
    #[inline]
    pub fn is_low_the_ssel_pin_is(&self) -> bool {
        *self == SPOLR::LOW_THE_SSEL_PIN_IS
    }
    #[doc = "Checks if the value of the field is `HIGH_THE_SSEL_PIN_I`"]
    #[inline]
    pub fn is_high_the_ssel_pin_i(&self) -> bool {
        *self == SPOLR::HIGH_THE_SSEL_PIN_I
    }
}
#[doc = "Values that can be written to the field `Enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLEW {
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    DISABLED_THE_SPI_IS,
    #[doc = "Enabled. The SPI is enabled for operation."]
    ENABLED_THE_SPI_IS_,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::DISABLED_THE_SPI_IS => false,
            ENABLEW::ENABLED_THE_SPI_IS_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    #[inline]
    pub fn disabled_the_spi_is(self) -> &'a mut W {
        self.variant(ENABLEW::DISABLED_THE_SPI_IS)
    }
    #[doc = "Enabled. The SPI is enabled for operation."]
    #[inline]
    pub fn enabled_the_spi_is_(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLED_THE_SPI_IS_)
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
#[doc = "Values that can be written to the field `Master`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTERW {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE_THE_SPI_,
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE_THE_SPI,
}
impl MASTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASTERW::SLAVE_MODE_THE_SPI_ => false,
            MASTERW::MASTER_MODE_THE_SPI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASTERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    #[inline]
    pub fn slave_mode_the_spi_(self) -> &'a mut W {
        self.variant(MASTERW::SLAVE_MODE_THE_SPI_)
    }
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    #[inline]
    pub fn master_mode_the_spi(self) -> &'a mut W {
        self.variant(MASTERW::MASTER_MODE_THE_SPI)
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
#[doc = "Values that can be written to the field `LSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFW {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD_DATA_IS_TR,
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE_DATA_IS_TRA,
}
impl LSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBFW::STANDARD_DATA_IS_TR => false,
            LSBFW::REVERSE_DATA_IS_TRA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    #[inline]
    pub fn standard_data_is_tr(self) -> &'a mut W {
        self.variant(LSBFW::STANDARD_DATA_IS_TR)
    }
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    #[inline]
    pub fn reverse_data_is_tra(self) -> &'a mut W {
        self.variant(LSBFW::REVERSE_DATA_IS_TRA)
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
#[doc = "Values that can be written to the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAW {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the frame (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE_THE_SPI_CAPT,
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the frame (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE_THE_SPI_CHA,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::CHANGE_THE_SPI_CAPT => false,
            CPHAW::CAPTURE_THE_SPI_CHA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Change. The SPI captures serial data on the first clock transition of the frame (when the clock changes away from the rest state). Data is changed on the following edge."]
    #[inline]
    pub fn change_the_spi_capt(self) -> &'a mut W {
        self.variant(CPHAW::CHANGE_THE_SPI_CAPT)
    }
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the frame (when the clock changes away from the rest state). Data is captured on the following edge."]
    #[inline]
    pub fn capture_the_spi_cha(self) -> &'a mut W {
        self.variant(CPHAW::CAPTURE_THE_SPI_CHA)
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
#[doc = "Values that can be written to the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLW {
    #[doc = "Low. The rest state of the clock (between frames) is low."]
    LOW_THE_REST_STATE_,
    #[doc = "High. The rest state of the clock (between frames) is high."]
    HIGH_THE_REST_STATE,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::LOW_THE_REST_STATE_ => false,
            CPOLW::HIGH_THE_REST_STATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The rest state of the clock (between frames) is low."]
    #[inline]
    pub fn low_the_rest_state_(self) -> &'a mut W {
        self.variant(CPOLW::LOW_THE_REST_STATE_)
    }
    #[doc = "High. The rest state of the clock (between frames) is high."]
    #[inline]
    pub fn high_the_rest_state(self) -> &'a mut W {
        self.variant(CPOLW::HIGH_THE_REST_STATE)
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
#[doc = "Values that can be written to the field `Loop`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPW::DISABLED_ => false,
            LOOPW::ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(LOOPW::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(LOOPW::ENABLED_)
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
#[doc = "Values that can be written to the field `SPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOLW {
    #[doc = "Low. The SSEL pin is active low. The value in the SSEL fields of the RXDAT, TXDATCTL, and TXCTL registers related to SSEL is not inverted relative to the pins."]
    LOW_THE_SSEL_PIN_IS,
    #[doc = "High. The SSEL pin is active high. The value in the SSEL fields of the RXDAT, TXDATCTL, and TXCTL registers related to SSEL is inverted relative to the pins."]
    HIGH_THE_SSEL_PIN_I,
}
impl SPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOLW::LOW_THE_SSEL_PIN_IS => false,
            SPOLW::HIGH_THE_SSEL_PIN_I => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _SPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The SSEL pin is active low. The value in the SSEL fields of the RXDAT, TXDATCTL, and TXCTL registers related to SSEL is not inverted relative to the pins."]
    #[inline]
    pub fn low_the_ssel_pin_is(self) -> &'a mut W {
        self.variant(SPOLW::LOW_THE_SSEL_PIN_IS)
    }
    #[doc = "High. The SSEL pin is active high. The value in the SSEL fields of the RXDAT, TXDATCTL, and TXCTL registers related to SSEL is inverted relative to the pins."]
    #[inline]
    pub fn high_the_ssel_pin_i(self) -> &'a mut W {
        self.variant(SPOLW::HIGH_THE_SSEL_PIN_I)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SPI enable."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline]
    pub fn master(&self) -> MASTERR {
        MASTERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline]
    pub fn lsbf(&self) -> LSBFR {
        LSBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Clock Phase select. ."]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        LOOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SSEL Polarity select."]
    #[inline]
    pub fn spol(&self) -> SPOLR {
        SPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SPI enable."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline]
    pub fn master(&mut self) -> _MASTERW {
        _MASTERW { w: self }
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline]
    pub fn lsbf(&mut self) -> _LSBFW {
        _LSBFW { w: self }
    }
    #[doc = "Bit 4 - Clock Phase select. ."]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
    #[doc = "Bit 8 - SSEL Polarity select."]
    #[inline]
    pub fn spol(&mut self) -> _SPOLW {
        _SPOLW { w: self }
    }
}
