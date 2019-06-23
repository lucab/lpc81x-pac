#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQ_FLAG {
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
#[doc = "Possible values of the field `GFLAG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG0R {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG0R {
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
            GFLAG0R::NO_PENDING_INTERRUPT => false,
            GFLAG0R::PENDING_INTERRUPT_T => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GFLAG0R {
        match value {
            false => GFLAG0R::NO_PENDING_INTERRUPT,
            true => GFLAG0R::PENDING_INTERRUPT_T,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GFLAG0R::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT_T`"]
    #[inline]
    pub fn is_pending_interrupt_t(&self) -> bool {
        *self == GFLAG0R::PENDING_INTERRUPT_T
    }
}
#[doc = "Possible values of the field `GFLAG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG1R {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER1 has reached the end of the time interval. If the INTEN bit in the CONTROL1 register is also set to 1, the interrupt for timer channel 1 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG1R {
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
            GFLAG1R::NO_PENDING_INTERRUPT => false,
            GFLAG1R::PENDING_INTERRUPT_T => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GFLAG1R {
        match value {
            false => GFLAG1R::NO_PENDING_INTERRUPT,
            true => GFLAG1R::PENDING_INTERRUPT_T,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GFLAG1R::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT_T`"]
    #[inline]
    pub fn is_pending_interrupt_t(&self) -> bool {
        *self == GFLAG1R::PENDING_INTERRUPT_T
    }
}
#[doc = "Possible values of the field `GFLAG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG2R {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER2 has reached the end of the time interval. If the INTEN bit in the CONTROL2 register is also set to 1, the interrupt for timer channel 2 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG2R {
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
            GFLAG2R::NO_PENDING_INTERRUPT => false,
            GFLAG2R::PENDING_INTERRUPT_T => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GFLAG2R {
        match value {
            false => GFLAG2R::NO_PENDING_INTERRUPT,
            true => GFLAG2R::PENDING_INTERRUPT_T,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GFLAG2R::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT_T`"]
    #[inline]
    pub fn is_pending_interrupt_t(&self) -> bool {
        *self == GFLAG2R::PENDING_INTERRUPT_T
    }
}
#[doc = "Possible values of the field `GFLAG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG3R {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER3 has reached the end of the time interval. If the INTEN bit in the CONTROL3 register is also set to 1, the interrupt for timer channel 3 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG3R {
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
            GFLAG3R::NO_PENDING_INTERRUPT => false,
            GFLAG3R::PENDING_INTERRUPT_T => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GFLAG3R {
        match value {
            false => GFLAG3R::NO_PENDING_INTERRUPT,
            true => GFLAG3R::PENDING_INTERRUPT_T,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GFLAG3R::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT_T`"]
    #[inline]
    pub fn is_pending_interrupt_t(&self) -> bool {
        *self == GFLAG3R::PENDING_INTERRUPT_T
    }
}
#[doc = "Values that can be written to the field `GFLAG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG0W {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GFLAG0W::NO_PENDING_INTERRUPT => false,
            GFLAG0W::PENDING_INTERRUPT_T => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFLAG0W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFLAG0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG0W::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline]
    pub fn pending_interrupt_t(self) -> &'a mut W {
        self.variant(GFLAG0W::PENDING_INTERRUPT_T)
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
#[doc = "Values that can be written to the field `GFLAG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG1W {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER1 has reached the end of the time interval. If the INTEN bit in the CONTROL1 register is also set to 1, the interrupt for timer channel 1 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GFLAG1W::NO_PENDING_INTERRUPT => false,
            GFLAG1W::PENDING_INTERRUPT_T => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFLAG1W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFLAG1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG1W::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER1 has reached the end of the time interval. If the INTEN bit in the CONTROL1 register is also set to 1, the interrupt for timer channel 1 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline]
    pub fn pending_interrupt_t(self) -> &'a mut W {
        self.variant(GFLAG1W::PENDING_INTERRUPT_T)
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
#[doc = "Values that can be written to the field `GFLAG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG2W {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER2 has reached the end of the time interval. If the INTEN bit in the CONTROL2 register is also set to 1, the interrupt for timer channel 2 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GFLAG2W::NO_PENDING_INTERRUPT => false,
            GFLAG2W::PENDING_INTERRUPT_T => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFLAG2W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFLAG2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG2W::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER2 has reached the end of the time interval. If the INTEN bit in the CONTROL2 register is also set to 1, the interrupt for timer channel 2 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline]
    pub fn pending_interrupt_t(self) -> &'a mut W {
        self.variant(GFLAG2W::PENDING_INTERRUPT_T)
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
#[doc = "Values that can be written to the field `GFLAG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG3W {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER3 has reached the end of the time interval. If the INTEN bit in the CONTROL3 register is also set to 1, the interrupt for timer channel 3 and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl GFLAG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GFLAG3W::NO_PENDING_INTERRUPT => false,
            GFLAG3W::PENDING_INTERRUPT_T => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFLAG3W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFLAG3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG3W::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER3 has reached the end of the time interval. If the INTEN bit in the CONTROL3 register is also set to 1, the interrupt for timer channel 3 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline]
    pub fn pending_interrupt_t(self) -> &'a mut W {
        self.variant(GFLAG3W::PENDING_INTERRUPT_T)
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
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline]
    pub fn gflag0(&self) -> GFLAG0R {
        GFLAG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1."]
    #[inline]
    pub fn gflag1(&self) -> GFLAG1R {
        GFLAG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2."]
    #[inline]
    pub fn gflag2(&self) -> GFLAG2R {
        GFLAG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3."]
    #[inline]
    pub fn gflag3(&self) -> GFLAG3R {
        GFLAG3R::_from({
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
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline]
    pub fn gflag0(&mut self) -> _GFLAG0W {
        _GFLAG0W { w: self }
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1."]
    #[inline]
    pub fn gflag1(&mut self) -> _GFLAG1W {
        _GFLAG1W { w: self }
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2."]
    #[inline]
    pub fn gflag2(&mut self) -> _GFLAG2W {
        _GFLAG2W { w: self }
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3."]
    #[inline]
    pub fn gflag3(&mut self) -> _GFLAG3W {
        _GFLAG3W { w: self }
    }
}
