#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCON {
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
#[doc = "Possible values of the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMR {
    #[doc = "Default. The part is in active or sleep mode."]
    DEFAULT_THE_PART_IS,
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    ARM_WFI_WILL_ENTER_DEEP_SLEEP,
    #[doc = "ARM WFI will enter Power-down mode."]
    ARM_WFI_WILL_ENTER_P,
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMR::DEFAULT_THE_PART_IS => 0,
            PMR::ARM_WFI_WILL_ENTER_DEEP_SLEEP => 1,
            PMR::ARM_WFI_WILL_ENTER_P => 2,
            PMR::ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN => 3,
            PMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMR {
        match value {
            0 => PMR::DEFAULT_THE_PART_IS,
            1 => PMR::ARM_WFI_WILL_ENTER_DEEP_SLEEP,
            2 => PMR::ARM_WFI_WILL_ENTER_P,
            3 => PMR::ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN,
            i => PMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT_THE_PART_IS`"]
    #[inline]
    pub fn is_default_the_part_is(&self) -> bool {
        *self == PMR::DEFAULT_THE_PART_IS
    }
    #[doc = "Checks if the value of the field is `ARM_WFI_WILL_ENTER_DEEP_SLEEP`"]
    #[inline]
    pub fn is_arm_wfi_will_enter_deep_sleep(&self) -> bool {
        *self == PMR::ARM_WFI_WILL_ENTER_DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `ARM_WFI_WILL_ENTER_P`"]
    #[inline]
    pub fn is_arm_wfi_will_enter_p(&self) -> bool {
        *self == PMR::ARM_WFI_WILL_ENTER_P
    }
    #[doc = "Checks if the value of the field is `ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN`"]
    #[inline]
    pub fn is_arm_wfi_will_enter_deep_power_down(&self) -> bool {
        *self == PMR::ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN
    }
}
#[doc = r" Value of the field"]
pub struct NODPDR {
    bits: bool,
}
impl NODPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `SLEEPFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAGR {
    #[doc = "Read: No power-down mode entered. LPC11Uxx is in Active mode. Write: No effect."]
    READ_NO_POWER_DOWN_,
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    READ_SLEEPDEEP_SLE,
}
impl SLEEPFLAGR {
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
            SLEEPFLAGR::READ_NO_POWER_DOWN_ => false,
            SLEEPFLAGR::READ_SLEEPDEEP_SLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPFLAGR {
        match value {
            false => SLEEPFLAGR::READ_NO_POWER_DOWN_,
            true => SLEEPFLAGR::READ_SLEEPDEEP_SLE,
        }
    }
    #[doc = "Checks if the value of the field is `READ_NO_POWER_DOWN_`"]
    #[inline]
    pub fn is_read_no_power_down_(&self) -> bool {
        *self == SLEEPFLAGR::READ_NO_POWER_DOWN_
    }
    #[doc = "Checks if the value of the field is `READ_SLEEPDEEP_SLE`"]
    #[inline]
    pub fn is_read_sleepdeep_sle(&self) -> bool {
        *self == SLEEPFLAGR::READ_SLEEPDEEP_SLE
    }
}
#[doc = "Possible values of the field `DPDFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAGR {
    #[doc = "Read: Deep power-down mode  not entered. Write: No effect."]
    READ_DEEP_POWER_DOWN_NOT_ENTERED,
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    READ_DEEP_POWER_DOWN_ENTERED,
}
impl DPDFLAGR {
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
            DPDFLAGR::READ_DEEP_POWER_DOWN_NOT_ENTERED => false,
            DPDFLAGR::READ_DEEP_POWER_DOWN_ENTERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDFLAGR {
        match value {
            false => DPDFLAGR::READ_DEEP_POWER_DOWN_NOT_ENTERED,
            true => DPDFLAGR::READ_DEEP_POWER_DOWN_ENTERED,
        }
    }
    #[doc = "Checks if the value of the field is `READ_DEEP_POWER_DOWN_NOT_ENTERED`"]
    #[inline]
    pub fn is_read_deep_power_down_not_entered(&self) -> bool {
        *self == DPDFLAGR::READ_DEEP_POWER_DOWN_NOT_ENTERED
    }
    #[doc = "Checks if the value of the field is `READ_DEEP_POWER_DOWN_ENTERED`"]
    #[inline]
    pub fn is_read_deep_power_down_entered(&self) -> bool {
        *self == DPDFLAGR::READ_DEEP_POWER_DOWN_ENTERED
    }
}
#[doc = "Values that can be written to the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMW {
    #[doc = "Default. The part is in active or sleep mode."]
    DEFAULT_THE_PART_IS,
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    ARM_WFI_WILL_ENTER_DEEP_SLEEP,
    #[doc = "ARM WFI will enter Power-down mode."]
    ARM_WFI_WILL_ENTER_P,
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMW::DEFAULT_THE_PART_IS => 0,
            PMW::ARM_WFI_WILL_ENTER_DEEP_SLEEP => 1,
            PMW::ARM_WFI_WILL_ENTER_P => 2,
            PMW::ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default. The part is in active or sleep mode."]
    #[inline]
    pub fn default_the_part_is(self) -> &'a mut W {
        self.variant(PMW::DEFAULT_THE_PART_IS)
    }
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    #[inline]
    pub fn arm_wfi_will_enter_deep_sleep(self) -> &'a mut W {
        self.variant(PMW::ARM_WFI_WILL_ENTER_DEEP_SLEEP)
    }
    #[doc = "ARM WFI will enter Power-down mode."]
    #[inline]
    pub fn arm_wfi_will_enter_p(self) -> &'a mut W {
        self.variant(PMW::ARM_WFI_WILL_ENTER_P)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    #[inline]
    pub fn arm_wfi_will_enter_deep_power_down(self) -> &'a mut W {
        self.variant(PMW::ARM_WFI_WILL_ENTER_DEEP_POWER_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NODPDW<'a> {
    w: &'a mut W,
}
impl<'a> _NODPDW<'a> {
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
#[doc = "Values that can be written to the field `SLEEPFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAGW {
    #[doc = "Read: No power-down mode entered. LPC11Uxx is in Active mode. Write: No effect."]
    READ_NO_POWER_DOWN_,
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    READ_SLEEPDEEP_SLE,
}
impl SLEEPFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPFLAGW::READ_NO_POWER_DOWN_ => false,
            SLEEPFLAGW::READ_SLEEPDEEP_SLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: No power-down mode entered. LPC11Uxx is in Active mode. Write: No effect."]
    #[inline]
    pub fn read_no_power_down_(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::READ_NO_POWER_DOWN_)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline]
    pub fn read_sleepdeep_sle(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::READ_SLEEPDEEP_SLE)
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
#[doc = "Values that can be written to the field `DPDFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAGW {
    #[doc = "Read: Deep power-down mode  not entered. Write: No effect."]
    READ_DEEP_POWER_DOWN_NOT_ENTERED,
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    READ_DEEP_POWER_DOWN_ENTERED,
}
impl DPDFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDFLAGW::READ_DEEP_POWER_DOWN_NOT_ENTERED => false,
            DPDFLAGW::READ_DEEP_POWER_DOWN_ENTERED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPDFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPDFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline]
    pub fn read_deep_power_down_not_entered(self) -> &'a mut W {
        self.variant(DPDFLAGW::READ_DEEP_POWER_DOWN_NOT_ENTERED)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline]
    pub fn read_deep_power_down_entered(self) -> &'a mut W {
        self.variant(DPDFLAGW::READ_DEEP_POWER_DOWN_ENTERED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Power mode"]
    #[inline]
    pub fn pm(&self) -> PMR {
        PMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline]
    pub fn nodpd(&self) -> NODPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NODPDR { bits }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline]
    pub fn sleepflag(&self) -> SLEEPFLAGR {
        SLEEPFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline]
    pub fn dpdflag(&self) -> DPDFLAGR {
        DPDFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:2 - Power mode"]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline]
    pub fn nodpd(&mut self) -> _NODPDW {
        _NODPDW { w: self }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline]
    pub fn sleepflag(&mut self) -> _SLEEPFLAGW {
        _SLEEPFLAGW { w: self }
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline]
    pub fn dpdflag(&mut self) -> _DPDFLAGW {
        _DPDFLAGW { w: self }
    }
}
