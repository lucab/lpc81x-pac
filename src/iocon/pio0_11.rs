#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO0_11 {
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
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED_,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVR {
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
            INVR::INPUT_NOT_INVERTED_ => false,
            INVR::INPUT_INVERTED_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::INPUT_NOT_INVERTED_,
            true => INVR::INPUT_INVERTED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED_`"]
    #[inline]
    pub fn is_input_not_inverted_(&self) -> bool {
        *self == INVR::INPUT_NOT_INVERTED_
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INVR::INPUT_INVERTED_HIGH
    }
}
#[doc = "Possible values of the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODER {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARD_MODE,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
    #[doc = "Reserved."]
    RESERVED_,
}
impl I2CMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CMODER::STANDARD_MODE => 0,
            I2CMODER::STANDARD_IO => 1,
            I2CMODER::FAST_MODE_PLUS_I2C => 2,
            I2CMODER::RESERVED_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CMODER {
        match value {
            0 => I2CMODER::STANDARD_MODE,
            1 => I2CMODER::STANDARD_IO,
            2 => I2CMODER::FAST_MODE_PLUS_I2C,
            3 => I2CMODER::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE`"]
    #[inline]
    pub fn is_standard_mode(&self) -> bool {
        *self == I2CMODER::STANDARD_MODE
    }
    #[doc = "Checks if the value of the field is `STANDARD_IO`"]
    #[inline]
    pub fn is_standard_io(&self) -> bool {
        *self == I2CMODER::STANDARD_IO
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_I2C`"]
    #[inline]
    pub fn is_fast_mode_plus_i2c(&self) -> bool {
        *self == I2CMODER::FAST_MODE_PLUS_I2C
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline]
    pub fn is_reserved_(&self) -> bool {
        *self == I2CMODER::RESERVED_
    }
}
#[doc = "Possible values of the field `S_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_MODER {
    #[doc = "Bypass input filter."]
    BYPASS_INPUT_FILTER_,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    _1_CLOCK_CYCLE_INPUT,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    _2_CLOCK_CYCLES_INPU,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    _3_CLOCK_CYCLES_INPU,
}
impl S_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S_MODER::BYPASS_INPUT_FILTER_ => 0,
            S_MODER::_1_CLOCK_CYCLE_INPUT => 1,
            S_MODER::_2_CLOCK_CYCLES_INPU => 2,
            S_MODER::_3_CLOCK_CYCLES_INPU => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S_MODER {
        match value {
            0 => S_MODER::BYPASS_INPUT_FILTER_,
            1 => S_MODER::_1_CLOCK_CYCLE_INPUT,
            2 => S_MODER::_2_CLOCK_CYCLES_INPU,
            3 => S_MODER::_3_CLOCK_CYCLES_INPU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS_INPUT_FILTER_`"]
    #[inline]
    pub fn is_bypass_input_filter_(&self) -> bool {
        *self == S_MODER::BYPASS_INPUT_FILTER_
    }
    #[doc = "Checks if the value of the field is `_1_CLOCK_CYCLE_INPUT`"]
    #[inline]
    pub fn is_1_clock_cycle_input(&self) -> bool {
        *self == S_MODER::_1_CLOCK_CYCLE_INPUT
    }
    #[doc = "Checks if the value of the field is `_2_CLOCK_CYCLES_INPU`"]
    #[inline]
    pub fn is_2_clock_cycles_inpu(&self) -> bool {
        *self == S_MODER::_2_CLOCK_CYCLES_INPU
    }
    #[doc = "Checks if the value of the field is `_3_CLOCK_CYCLES_INPU`"]
    #[inline]
    pub fn is_3_clock_cycles_inpu(&self) -> bool {
        *self == S_MODER::_3_CLOCK_CYCLES_INPU
    }
}
#[doc = "Possible values of the field `CLK_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DIVR {
    #[doc = "IOCONFILTRCLKDIV0."]
    IOCONFILTRCLKDIV0_,
    #[doc = "IOCONFILTRCLKDIV1."]
    IOCONFILTRCLKDIV1_,
    #[doc = "IOCONFILTRCLKDIV2."]
    IOCONFILTRCLKDIV2_,
    #[doc = "IOCONFILTRCLKDIV3."]
    IOCONFILTRCLKDIV3_,
    #[doc = "IOCONFILTRCLKDIV4."]
    IOCONFILTRCLKDIV4_,
    #[doc = "IOCONFILTRCLKDIV5."]
    IOCONFILTRCLKDIV5_,
    #[doc = "IOCONFILTRCLKDIV6."]
    IOCONFILTRCLKDIV6_,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_DIVR::IOCONFILTRCLKDIV0_ => 0,
            CLK_DIVR::IOCONFILTRCLKDIV1_ => 1,
            CLK_DIVR::IOCONFILTRCLKDIV2_ => 2,
            CLK_DIVR::IOCONFILTRCLKDIV3_ => 3,
            CLK_DIVR::IOCONFILTRCLKDIV4_ => 4,
            CLK_DIVR::IOCONFILTRCLKDIV5_ => 5,
            CLK_DIVR::IOCONFILTRCLKDIV6_ => 6,
            CLK_DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_DIVR {
        match value {
            0 => CLK_DIVR::IOCONFILTRCLKDIV0_,
            1 => CLK_DIVR::IOCONFILTRCLKDIV1_,
            2 => CLK_DIVR::IOCONFILTRCLKDIV2_,
            3 => CLK_DIVR::IOCONFILTRCLKDIV3_,
            4 => CLK_DIVR::IOCONFILTRCLKDIV4_,
            5 => CLK_DIVR::IOCONFILTRCLKDIV5_,
            6 => CLK_DIVR::IOCONFILTRCLKDIV6_,
            i => CLK_DIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IOCONFILTRCLKDIV0_`"]
    #[inline]
    pub fn is_ioconfiltrclkdiv0_(&self) -> bool {
        *self == CLK_DIVR::IOCONFILTRCLKDIV0_
    }
    #[doc = "Checks if the value of the field is `IOCONFILTRCLKDIV1_`"]
    #[inline]
    pub fn is_ioconfiltrclkdiv1_(&self) -> bool {
        *self == CLK_DIVR::IOCONFILTRCLKDIV1_
    }
    #[doc = "Checks if the value of the field is `IOCONFILTRCLKDIV2_`"]
    #[inline]
    pub fn is_ioconfiltrclkdiv2_(&self) -> bool {
        *self == CLK_DIVR::IOCONFILTRCLKDIV2_
    }
    #[doc = "Checks if the value of the field is `IOCONFILTRCLKDIV3_`"]
    #[inline]
    pub fn is_ioconfiltrclkdiv3_(&self) -> bool {
        *self == CLK_DIVR::IOCONFILTRCLKDIV3_
    }
    #[doc = "Checks if the value of the field is `IOCONFILTRCLKDIV4_`"]
    #[inline]
    pub fn is_ioconfiltrclkdiv4_(&self) -> bool {
        *self == CLK_DIVR::IOCONFILTRCLKDIV4_
    }
    #[doc = "Checks if the value of the field is `IOCONFILTRCLKDIV5_`"]
    #[inline]
    pub fn is_ioconfiltrclkdiv5_(&self) -> bool {
        *self == CLK_DIVR::IOCONFILTRCLKDIV5_
    }
    #[doc = "Checks if the value of the field is `IOCONFILTRCLKDIV6_`"]
    #[inline]
    pub fn is_ioconfiltrclkdiv6_(&self) -> bool {
        *self == CLK_DIVR::IOCONFILTRCLKDIV6_
    }
}
#[doc = "Values that can be written to the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED_,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INPUT_NOT_INVERTED_ => false,
            INVW::INPUT_INVERTED_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    #[inline]
    pub fn input_not_inverted_(self) -> &'a mut W {
        self.variant(INVW::INPUT_NOT_INVERTED_)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline]
    pub fn input_inverted_high(self) -> &'a mut W {
        self.variant(INVW::INPUT_INVERTED_HIGH)
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
#[doc = "Values that can be written to the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODEW {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARD_MODE,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
    #[doc = "Reserved."]
    RESERVED_,
}
impl I2CMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2CMODEW::STANDARD_MODE => 0,
            I2CMODEW::STANDARD_IO => 1,
            I2CMODEW::FAST_MODE_PLUS_I2C => 2,
            I2CMODEW::RESERVED_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2CMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Standard mode/ Fast-mode I2C."]
    #[inline]
    pub fn standard_mode(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_MODE)
    }
    #[doc = "Standard I/O functionality"]
    #[inline]
    pub fn standard_io(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_IO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline]
    pub fn fast_mode_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODEW::FAST_MODE_PLUS_I2C)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(I2CMODEW::RESERVED_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_MODEW {
    #[doc = "Bypass input filter."]
    BYPASS_INPUT_FILTER_,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    _1_CLOCK_CYCLE_INPUT,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    _2_CLOCK_CYCLES_INPU,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    _3_CLOCK_CYCLES_INPU,
}
impl S_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S_MODEW::BYPASS_INPUT_FILTER_ => 0,
            S_MODEW::_1_CLOCK_CYCLE_INPUT => 1,
            S_MODEW::_2_CLOCK_CYCLES_INPU => 2,
            S_MODEW::_3_CLOCK_CYCLES_INPU => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass input filter."]
    #[inline]
    pub fn bypass_input_filter_(self) -> &'a mut W {
        self.variant(S_MODEW::BYPASS_INPUT_FILTER_)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline]
    pub fn _1_clock_cycle_input(self) -> &'a mut W {
        self.variant(S_MODEW::_1_CLOCK_CYCLE_INPUT)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline]
    pub fn _2_clock_cycles_inpu(self) -> &'a mut W {
        self.variant(S_MODEW::_2_CLOCK_CYCLES_INPU)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline]
    pub fn _3_clock_cycles_inpu(self) -> &'a mut W {
        self.variant(S_MODEW::_3_CLOCK_CYCLES_INPU)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DIVW {
    #[doc = "IOCONFILTRCLKDIV0."]
    IOCONFILTRCLKDIV0_,
    #[doc = "IOCONFILTRCLKDIV1."]
    IOCONFILTRCLKDIV1_,
    #[doc = "IOCONFILTRCLKDIV2."]
    IOCONFILTRCLKDIV2_,
    #[doc = "IOCONFILTRCLKDIV3."]
    IOCONFILTRCLKDIV3_,
    #[doc = "IOCONFILTRCLKDIV4."]
    IOCONFILTRCLKDIV4_,
    #[doc = "IOCONFILTRCLKDIV5."]
    IOCONFILTRCLKDIV5_,
    #[doc = "IOCONFILTRCLKDIV6."]
    IOCONFILTRCLKDIV6_,
}
impl CLK_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_DIVW::IOCONFILTRCLKDIV0_ => 0,
            CLK_DIVW::IOCONFILTRCLKDIV1_ => 1,
            CLK_DIVW::IOCONFILTRCLKDIV2_ => 2,
            CLK_DIVW::IOCONFILTRCLKDIV3_ => 3,
            CLK_DIVW::IOCONFILTRCLKDIV4_ => 4,
            CLK_DIVW::IOCONFILTRCLKDIV5_ => 5,
            CLK_DIVW::IOCONFILTRCLKDIV6_ => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_DIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IOCONFILTRCLKDIV0."]
    #[inline]
    pub fn ioconfiltrclkdiv0_(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONFILTRCLKDIV0_)
    }
    #[doc = "IOCONFILTRCLKDIV1."]
    #[inline]
    pub fn ioconfiltrclkdiv1_(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONFILTRCLKDIV1_)
    }
    #[doc = "IOCONFILTRCLKDIV2."]
    #[inline]
    pub fn ioconfiltrclkdiv2_(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONFILTRCLKDIV2_)
    }
    #[doc = "IOCONFILTRCLKDIV3."]
    #[inline]
    pub fn ioconfiltrclkdiv3_(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONFILTRCLKDIV3_)
    }
    #[doc = "IOCONFILTRCLKDIV4."]
    #[inline]
    pub fn ioconfiltrclkdiv4_(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONFILTRCLKDIV4_)
    }
    #[doc = "IOCONFILTRCLKDIV5."]
    #[inline]
    pub fn ioconfiltrclkdiv5_(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONFILTRCLKDIV5_)
    }
    #[doc = "IOCONFILTRCLKDIV6."]
    #[inline]
    pub fn ioconfiltrclkdiv6_(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONFILTRCLKDIV6_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline]
    pub fn i2cmode(&self) -> I2CMODER {
        I2CMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline]
    pub fn s_mode(&self) -> S_MODER {
        S_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline]
    pub fn clk_div(&self) -> CLK_DIVR {
        CLK_DIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline]
    pub fn i2cmode(&mut self) -> _I2CMODEW {
        _I2CMODEW { w: self }
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline]
    pub fn s_mode(&mut self) -> _S_MODEW {
        _S_MODEW { w: self }
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline]
    pub fn clk_div(&mut self) -> _CLK_DIVW {
        _CLK_DIVW { w: self }
    }
}
