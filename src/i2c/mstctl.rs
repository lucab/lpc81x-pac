#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSTCTL {
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
#[doc = "Possible values of the field `MSTCONTINUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCONTINUER {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE_INFORMS_TH,
}
impl MSTCONTINUER {
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
            MSTCONTINUER::NO_EFFECT_ => false,
            MSTCONTINUER::CONTINUE_INFORMS_TH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTCONTINUER {
        match value {
            false => MSTCONTINUER::NO_EFFECT_,
            true => MSTCONTINUER::CONTINUE_INFORMS_TH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline]
    pub fn is_no_effect_(&self) -> bool {
        *self == MSTCONTINUER::NO_EFFECT_
    }
    #[doc = "Checks if the value of the field is `CONTINUE_INFORMS_TH`"]
    #[inline]
    pub fn is_continue_informs_th(&self) -> bool {
        *self == MSTCONTINUER::CONTINUE_INFORMS_TH
    }
}
#[doc = "Possible values of the field `MSTSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTARTR {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START_A_START_WILL_,
}
impl MSTSTARTR {
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
            MSTSTARTR::NO_EFFECT_ => false,
            MSTSTARTR::START_A_START_WILL_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTARTR {
        match value {
            false => MSTSTARTR::NO_EFFECT_,
            true => MSTSTARTR::START_A_START_WILL_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline]
    pub fn is_no_effect_(&self) -> bool {
        *self == MSTSTARTR::NO_EFFECT_
    }
    #[doc = "Checks if the value of the field is `START_A_START_WILL_`"]
    #[inline]
    pub fn is_start_a_start_will_(&self) -> bool {
        *self == MSTSTARTR::START_A_START_WILL_
    }
}
#[doc = "Possible values of the field `MSTSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTOPR {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a Nack to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP_A_STOP_WILL_BE,
}
impl MSTSTOPR {
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
            MSTSTOPR::NO_EFFECT_ => false,
            MSTSTOPR::STOP_A_STOP_WILL_BE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTOPR {
        match value {
            false => MSTSTOPR::NO_EFFECT_,
            true => MSTSTOPR::STOP_A_STOP_WILL_BE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline]
    pub fn is_no_effect_(&self) -> bool {
        *self == MSTSTOPR::NO_EFFECT_
    }
    #[doc = "Checks if the value of the field is `STOP_A_STOP_WILL_BE`"]
    #[inline]
    pub fn is_stop_a_stop_will_be(&self) -> bool {
        *self == MSTSTOPR::STOP_A_STOP_WILL_BE
    }
}
#[doc = "Values that can be written to the field `MSTCONTINUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCONTINUEW {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE_INFORMS_TH,
}
impl MSTCONTINUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTCONTINUEW::NO_EFFECT_ => false,
            MSTCONTINUEW::CONTINUE_INFORMS_TH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTCONTINUEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTCONTINUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTCONTINUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(MSTCONTINUEW::NO_EFFECT_)
    }
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline]
    pub fn continue_informs_th(self) -> &'a mut W {
        self.variant(MSTCONTINUEW::CONTINUE_INFORMS_TH)
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
#[doc = "Values that can be written to the field `MSTSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTARTW {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START_A_START_WILL_,
}
impl MSTSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTARTW::NO_EFFECT_ => false,
            MSTSTARTW::START_A_START_WILL_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(MSTSTARTW::NO_EFFECT_)
    }
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    #[inline]
    pub fn start_a_start_will_(self) -> &'a mut W {
        self.variant(MSTSTARTW::START_A_START_WILL_)
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
#[doc = "Values that can be written to the field `MSTSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTOPW {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a Nack to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP_A_STOP_WILL_BE,
}
impl MSTSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTOPW::NO_EFFECT_ => false,
            MSTSTOPW::STOP_A_STOP_WILL_BE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(MSTSTOPW::NO_EFFECT_)
    }
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a Nack to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    #[inline]
    pub fn stop_a_stop_will_be(self) -> &'a mut W {
        self.variant(MSTSTOPW::STOP_A_STOP_WILL_BE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline]
    pub fn mstcontinue(&self) -> MSTCONTINUER {
        MSTCONTINUER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline]
    pub fn mststart(&self) -> MSTSTARTR {
        MSTSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline]
    pub fn mststop(&self) -> MSTSTOPR {
        MSTSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline]
    pub fn mstcontinue(&mut self) -> _MSTCONTINUEW {
        _MSTCONTINUEW { w: self }
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline]
    pub fn mststart(&mut self) -> _MSTSTARTW {
        _MSTSTARTW { w: self }
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline]
    pub fn mststop(&mut self) -> _MSTSTOPW {
        _MSTSTOPW { w: self }
    }
}
