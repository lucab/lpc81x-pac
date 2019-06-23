#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPDCTRL {
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
#[doc = "Possible values of the field `WAKEUPHYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYSR {
    #[doc = "Disabled. Hysteresis for WAKUP pin disabled."]
    DISABLED_HYSTERESIS,
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    ENABLED_HYSTERESIS_,
}
impl WAKEUPHYSR {
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
            WAKEUPHYSR::DISABLED_HYSTERESIS => false,
            WAKEUPHYSR::ENABLED_HYSTERESIS_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUPHYSR {
        match value {
            false => WAKEUPHYSR::DISABLED_HYSTERESIS,
            true => WAKEUPHYSR::ENABLED_HYSTERESIS_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_HYSTERESIS`"]
    #[inline]
    pub fn is_disabled_hysteresis(&self) -> bool {
        *self == WAKEUPHYSR::DISABLED_HYSTERESIS
    }
    #[doc = "Checks if the value of the field is `ENABLED_HYSTERESIS_`"]
    #[inline]
    pub fn is_enabled_hysteresis_(&self) -> bool {
        *self == WAKEUPHYSR::ENABLED_HYSTERESIS_
    }
}
#[doc = "Possible values of the field `WAKEPAD_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEPAD_DISABLER {
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    ENABLED_THE_WAKE_UP,
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    DISABLED_SETTING_TH,
}
impl WAKEPAD_DISABLER {
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
            WAKEPAD_DISABLER::ENABLED_THE_WAKE_UP => false,
            WAKEPAD_DISABLER::DISABLED_SETTING_TH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEPAD_DISABLER {
        match value {
            false => WAKEPAD_DISABLER::ENABLED_THE_WAKE_UP,
            true => WAKEPAD_DISABLER::DISABLED_SETTING_TH,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_WAKE_UP`"]
    #[inline]
    pub fn is_enabled_the_wake_up(&self) -> bool {
        *self == WAKEPAD_DISABLER::ENABLED_THE_WAKE_UP
    }
    #[doc = "Checks if the value of the field is `DISABLED_SETTING_TH`"]
    #[inline]
    pub fn is_disabled_setting_th(&self) -> bool {
        *self == WAKEPAD_DISABLER::DISABLED_SETTING_TH
    }
}
#[doc = "Possible values of the field `LPOSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCENR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
}
impl LPOSCENR {
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
            LPOSCENR::DISABLED_ => false,
            LPOSCENR::ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPOSCENR {
        match value {
            false => LPOSCENR::DISABLED_,
            true => LPOSCENR::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == LPOSCENR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == LPOSCENR::ENABLED_
    }
}
#[doc = "Possible values of the field `LPOSCDPDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCDPDENR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
}
impl LPOSCDPDENR {
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
            LPOSCDPDENR::DISABLED_ => false,
            LPOSCDPDENR::ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPOSCDPDENR {
        match value {
            false => LPOSCDPDENR::DISABLED_,
            true => LPOSCDPDENR::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == LPOSCDPDENR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == LPOSCDPDENR::ENABLED_
    }
}
#[doc = "Values that can be written to the field `WAKEUPHYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYSW {
    #[doc = "Disabled. Hysteresis for WAKUP pin disabled."]
    DISABLED_HYSTERESIS,
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    ENABLED_HYSTERESIS_,
}
impl WAKEUPHYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUPHYSW::DISABLED_HYSTERESIS => false,
            WAKEUPHYSW::ENABLED_HYSTERESIS_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPHYSW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPHYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUPHYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Hysteresis for WAKUP pin disabled."]
    #[inline]
    pub fn disabled_hysteresis(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::DISABLED_HYSTERESIS)
    }
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    #[inline]
    pub fn enabled_hysteresis_(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::ENABLED_HYSTERESIS_)
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
#[doc = "Values that can be written to the field `WAKEPAD_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEPAD_DISABLEW {
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    ENABLED_THE_WAKE_UP,
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    DISABLED_SETTING_TH,
}
impl WAKEPAD_DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEPAD_DISABLEW::ENABLED_THE_WAKE_UP => false,
            WAKEPAD_DISABLEW::DISABLED_SETTING_TH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEPAD_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEPAD_DISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEPAD_DISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    #[inline]
    pub fn enabled_the_wake_up(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLEW::ENABLED_THE_WAKE_UP)
    }
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    #[inline]
    pub fn disabled_setting_th(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLEW::DISABLED_SETTING_TH)
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
#[doc = "Values that can be written to the field `LPOSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCENW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
}
impl LPOSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPOSCENW::DISABLED_ => false,
            LPOSCENW::ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPOSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPOSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPOSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(LPOSCENW::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(LPOSCENW::ENABLED_)
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
#[doc = "Values that can be written to the field `LPOSCDPDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCDPDENW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
}
impl LPOSCDPDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPOSCDPDENW::DISABLED_ => false,
            LPOSCDPDENW::ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPOSCDPDENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPOSCDPDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPOSCDPDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(LPOSCDPDENW::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(LPOSCDPDENW::ENABLED_)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline]
    pub fn wakeuphys(&self) -> WAKEUPHYSR {
        WAKEUPHYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline]
    pub fn wakepad_disable(&self) -> WAKEPAD_DISABLER {
        WAKEPAD_DISABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC."]
    #[inline]
    pub fn lposcen(&self) -> LPOSCENR {
        LPOSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable the low-power oscillator in Deep power-down mode. Setting this bit causes the low-power oscillator to remain running during Deep power-down mode provided that bit 12 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Do not set this bit unless you must use the self wake-up timer to wake up from Deep power-down mode."]
    #[inline]
    pub fn lposcdpden(&self) -> LPOSCDPDENR {
        LPOSCDPDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline]
    pub fn wakeuphys(&mut self) -> _WAKEUPHYSW {
        _WAKEUPHYSW { w: self }
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline]
    pub fn wakepad_disable(&mut self) -> _WAKEPAD_DISABLEW {
        _WAKEPAD_DISABLEW { w: self }
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC."]
    #[inline]
    pub fn lposcen(&mut self) -> _LPOSCENW {
        _LPOSCENW { w: self }
    }
    #[doc = "Bit 3 - Enable the low-power oscillator in Deep power-down mode. Setting this bit causes the low-power oscillator to remain running during Deep power-down mode provided that bit 12 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Do not set this bit unless you must use the self wake-up timer to wake up from Deep power-down mode."]
    #[inline]
    pub fn lposcdpden(&mut self) -> _LPOSCDPDENW {
        _LPOSCDPDENW { w: self }
    }
}
