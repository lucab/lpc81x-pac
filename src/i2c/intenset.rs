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
#[doc = "Possible values of the field `MSTPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGENR {
    #[doc = "The MstPending interrupt is disabled."]
    DISABLED,
    #[doc = "The MstPending interrupt is enabled."]
    ENABLED,
}
impl MSTPENDINGENR {
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
            MSTPENDINGENR::DISABLED => false,
            MSTPENDINGENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTPENDINGENR {
        match value {
            false => MSTPENDINGENR::DISABLED,
            true => MSTPENDINGENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MSTPENDINGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MSTPENDINGENR::ENABLED
    }
}
#[doc = "Possible values of the field `MSTARBLOSSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSENR {
    #[doc = "The MstArbLoss interrupt is disabled."]
    DISABLED,
    #[doc = "The MstArbLoss interrupt is enabled."]
    THE_MSTARBLOSS_INTER,
}
impl MSTARBLOSSENR {
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
            MSTARBLOSSENR::DISABLED => false,
            MSTARBLOSSENR::THE_MSTARBLOSS_INTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTARBLOSSENR {
        match value {
            false => MSTARBLOSSENR::DISABLED,
            true => MSTARBLOSSENR::THE_MSTARBLOSS_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MSTARBLOSSENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_MSTARBLOSS_INTER`"]
    #[inline]
    pub fn is_the_mstarbloss_inter(&self) -> bool {
        *self == MSTARBLOSSENR::THE_MSTARBLOSS_INTER
    }
}
#[doc = "Possible values of the field `MSTSTSTPERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRENR {
    #[doc = "The MstStStpErr interrupt is disabled."]
    DISABLED,
    #[doc = "The MstStStpErr interrupt is enabled."]
    THE_MSTSTSTPERR_INTE,
}
impl MSTSTSTPERRENR {
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
            MSTSTSTPERRENR::DISABLED => false,
            MSTSTSTPERRENR::THE_MSTSTSTPERR_INTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTSTPERRENR {
        match value {
            false => MSTSTSTPERRENR::DISABLED,
            true => MSTSTSTPERRENR::THE_MSTSTSTPERR_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MSTSTSTPERRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_MSTSTSTPERR_INTE`"]
    #[inline]
    pub fn is_the_mstststperr_inte(&self) -> bool {
        *self == MSTSTSTPERRENR::THE_MSTSTSTPERR_INTE
    }
}
#[doc = "Possible values of the field `SLVPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGENR {
    #[doc = "The SlvPending interrupt is disabled."]
    DISABLED,
    #[doc = "The SlvPending interrupt is enabled."]
    THE_SLVPENDING_INTER,
}
impl SLVPENDINGENR {
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
            SLVPENDINGENR::DISABLED => false,
            SLVPENDINGENR::THE_SLVPENDING_INTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVPENDINGENR {
        match value {
            false => SLVPENDINGENR::DISABLED,
            true => SLVPENDINGENR::THE_SLVPENDING_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLVPENDINGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_SLVPENDING_INTER`"]
    #[inline]
    pub fn is_the_slvpending_inter(&self) -> bool {
        *self == SLVPENDINGENR::THE_SLVPENDING_INTER
    }
}
#[doc = "Possible values of the field `SLVNOTSTREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRENR {
    #[doc = "The SlvNotStr interrupt is disabled."]
    DISABLED,
    #[doc = "The SlvNotStr interrupt is enabled."]
    THE_SLVNOTSTR_INTERR,
}
impl SLVNOTSTRENR {
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
            SLVNOTSTRENR::DISABLED => false,
            SLVNOTSTRENR::THE_SLVNOTSTR_INTERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVNOTSTRENR {
        match value {
            false => SLVNOTSTRENR::DISABLED,
            true => SLVNOTSTRENR::THE_SLVNOTSTR_INTERR,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLVNOTSTRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_SLVNOTSTR_INTERR`"]
    #[inline]
    pub fn is_the_slvnotstr_interr(&self) -> bool {
        *self == SLVNOTSTRENR::THE_SLVNOTSTR_INTERR
    }
}
#[doc = "Possible values of the field `SLVDESELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELENR {
    #[doc = "The SlvDeSel interrupt is disabled."]
    DISABLED,
    #[doc = "The SlvDeSel interrupt is enabled."]
    THE_SLVDESEL_INTERRU,
}
impl SLVDESELENR {
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
            SLVDESELENR::DISABLED => false,
            SLVDESELENR::THE_SLVDESEL_INTERRU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVDESELENR {
        match value {
            false => SLVDESELENR::DISABLED,
            true => SLVDESELENR::THE_SLVDESEL_INTERRU,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDESELENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_SLVDESEL_INTERRU`"]
    #[inline]
    pub fn is_the_slvdesel_interru(&self) -> bool {
        *self == SLVDESELENR::THE_SLVDESEL_INTERRU
    }
}
#[doc = "Possible values of the field `MONRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYENR {
    #[doc = "The MonRdy interrupt is disabled."]
    DISABLED,
    #[doc = "The MonRdy interrupt is enabled."]
    THE_MONRDY_INTERRUPT,
}
impl MONRDYENR {
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
            MONRDYENR::DISABLED => false,
            MONRDYENR::THE_MONRDY_INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONRDYENR {
        match value {
            false => MONRDYENR::DISABLED,
            true => MONRDYENR::THE_MONRDY_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MONRDYENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_MONRDY_INTERRUPT`"]
    #[inline]
    pub fn is_the_monrdy_interrupt(&self) -> bool {
        *self == MONRDYENR::THE_MONRDY_INTERRUPT
    }
}
#[doc = "Possible values of the field `MONOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVENR {
    #[doc = "The MonOv interrupt is disabled."]
    DISABLED,
    #[doc = "The MonOv interrupt is enabled."]
    THE_MONOV_INTERRUPT_,
}
impl MONOVENR {
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
            MONOVENR::DISABLED => false,
            MONOVENR::THE_MONOV_INTERRUPT_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONOVENR {
        match value {
            false => MONOVENR::DISABLED,
            true => MONOVENR::THE_MONOV_INTERRUPT_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MONOVENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_MONOV_INTERRUPT_`"]
    #[inline]
    pub fn is_the_monov_interrupt_(&self) -> bool {
        *self == MONOVENR::THE_MONOV_INTERRUPT_
    }
}
#[doc = "Possible values of the field `MONIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEENR {
    #[doc = "The MonIdle interrupt is disabled."]
    DISABLED,
    #[doc = "The MonIdle interrupt is enabled."]
    THE_MONIDLE_INTERRUP,
}
impl MONIDLEENR {
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
            MONIDLEENR::DISABLED => false,
            MONIDLEENR::THE_MONIDLE_INTERRUP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONIDLEENR {
        match value {
            false => MONIDLEENR::DISABLED,
            true => MONIDLEENR::THE_MONIDLE_INTERRUP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MONIDLEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_MONIDLE_INTERRUP`"]
    #[inline]
    pub fn is_the_monidle_interrup(&self) -> bool {
        *self == MONIDLEENR::THE_MONIDLE_INTERRUP
    }
}
#[doc = "Possible values of the field `EVENTTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTENR {
    #[doc = "The Event Timeout interrupt is disabled."]
    DISABLED,
    #[doc = "The Event Timeout interrupt is enabled."]
    THE_EVENT_TIMEOUT_IN,
}
impl EVENTTIMEOUTENR {
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
            EVENTTIMEOUTENR::DISABLED => false,
            EVENTTIMEOUTENR::THE_EVENT_TIMEOUT_IN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVENTTIMEOUTENR {
        match value {
            false => EVENTTIMEOUTENR::DISABLED,
            true => EVENTTIMEOUTENR::THE_EVENT_TIMEOUT_IN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EVENTTIMEOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_EVENT_TIMEOUT_IN`"]
    #[inline]
    pub fn is_the_event_timeout_in(&self) -> bool {
        *self == EVENTTIMEOUTENR::THE_EVENT_TIMEOUT_IN
    }
}
#[doc = "Possible values of the field `SCLTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTENR {
    #[doc = "The SCL Timeout interrupt is disabled."]
    DISABLED,
    #[doc = "The SCL Timeout interrupt is enabled."]
    THE_SCL_TIMEOUT_INTE,
}
impl SCLTIMEOUTENR {
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
            SCLTIMEOUTENR::DISABLED => false,
            SCLTIMEOUTENR::THE_SCL_TIMEOUT_INTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLTIMEOUTENR {
        match value {
            false => SCLTIMEOUTENR::DISABLED,
            true => SCLTIMEOUTENR::THE_SCL_TIMEOUT_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SCLTIMEOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_SCL_TIMEOUT_INTE`"]
    #[inline]
    pub fn is_the_scl_timeout_inte(&self) -> bool {
        *self == SCLTIMEOUTENR::THE_SCL_TIMEOUT_INTE
    }
}
#[doc = "Values that can be written to the field `MSTPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGENW {
    #[doc = "The MstPending interrupt is disabled."]
    DISABLED,
    #[doc = "The MstPending interrupt is enabled."]
    ENABLED,
}
impl MSTPENDINGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTPENDINGENW::DISABLED => false,
            MSTPENDINGENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTPENDINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTPENDINGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTPENDINGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MstPending interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTPENDINGENW::DISABLED)
    }
    #[doc = "The MstPending interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTPENDINGENW::ENABLED)
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
#[doc = "Values that can be written to the field `MSTARBLOSSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSENW {
    #[doc = "The MstArbLoss interrupt is disabled."]
    DISABLED,
    #[doc = "The MstArbLoss interrupt is enabled."]
    THE_MSTARBLOSS_INTER,
}
impl MSTARBLOSSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSENW::DISABLED => false,
            MSTARBLOSSENW::THE_MSTARBLOSS_INTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTARBLOSSENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTARBLOSSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTARBLOSSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MstArbLoss interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSENW::DISABLED)
    }
    #[doc = "The MstArbLoss interrupt is enabled."]
    #[inline]
    pub fn the_mstarbloss_inter(self) -> &'a mut W {
        self.variant(MSTARBLOSSENW::THE_MSTARBLOSS_INTER)
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
#[doc = "Values that can be written to the field `MSTSTSTPERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRENW {
    #[doc = "The MstStStpErr interrupt is disabled."]
    DISABLED,
    #[doc = "The MstStStpErr interrupt is enabled."]
    THE_MSTSTSTPERR_INTE,
}
impl MSTSTSTPERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRENW::DISABLED => false,
            MSTSTSTPERRENW::THE_MSTSTSTPERR_INTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTSTPERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTSTPERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTSTPERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MstStStpErr interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERRENW::DISABLED)
    }
    #[doc = "The MstStStpErr interrupt is enabled."]
    #[inline]
    pub fn the_mstststperr_inte(self) -> &'a mut W {
        self.variant(MSTSTSTPERRENW::THE_MSTSTSTPERR_INTE)
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
#[doc = "Values that can be written to the field `SLVPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGENW {
    #[doc = "The SlvPending interrupt is disabled."]
    DISABLED,
    #[doc = "The SlvPending interrupt is enabled."]
    THE_SLVPENDING_INTER,
}
impl SLVPENDINGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVPENDINGENW::DISABLED => false,
            SLVPENDINGENW::THE_SLVPENDING_INTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVPENDINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVPENDINGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVPENDINGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SlvPending interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVPENDINGENW::DISABLED)
    }
    #[doc = "The SlvPending interrupt is enabled."]
    #[inline]
    pub fn the_slvpending_inter(self) -> &'a mut W {
        self.variant(SLVPENDINGENW::THE_SLVPENDING_INTER)
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
#[doc = "Values that can be written to the field `SLVNOTSTREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRENW {
    #[doc = "The SlvNotStr interrupt is disabled."]
    DISABLED,
    #[doc = "The SlvNotStr interrupt is enabled."]
    THE_SLVNOTSTR_INTERR,
}
impl SLVNOTSTRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNOTSTRENW::DISABLED => false,
            SLVNOTSTRENW::THE_SLVNOTSTR_INTERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVNOTSTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNOTSTRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVNOTSTRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SlvNotStr interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVNOTSTRENW::DISABLED)
    }
    #[doc = "The SlvNotStr interrupt is enabled."]
    #[inline]
    pub fn the_slvnotstr_interr(self) -> &'a mut W {
        self.variant(SLVNOTSTRENW::THE_SLVNOTSTR_INTERR)
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
#[doc = "Values that can be written to the field `SLVDESELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELENW {
    #[doc = "The SlvDeSel interrupt is disabled."]
    DISABLED,
    #[doc = "The SlvDeSel interrupt is enabled."]
    THE_SLVDESEL_INTERRU,
}
impl SLVDESELENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDESELENW::DISABLED => false,
            SLVDESELENW::THE_SLVDESEL_INTERRU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVDESELENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDESELENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVDESELENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SlvDeSel interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDESELENW::DISABLED)
    }
    #[doc = "The SlvDeSel interrupt is enabled."]
    #[inline]
    pub fn the_slvdesel_interru(self) -> &'a mut W {
        self.variant(SLVDESELENW::THE_SLVDESEL_INTERRU)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYENW {
    #[doc = "The MonRdy interrupt is disabled."]
    DISABLED,
    #[doc = "The MonRdy interrupt is enabled."]
    THE_MONRDY_INTERRUPT,
}
impl MONRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONRDYENW::DISABLED => false,
            MONRDYENW::THE_MONRDY_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MonRdy interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONRDYENW::DISABLED)
    }
    #[doc = "The MonRdy interrupt is enabled."]
    #[inline]
    pub fn the_monrdy_interrupt(self) -> &'a mut W {
        self.variant(MONRDYENW::THE_MONRDY_INTERRUPT)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVENW {
    #[doc = "The MonOv interrupt is disabled."]
    DISABLED,
    #[doc = "The MonOv interrupt is enabled."]
    THE_MONOV_INTERRUPT_,
}
impl MONOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONOVENW::DISABLED => false,
            MONOVENW::THE_MONOV_INTERRUPT_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MonOv interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONOVENW::DISABLED)
    }
    #[doc = "The MonOv interrupt is enabled."]
    #[inline]
    pub fn the_monov_interrupt_(self) -> &'a mut W {
        self.variant(MONOVENW::THE_MONOV_INTERRUPT_)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEENW {
    #[doc = "The MonIdle interrupt is disabled."]
    DISABLED,
    #[doc = "The MonIdle interrupt is enabled."]
    THE_MONIDLE_INTERRUP,
}
impl MONIDLEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONIDLEENW::DISABLED => false,
            MONIDLEENW::THE_MONIDLE_INTERRUP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONIDLEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONIDLEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MonIdle interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONIDLEENW::DISABLED)
    }
    #[doc = "The MonIdle interrupt is enabled."]
    #[inline]
    pub fn the_monidle_interrup(self) -> &'a mut W {
        self.variant(MONIDLEENW::THE_MONIDLE_INTERRUP)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EVENTTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTENW {
    #[doc = "The Event Timeout interrupt is disabled."]
    DISABLED,
    #[doc = "The Event Timeout interrupt is enabled."]
    THE_EVENT_TIMEOUT_IN,
}
impl EVENTTIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTENW::DISABLED => false,
            EVENTTIMEOUTENW::THE_EVENT_TIMEOUT_IN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVENTTIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTTIMEOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVENTTIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Event Timeout interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTENW::DISABLED)
    }
    #[doc = "The Event Timeout interrupt is enabled."]
    #[inline]
    pub fn the_event_timeout_in(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTENW::THE_EVENT_TIMEOUT_IN)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTENW {
    #[doc = "The SCL Timeout interrupt is disabled."]
    DISABLED,
    #[doc = "The SCL Timeout interrupt is enabled."]
    THE_SCL_TIMEOUT_INTE,
}
impl SCLTIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTENW::DISABLED => false,
            SCLTIMEOUTENW::THE_SCL_TIMEOUT_INTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLTIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLTIMEOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLTIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SCL Timeout interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTENW::DISABLED)
    }
    #[doc = "The SCL Timeout interrupt is enabled."]
    #[inline]
    pub fn the_scl_timeout_inte(self) -> &'a mut W {
        self.variant(SCLTIMEOUTENW::THE_SCL_TIMEOUT_INTE)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline]
    pub fn mstpendingen(&self) -> MSTPENDINGENR {
        MSTPENDINGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline]
    pub fn mstarblossen(&self) -> MSTARBLOSSENR {
        MSTARBLOSSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline]
    pub fn mstststperren(&self) -> MSTSTSTPERRENR {
        MSTSTSTPERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline]
    pub fn slvpendingen(&self) -> SLVPENDINGENR {
        SLVPENDINGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline]
    pub fn slvnotstren(&self) -> SLVNOTSTRENR {
        SLVNOTSTRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline]
    pub fn slvdeselen(&self) -> SLVDESELENR {
        SLVDESELENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline]
    pub fn monrdyen(&self) -> MONRDYENR {
        MONRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline]
    pub fn monoven(&self) -> MONOVENR {
        MONOVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline]
    pub fn monidleen(&self) -> MONIDLEENR {
        MONIDLEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Event Timeout interrupt Enable."]
    #[inline]
    pub fn eventtimeouten(&self) -> EVENTTIMEOUTENR {
        EVENTTIMEOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - SCL Timeout interrupt Enable."]
    #[inline]
    pub fn scltimeouten(&self) -> SCLTIMEOUTENR {
        SCLTIMEOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline]
    pub fn mstpendingen(&mut self) -> _MSTPENDINGENW {
        _MSTPENDINGENW { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline]
    pub fn mstarblossen(&mut self) -> _MSTARBLOSSENW {
        _MSTARBLOSSENW { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline]
    pub fn mstststperren(&mut self) -> _MSTSTSTPERRENW {
        _MSTSTSTPERRENW { w: self }
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline]
    pub fn slvpendingen(&mut self) -> _SLVPENDINGENW {
        _SLVPENDINGENW { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline]
    pub fn slvnotstren(&mut self) -> _SLVNOTSTRENW {
        _SLVNOTSTRENW { w: self }
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline]
    pub fn slvdeselen(&mut self) -> _SLVDESELENW {
        _SLVDESELENW { w: self }
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline]
    pub fn monrdyen(&mut self) -> _MONRDYENW {
        _MONRDYENW { w: self }
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline]
    pub fn monoven(&mut self) -> _MONOVENW {
        _MONOVENW { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline]
    pub fn monidleen(&mut self) -> _MONIDLEENW {
        _MONIDLEENW { w: self }
    }
    #[doc = "Bit 24 - Event Timeout interrupt Enable."]
    #[inline]
    pub fn eventtimeouten(&mut self) -> _EVENTTIMEOUTENW {
        _EVENTTIMEOUTENW { w: self }
    }
    #[doc = "Bit 25 - SCL Timeout interrupt Enable."]
    #[inline]
    pub fn scltimeouten(&mut self) -> _SCLTIMEOUTENW {
        _SCLTIMEOUTENW { w: self }
    }
}
