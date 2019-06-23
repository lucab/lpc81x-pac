#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `RXRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDYENR {
    #[doc = "No interrupt will be generated when receiver data is available."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    AN_INTERRUPT_WILL_BE,
}
impl RXRDYENR {
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
            RXRDYENR::NO_INTERRUPT_WILL_BE => false,
            RXRDYENR::AN_INTERRUPT_WILL_BE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXRDYENR {
        match value {
            false => RXRDYENR::NO_INTERRUPT_WILL_BE,
            true => RXRDYENR::AN_INTERRUPT_WILL_BE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_no_interrupt_will_be(&self) -> bool {
        *self == RXRDYENR::NO_INTERRUPT_WILL_BE
    }
    #[doc = "Checks if the value of the field is `AN_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_an_interrupt_will_be(&self) -> bool {
        *self == RXRDYENR::AN_INTERRUPT_WILL_BE
    }
}
#[doc = "Possible values of the field `TXRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDYENR {
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    AN_INTERRUPT_WILL_BE,
}
impl TXRDYENR {
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
            TXRDYENR::NO_INTERRUPT_WILL_BE => false,
            TXRDYENR::AN_INTERRUPT_WILL_BE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRDYENR {
        match value {
            false => TXRDYENR::NO_INTERRUPT_WILL_BE,
            true => TXRDYENR::AN_INTERRUPT_WILL_BE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_no_interrupt_will_be(&self) -> bool {
        *self == TXRDYENR::NO_INTERRUPT_WILL_BE
    }
    #[doc = "Checks if the value of the field is `AN_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_an_interrupt_will_be(&self) -> bool {
        *self == TXRDYENR::AN_INTERRUPT_WILL_BE
    }
}
#[doc = "Possible values of the field `RXOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVENR {
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    AN_INTERRUPT_WILL_BE,
}
impl RXOVENR {
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
            RXOVENR::NO_INTERRUPT_WILL_BE => false,
            RXOVENR::AN_INTERRUPT_WILL_BE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOVENR {
        match value {
            false => RXOVENR::NO_INTERRUPT_WILL_BE,
            true => RXOVENR::AN_INTERRUPT_WILL_BE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_no_interrupt_will_be(&self) -> bool {
        *self == RXOVENR::NO_INTERRUPT_WILL_BE
    }
    #[doc = "Checks if the value of the field is `AN_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_an_interrupt_will_be(&self) -> bool {
        *self == RXOVENR::AN_INTERRUPT_WILL_BE
    }
}
#[doc = "Possible values of the field `TXUREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXURENR {
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    AN_INTERRUPT_WILL_BE,
}
impl TXURENR {
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
            TXURENR::NO_INTERRUPT_WILL_BE => false,
            TXURENR::AN_INTERRUPT_WILL_BE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXURENR {
        match value {
            false => TXURENR::NO_INTERRUPT_WILL_BE,
            true => TXURENR::AN_INTERRUPT_WILL_BE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_no_interrupt_will_be(&self) -> bool {
        *self == TXURENR::NO_INTERRUPT_WILL_BE
    }
    #[doc = "Checks if the value of the field is `AN_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_an_interrupt_will_be(&self) -> bool {
        *self == TXURENR::AN_INTERRUPT_WILL_BE
    }
}
#[doc = "Possible values of the field `SSAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAENR {
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    AN_INTERRUPT_WILL_BE,
}
impl SSAENR {
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
            SSAENR::NO_INTERRUPT_WILL_BE => false,
            SSAENR::AN_INTERRUPT_WILL_BE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSAENR {
        match value {
            false => SSAENR::NO_INTERRUPT_WILL_BE,
            true => SSAENR::AN_INTERRUPT_WILL_BE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_no_interrupt_will_be(&self) -> bool {
        *self == SSAENR::NO_INTERRUPT_WILL_BE
    }
    #[doc = "Checks if the value of the field is `AN_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_an_interrupt_will_be(&self) -> bool {
        *self == SSAENR::AN_INTERRUPT_WILL_BE
    }
}
#[doc = "Possible values of the field `SSDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDENR {
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    AN_INTERRUPT_WILL_BE,
}
impl SSDENR {
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
            SSDENR::NO_INTERRUPT_WILL_BE => false,
            SSDENR::AN_INTERRUPT_WILL_BE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSDENR {
        match value {
            false => SSDENR::NO_INTERRUPT_WILL_BE,
            true => SSDENR::AN_INTERRUPT_WILL_BE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_no_interrupt_will_be(&self) -> bool {
        *self == SSDENR::NO_INTERRUPT_WILL_BE
    }
    #[doc = "Checks if the value of the field is `AN_INTERRUPT_WILL_BE`"]
    #[inline]
    pub fn is_an_interrupt_will_be(&self) -> bool {
        *self == SSDENR::AN_INTERRUPT_WILL_BE
    }
}
#[doc = "Values that can be written to the field `RXRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDYENW {
    #[doc = "No interrupt will be generated when receiver data is available."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    AN_INTERRUPT_WILL_BE,
}
impl RXRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXRDYENW::NO_INTERRUPT_WILL_BE => false,
            RXRDYENW::AN_INTERRUPT_WILL_BE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when receiver data is available."]
    #[inline]
    pub fn no_interrupt_will_be(self) -> &'a mut W {
        self.variant(RXRDYENW::NO_INTERRUPT_WILL_BE)
    }
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    #[inline]
    pub fn an_interrupt_will_be(self) -> &'a mut W {
        self.variant(RXRDYENW::AN_INTERRUPT_WILL_BE)
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
#[doc = "Values that can be written to the field `TXRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDYENW {
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    AN_INTERRUPT_WILL_BE,
}
impl TXRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRDYENW::NO_INTERRUPT_WILL_BE => false,
            TXRDYENW::AN_INTERRUPT_WILL_BE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    #[inline]
    pub fn no_interrupt_will_be(self) -> &'a mut W {
        self.variant(TXRDYENW::NO_INTERRUPT_WILL_BE)
    }
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    #[inline]
    pub fn an_interrupt_will_be(self) -> &'a mut W {
        self.variant(TXRDYENW::AN_INTERRUPT_WILL_BE)
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
#[doc = "Values that can be written to the field `RXOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVENW {
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    AN_INTERRUPT_WILL_BE,
}
impl RXOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOVENW::NO_INTERRUPT_WILL_BE => false,
            RXOVENW::AN_INTERRUPT_WILL_BE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    #[inline]
    pub fn no_interrupt_will_be(self) -> &'a mut W {
        self.variant(RXOVENW::NO_INTERRUPT_WILL_BE)
    }
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    #[inline]
    pub fn an_interrupt_will_be(self) -> &'a mut W {
        self.variant(RXOVENW::AN_INTERRUPT_WILL_BE)
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
#[doc = "Values that can be written to the field `TXUREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXURENW {
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    AN_INTERRUPT_WILL_BE,
}
impl TXURENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXURENW::NO_INTERRUPT_WILL_BE => false,
            TXURENW::AN_INTERRUPT_WILL_BE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXURENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXURENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXURENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    #[inline]
    pub fn no_interrupt_will_be(self) -> &'a mut W {
        self.variant(TXURENW::NO_INTERRUPT_WILL_BE)
    }
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    #[inline]
    pub fn an_interrupt_will_be(self) -> &'a mut W {
        self.variant(TXURENW::AN_INTERRUPT_WILL_BE)
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
#[doc = "Values that can be written to the field `SSAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAENW {
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    AN_INTERRUPT_WILL_BE,
}
impl SSAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSAENW::NO_INTERRUPT_WILL_BE => false,
            SSAENW::AN_INTERRUPT_WILL_BE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSAENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline]
    pub fn no_interrupt_will_be(self) -> &'a mut W {
        self.variant(SSAENW::NO_INTERRUPT_WILL_BE)
    }
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline]
    pub fn an_interrupt_will_be(self) -> &'a mut W {
        self.variant(SSAENW::AN_INTERRUPT_WILL_BE)
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
#[doc = "Values that can be written to the field `SSDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDENW {
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    NO_INTERRUPT_WILL_BE,
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    AN_INTERRUPT_WILL_BE,
}
impl SSDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSDENW::NO_INTERRUPT_WILL_BE => false,
            SSDENW::AN_INTERRUPT_WILL_BE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline]
    pub fn no_interrupt_will_be(self) -> &'a mut W {
        self.variant(SSDENW::NO_INTERRUPT_WILL_BE)
    }
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline]
    pub fn an_interrupt_will_be(self) -> &'a mut W {
        self.variant(SSDENW::AN_INTERRUPT_WILL_BE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline]
    pub fn rxrdyen(&self) -> RXRDYENR {
        RXRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline]
    pub fn txrdyen(&self) -> TXRDYENR {
        TXRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline]
    pub fn rxoven(&self) -> RXOVENR {
        RXOVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline]
    pub fn txuren(&self) -> TXURENR {
        TXURENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when one or more Slave Select is asserted."]
    #[inline]
    pub fn ssaen(&self) -> SSAENR {
        SSAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when all Slave Selects are deasserted."]
    #[inline]
    pub fn ssden(&self) -> SSDENR {
        SSDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline]
    pub fn rxrdyen(&mut self) -> _RXRDYENW {
        _RXRDYENW { w: self }
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline]
    pub fn txrdyen(&mut self) -> _TXRDYENW {
        _TXRDYENW { w: self }
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline]
    pub fn rxoven(&mut self) -> _RXOVENW {
        _RXOVENW { w: self }
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline]
    pub fn txuren(&mut self) -> _TXURENW {
        _TXURENW { w: self }
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when one or more Slave Select is asserted."]
    #[inline]
    pub fn ssaen(&mut self) -> _SSAENW {
        _SSAENW { w: self }
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when all Slave Selects are deasserted."]
    #[inline]
    pub fn ssden(&mut self) -> _SSDENW {
        _SSDENW { w: self }
    }
}
