#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLVCTL {
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
#[doc = "Possible values of the field `SlvContinue`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVCONTINUER {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Continue. Informs the Slave function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE_INFORMS_TH,
}
impl SLVCONTINUER {
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
            SLVCONTINUER::NO_EFFECT_ => false,
            SLVCONTINUER::CONTINUE_INFORMS_TH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVCONTINUER {
        match value {
            false => SLVCONTINUER::NO_EFFECT_,
            true => SLVCONTINUER::CONTINUE_INFORMS_TH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline]
    pub fn is_no_effect_(&self) -> bool {
        *self == SLVCONTINUER::NO_EFFECT_
    }
    #[doc = "Checks if the value of the field is `CONTINUE_INFORMS_TH`"]
    #[inline]
    pub fn is_continue_informs_th(&self) -> bool {
        *self == SLVCONTINUER::CONTINUE_INFORMS_TH
    }
}
#[doc = "Possible values of the field `SlvNack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNACKR {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Nack. Causes the Slave function to Nack the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK_CAUSES_THE_SLA,
}
impl SLVNACKR {
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
            SLVNACKR::NO_EFFECT_ => false,
            SLVNACKR::NACK_CAUSES_THE_SLA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVNACKR {
        match value {
            false => SLVNACKR::NO_EFFECT_,
            true => SLVNACKR::NACK_CAUSES_THE_SLA,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline]
    pub fn is_no_effect_(&self) -> bool {
        *self == SLVNACKR::NO_EFFECT_
    }
    #[doc = "Checks if the value of the field is `NACK_CAUSES_THE_SLA`"]
    #[inline]
    pub fn is_nack_causes_the_sla(&self) -> bool {
        *self == SLVNACKR::NACK_CAUSES_THE_SLA
    }
}
#[doc = "Values that can be written to the field `SlvContinue`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVCONTINUEW {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Continue. Informs the Slave function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE_INFORMS_TH,
}
impl SLVCONTINUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVCONTINUEW::NO_EFFECT_ => false,
            SLVCONTINUEW::CONTINUE_INFORMS_TH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVCONTINUEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVCONTINUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVCONTINUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(SLVCONTINUEW::NO_EFFECT_)
    }
    #[doc = "Continue. Informs the Slave function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline]
    pub fn continue_informs_th(self) -> &'a mut W {
        self.variant(SLVCONTINUEW::CONTINUE_INFORMS_TH)
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
#[doc = "Values that can be written to the field `SlvNack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNACKW {
    #[doc = "No effect."]
    NO_EFFECT_,
    #[doc = "Nack. Causes the Slave function to Nack the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK_CAUSES_THE_SLA,
}
impl SLVNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNACKW::NO_EFFECT_ => false,
            SLVNACKW::NACK_CAUSES_THE_SLA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(SLVNACKW::NO_EFFECT_)
    }
    #[doc = "Nack. Causes the Slave function to Nack the master when the slave is receiving data from the master (Slave Receiver mode)."]
    #[inline]
    pub fn nack_causes_the_sla(self) -> &'a mut W {
        self.variant(SLVNACKW::NACK_CAUSES_THE_SLA)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Slave Continue."]
    #[inline]
    pub fn slv_continue(&self) -> SLVCONTINUER {
        SLVCONTINUER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Slave Nack."]
    #[inline]
    pub fn slv_nack(&self) -> SLVNACKR {
        SLVNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Slave Continue."]
    #[inline]
    pub fn slv_continue(&mut self) -> _SLVCONTINUEW {
        _SLVCONTINUEW { w: self }
    }
    #[doc = "Bit 1 - Slave Nack."]
    #[inline]
    pub fn slv_nack(&mut self) -> _SLVNACKW {
        _SLVNACKW { w: self }
    }
}
