#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSPLLCLKSEL {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "IRC"]
    IRC,
    #[doc = "Crystal Oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR_,
    #[doc = "Reserved."]
    RESERVED_,
    #[doc = "CLKIN. External clock input."]
    CLKIN_EXTERNAL_CLOC,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::IRC => 0,
            SELR::CRYSTAL_OSCILLATOR_ => 1,
            SELR::RESERVED_ => 2,
            SELR::CLKIN_EXTERNAL_CLOC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::IRC,
            1 => SELR::CRYSTAL_OSCILLATOR_,
            2 => SELR::RESERVED_,
            3 => SELR::CLKIN_EXTERNAL_CLOC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline]
    pub fn is_irc(&self) -> bool {
        *self == SELR::IRC
    }
    #[doc = "Checks if the value of the field is `CRYSTAL_OSCILLATOR_`"]
    #[inline]
    pub fn is_crystal_oscillator_(&self) -> bool {
        *self == SELR::CRYSTAL_OSCILLATOR_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline]
    pub fn is_reserved_(&self) -> bool {
        *self == SELR::RESERVED_
    }
    #[doc = "Checks if the value of the field is `CLKIN_EXTERNAL_CLOC`"]
    #[inline]
    pub fn is_clkin_external_cloc(&self) -> bool {
        *self == SELR::CLKIN_EXTERNAL_CLOC
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "IRC"]
    IRC,
    #[doc = "Crystal Oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR_,
    #[doc = "Reserved."]
    RESERVED_,
    #[doc = "CLKIN. External clock input."]
    CLKIN_EXTERNAL_CLOC,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::IRC => 0,
            SELW::CRYSTAL_OSCILLATOR_ => 1,
            SELW::RESERVED_ => 2,
            SELW::CLKIN_EXTERNAL_CLOC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IRC"]
    #[inline]
    pub fn irc(self) -> &'a mut W {
        self.variant(SELW::IRC)
    }
    #[doc = "Crystal Oscillator (SYSOSC)"]
    #[inline]
    pub fn crystal_oscillator_(self) -> &'a mut W {
        self.variant(SELW::CRYSTAL_OSCILLATOR_)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(SELW::RESERVED_)
    }
    #[doc = "CLKIN. External clock input."]
    #[inline]
    pub fn clkin_external_cloc(self) -> &'a mut W {
        self.variant(SELW::CLKIN_EXTERNAL_CLOC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
