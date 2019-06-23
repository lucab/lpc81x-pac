#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `TXBRKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRKENR {
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
    #[doc = "Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTRL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINUOUS_BREAK_IS_,
}
impl TXBRKENR {
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
            TXBRKENR::NORMAL_OPERATION_ => false,
            TXBRKENR::CONTINUOUS_BREAK_IS_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXBRKENR {
        match value {
            false => TXBRKENR::NORMAL_OPERATION_,
            true => TXBRKENR::CONTINUOUS_BREAK_IS_,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline]
    pub fn is_normal_operation_(&self) -> bool {
        *self == TXBRKENR::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_BREAK_IS_`"]
    #[inline]
    pub fn is_continuous_break_is_(&self) -> bool {
        *self == TXBRKENR::CONTINUOUS_BREAK_IS_
    }
}
#[doc = "Possible values of the field `ADDRDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDETR {
    #[doc = "Enabled. The USART receiver is enabled for all incoming data."]
    ENABLED_THE_USART_R,
    #[doc = "Disabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    DISABLED_THE_USART_,
}
impl ADDRDETR {
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
            ADDRDETR::ENABLED_THE_USART_R => false,
            ADDRDETR::DISABLED_THE_USART_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRDETR {
        match value {
            false => ADDRDETR::ENABLED_THE_USART_R,
            true => ADDRDETR::DISABLED_THE_USART_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_USART_R`"]
    #[inline]
    pub fn is_enabled_the_usart_r(&self) -> bool {
        *self == ADDRDETR::ENABLED_THE_USART_R
    }
    #[doc = "Checks if the value of the field is `DISABLED_THE_USART_`"]
    #[inline]
    pub fn is_disabled_the_usart_(&self) -> bool {
        *self == ADDRDETR::DISABLED_THE_USART_
    }
}
#[doc = "Possible values of the field `TXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDISR {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    NOT_DISABLED_USART_,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED_USART_TRAN,
}
impl TXDISR {
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
            TXDISR::NOT_DISABLED_USART_ => false,
            TXDISR::DISABLED_USART_TRAN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDISR {
        match value {
            false => TXDISR::NOT_DISABLED_USART_,
            true => TXDISR::DISABLED_USART_TRAN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DISABLED_USART_`"]
    #[inline]
    pub fn is_not_disabled_usart_(&self) -> bool {
        *self == TXDISR::NOT_DISABLED_USART_
    }
    #[doc = "Checks if the value of the field is `DISABLED_USART_TRAN`"]
    #[inline]
    pub fn is_disabled_usart_tran(&self) -> bool {
        *self == TXDISR::DISABLED_USART_TRAN
    }
}
#[doc = "Possible values of the field `CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCR {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER_,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINUOUS_CLOCK_SC,
}
impl CCR {
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
            CCR::CLOCK_ON_CHARACTER_ => false,
            CCR::CONTINUOUS_CLOCK_SC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCR {
        match value {
            false => CCR::CLOCK_ON_CHARACTER_,
            true => CCR::CONTINUOUS_CLOCK_SC,
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_ON_CHARACTER_`"]
    #[inline]
    pub fn is_clock_on_character_(&self) -> bool {
        *self == CCR::CLOCK_ON_CHARACTER_
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_CLOCK_SC`"]
    #[inline]
    pub fn is_continuous_clock_sc(&self) -> bool {
        *self == CCR::CONTINUOUS_CLOCK_SC
    }
}
#[doc = "Possible values of the field `CLRCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRCCR {
    #[doc = "No affect on the CC bit."]
    NO_AFFECT_ON_THE_CC_,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR_THE_CC_B,
}
impl CLRCCR {
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
            CLRCCR::NO_AFFECT_ON_THE_CC_ => false,
            CLRCCR::AUTO_CLEAR_THE_CC_B => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLRCCR {
        match value {
            false => CLRCCR::NO_AFFECT_ON_THE_CC_,
            true => CLRCCR::AUTO_CLEAR_THE_CC_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_AFFECT_ON_THE_CC_`"]
    #[inline]
    pub fn is_no_affect_on_the_cc_(&self) -> bool {
        *self == CLRCCR::NO_AFFECT_ON_THE_CC_
    }
    #[doc = "Checks if the value of the field is `AUTO_CLEAR_THE_CC_B`"]
    #[inline]
    pub fn is_auto_clear_the_cc_b(&self) -> bool {
        *self == CLRCCR::AUTO_CLEAR_THE_CC_B
    }
}
#[doc = "Values that can be written to the field `TXBRKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRKENW {
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
    #[doc = "Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTRL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINUOUS_BREAK_IS_,
}
impl TXBRKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXBRKENW::NORMAL_OPERATION_ => false,
            TXBRKENW::CONTINUOUS_BREAK_IS_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXBRKENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBRKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXBRKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(TXBRKENW::NORMAL_OPERATION_)
    }
    #[doc = "Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTRL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    #[inline]
    pub fn continuous_break_is_(self) -> &'a mut W {
        self.variant(TXBRKENW::CONTINUOUS_BREAK_IS_)
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
#[doc = "Values that can be written to the field `ADDRDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDETW {
    #[doc = "Enabled. The USART receiver is enabled for all incoming data."]
    ENABLED_THE_USART_R,
    #[doc = "Disabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    DISABLED_THE_USART_,
}
impl ADDRDETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRDETW::ENABLED_THE_USART_R => false,
            ADDRDETW::DISABLED_THE_USART_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRDETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRDETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRDETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. The USART receiver is enabled for all incoming data."]
    #[inline]
    pub fn enabled_the_usart_r(self) -> &'a mut W {
        self.variant(ADDRDETW::ENABLED_THE_USART_R)
    }
    #[doc = "Disabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    #[inline]
    pub fn disabled_the_usart_(self) -> &'a mut W {
        self.variant(ADDRDETW::DISABLED_THE_USART_)
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
#[doc = "Values that can be written to the field `TXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDISW {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    NOT_DISABLED_USART_,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED_USART_TRAN,
}
impl TXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDISW::NOT_DISABLED_USART_ => false,
            TXDISW::DISABLED_USART_TRAN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not disabled. USART transmitter is not disabled."]
    #[inline]
    pub fn not_disabled_usart_(self) -> &'a mut W {
        self.variant(TXDISW::NOT_DISABLED_USART_)
    }
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    #[inline]
    pub fn disabled_usart_tran(self) -> &'a mut W {
        self.variant(TXDISW::DISABLED_USART_TRAN)
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
#[doc = "Values that can be written to the field `CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCW {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER_,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINUOUS_CLOCK_SC,
}
impl CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCW::CLOCK_ON_CHARACTER_ => false,
            CCW::CONTINUOUS_CLOCK_SC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    #[inline]
    pub fn clock_on_character_(self) -> &'a mut W {
        self.variant(CCW::CLOCK_ON_CHARACTER_)
    }
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    #[inline]
    pub fn continuous_clock_sc(self) -> &'a mut W {
        self.variant(CCW::CONTINUOUS_CLOCK_SC)
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
#[doc = "Values that can be written to the field `CLRCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRCCW {
    #[doc = "No affect on the CC bit."]
    NO_AFFECT_ON_THE_CC_,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR_THE_CC_B,
}
impl CLRCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRCCW::NO_AFFECT_ON_THE_CC_ => false,
            CLRCCW::AUTO_CLEAR_THE_CC_B => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRCCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRCCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No affect on the CC bit."]
    #[inline]
    pub fn no_affect_on_the_cc_(self) -> &'a mut W {
        self.variant(CLRCCW::NO_AFFECT_ON_THE_CC_)
    }
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    #[inline]
    pub fn auto_clear_the_cc_b(self) -> &'a mut W {
        self.variant(CLRCCW::AUTO_CLEAR_THE_CC_B)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Break Enable."]
    #[inline]
    pub fn txbrken(&self) -> TXBRKENR {
        TXBRKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline]
    pub fn addrdet(&self) -> ADDRDETR {
        ADDRDETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline]
    pub fn txdis(&self) -> TXDISR {
        TXDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline]
    pub fn cc(&self) -> CCR {
        CCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline]
    pub fn clrcc(&self) -> CLRCCR {
        CLRCCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 1 - Break Enable."]
    #[inline]
    pub fn txbrken(&mut self) -> _TXBRKENW {
        _TXBRKENW { w: self }
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline]
    pub fn addrdet(&mut self) -> _ADDRDETW {
        _ADDRDETW { w: self }
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline]
    pub fn txdis(&mut self) -> _TXDISW {
        _TXDISW { w: self }
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline]
    pub fn cc(&mut self) -> _CCW {
        _CCW { w: self }
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline]
    pub fn clrcc(&mut self) -> _CLRCCW {
        _CLRCCW { w: self }
    }
}
