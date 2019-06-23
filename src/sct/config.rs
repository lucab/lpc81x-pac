#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYR {
    #[doc = "The SCT operates as two 16-bit counters named L and H."]
    THE_SCT_OPERATES_AS_,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED,
}
impl UNIFYR {
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
            UNIFYR::THE_SCT_OPERATES_AS_ => false,
            UNIFYR::UNIFIED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNIFYR {
        match value {
            false => UNIFYR::THE_SCT_OPERATES_AS_,
            true => UNIFYR::UNIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `THE_SCT_OPERATES_AS_`"]
    #[inline]
    pub fn is_the_sct_operates_as_(&self) -> bool {
        *self == UNIFYR::THE_SCT_OPERATES_AS_
    }
    #[doc = "Checks if the value of the field is `UNIFIED`"]
    #[inline]
    pub fn is_unified(&self) -> bool {
        *self == UNIFYR::UNIFIED
    }
}
#[doc = "Possible values of the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODER {
    #[doc = "The bus clock clocks the SCT and prescalers."]
    THE_BUS_CLOCK_CLOCKS,
    #[doc = "The SCT clock is the bus clock, but the prescalers are  enabled to count only when sampling of the input selected by  the CKSEL field finds the selected edge. The minimum pulse  width on the clock input is 1 bus clock period. This mode is the high-performance  sampled-clock mode."]
    THE_SCT_CLOCK_IS_THE,
    #[doc = "The input selected by  CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted.  The minimum pulse width on the clock input is 1 bus clock  period. This mode is the low-power sampled-clock mode."]
    THE_INPUT_SELECTED_B,
    #[doc = "Reserved."]
    RESERVED_,
}
impl CLKMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKMODER::THE_BUS_CLOCK_CLOCKS => 0,
            CLKMODER::THE_SCT_CLOCK_IS_THE => 1,
            CLKMODER::THE_INPUT_SELECTED_B => 2,
            CLKMODER::RESERVED_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKMODER {
        match value {
            0 => CLKMODER::THE_BUS_CLOCK_CLOCKS,
            1 => CLKMODER::THE_SCT_CLOCK_IS_THE,
            2 => CLKMODER::THE_INPUT_SELECTED_B,
            3 => CLKMODER::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THE_BUS_CLOCK_CLOCKS`"]
    #[inline]
    pub fn is_the_bus_clock_clocks(&self) -> bool {
        *self == CLKMODER::THE_BUS_CLOCK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `THE_SCT_CLOCK_IS_THE`"]
    #[inline]
    pub fn is_the_sct_clock_is_the(&self) -> bool {
        *self == CLKMODER::THE_SCT_CLOCK_IS_THE
    }
    #[doc = "Checks if the value of the field is `THE_INPUT_SELECTED_B`"]
    #[inline]
    pub fn is_the_input_selected_b(&self) -> bool {
        *self == CLKMODER::THE_INPUT_SELECTED_B
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline]
    pub fn is_reserved_(&self) -> bool {
        *self == CLKMODER::RESERVED_
    }
}
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "Rising edges on input 0."]
    RISING_EDGES_ON_INPUT_0,
    #[doc = "Falling edges on input 0."]
    FALLING_EDGES_ON_INPUT_0,
    #[doc = "Rising edges on input 1."]
    RISING_EDGES_ON_INPUT_1,
    #[doc = "Falling edges on input 1."]
    FALLING_EDGES_ON_INPUT_1,
    #[doc = "Rising edges on input 2."]
    RISING_EDGES_ON_INPUT_2,
    #[doc = "Falling edges on input 2."]
    FALLING_EDGES_ON_INPUT_2,
    #[doc = "Rising edges on input 3."]
    RISING_EDGES_ON_INPUT_3,
    #[doc = "Falling edges on input 3."]
    FALLING_EDGES_ON_INPUT_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::RISING_EDGES_ON_INPUT_0 => 0,
            CLKSELR::FALLING_EDGES_ON_INPUT_0 => 1,
            CLKSELR::RISING_EDGES_ON_INPUT_1 => 2,
            CLKSELR::FALLING_EDGES_ON_INPUT_1 => 3,
            CLKSELR::RISING_EDGES_ON_INPUT_2 => 4,
            CLKSELR::FALLING_EDGES_ON_INPUT_2 => 5,
            CLKSELR::RISING_EDGES_ON_INPUT_3 => 6,
            CLKSELR::FALLING_EDGES_ON_INPUT_3 => 7,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::RISING_EDGES_ON_INPUT_0,
            1 => CLKSELR::FALLING_EDGES_ON_INPUT_0,
            2 => CLKSELR::RISING_EDGES_ON_INPUT_1,
            3 => CLKSELR::FALLING_EDGES_ON_INPUT_1,
            4 => CLKSELR::RISING_EDGES_ON_INPUT_2,
            5 => CLKSELR::FALLING_EDGES_ON_INPUT_2,
            6 => CLKSELR::RISING_EDGES_ON_INPUT_3,
            7 => CLKSELR::FALLING_EDGES_ON_INPUT_3,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_0`"]
    #[inline]
    pub fn is_rising_edges_on_input_0(&self) -> bool {
        *self == CLKSELR::RISING_EDGES_ON_INPUT_0
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_0`"]
    #[inline]
    pub fn is_falling_edges_on_input_0(&self) -> bool {
        *self == CLKSELR::FALLING_EDGES_ON_INPUT_0
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_1`"]
    #[inline]
    pub fn is_rising_edges_on_input_1(&self) -> bool {
        *self == CLKSELR::RISING_EDGES_ON_INPUT_1
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_1`"]
    #[inline]
    pub fn is_falling_edges_on_input_1(&self) -> bool {
        *self == CLKSELR::FALLING_EDGES_ON_INPUT_1
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_2`"]
    #[inline]
    pub fn is_rising_edges_on_input_2(&self) -> bool {
        *self == CLKSELR::RISING_EDGES_ON_INPUT_2
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_2`"]
    #[inline]
    pub fn is_falling_edges_on_input_2(&self) -> bool {
        *self == CLKSELR::FALLING_EDGES_ON_INPUT_2
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_3`"]
    #[inline]
    pub fn is_rising_edges_on_input_3(&self) -> bool {
        *self == CLKSELR::RISING_EDGES_ON_INPUT_3
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_3`"]
    #[inline]
    pub fn is_falling_edges_on_input_3(&self) -> bool {
        *self == CLKSELR::FALLING_EDGES_ON_INPUT_3
    }
}
#[doc = r" Value of the field"]
pub struct NORELAOD_LR {
    bits: bool,
}
impl NORELAOD_LR {
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
#[doc = r" Value of the field"]
pub struct NORELOAD_HR {
    bits: bool,
}
impl NORELOAD_HR {
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
#[doc = r" Value of the field"]
pub struct INSYNCR {
    bits: u8,
}
impl INSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AUTOLIMIT_LR {
    bits: bool,
}
impl AUTOLIMIT_LR {
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
#[doc = r" Value of the field"]
pub struct AUTOLIMIT_HR {
    bits: bool,
}
impl AUTOLIMIT_HR {
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
#[doc = "Values that can be written to the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYW {
    #[doc = "The SCT operates as two 16-bit counters named L and H."]
    THE_SCT_OPERATES_AS_,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED,
}
impl UNIFYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNIFYW::THE_SCT_OPERATES_AS_ => false,
            UNIFYW::UNIFIED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNIFYW<'a> {
    w: &'a mut W,
}
impl<'a> _UNIFYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNIFYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SCT operates as two 16-bit counters named L and H."]
    #[inline]
    pub fn the_sct_operates_as_(self) -> &'a mut W {
        self.variant(UNIFYW::THE_SCT_OPERATES_AS_)
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline]
    pub fn unified(self) -> &'a mut W {
        self.variant(UNIFYW::UNIFIED)
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
#[doc = "Values that can be written to the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODEW {
    #[doc = "The bus clock clocks the SCT and prescalers."]
    THE_BUS_CLOCK_CLOCKS,
    #[doc = "The SCT clock is the bus clock, but the prescalers are  enabled to count only when sampling of the input selected by  the CKSEL field finds the selected edge. The minimum pulse  width on the clock input is 1 bus clock period. This mode is the high-performance  sampled-clock mode."]
    THE_SCT_CLOCK_IS_THE,
    #[doc = "The input selected by  CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted.  The minimum pulse width on the clock input is 1 bus clock  period. This mode is the low-power sampled-clock mode."]
    THE_INPUT_SELECTED_B,
    #[doc = "Reserved."]
    RESERVED_,
}
impl CLKMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKMODEW::THE_BUS_CLOCK_CLOCKS => 0,
            CLKMODEW::THE_SCT_CLOCK_IS_THE => 1,
            CLKMODEW::THE_INPUT_SELECTED_B => 2,
            CLKMODEW::RESERVED_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The bus clock clocks the SCT and prescalers."]
    #[inline]
    pub fn the_bus_clock_clocks(self) -> &'a mut W {
        self.variant(CLKMODEW::THE_BUS_CLOCK_CLOCKS)
    }
    #[doc = "The SCT clock is the bus clock, but the prescalers are enabled to count only when sampling of the input selected by the CKSEL field finds the selected edge. The minimum pulse width on the clock input is 1 bus clock period. This mode is the high-performance sampled-clock mode."]
    #[inline]
    pub fn the_sct_clock_is_the(self) -> &'a mut W {
        self.variant(CLKMODEW::THE_SCT_CLOCK_IS_THE)
    }
    #[doc = "The input selected by CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power sampled-clock mode."]
    #[inline]
    pub fn the_input_selected_b(self) -> &'a mut W {
        self.variant(CLKMODEW::THE_INPUT_SELECTED_B)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(CLKMODEW::RESERVED_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELW {
    #[doc = "Rising edges on input 0."]
    RISING_EDGES_ON_INPUT_0,
    #[doc = "Falling edges on input 0."]
    FALLING_EDGES_ON_INPUT_0,
    #[doc = "Rising edges on input 1."]
    RISING_EDGES_ON_INPUT_1,
    #[doc = "Falling edges on input 1."]
    FALLING_EDGES_ON_INPUT_1,
    #[doc = "Rising edges on input 2."]
    RISING_EDGES_ON_INPUT_2,
    #[doc = "Falling edges on input 2."]
    FALLING_EDGES_ON_INPUT_2,
    #[doc = "Rising edges on input 3."]
    RISING_EDGES_ON_INPUT_3,
    #[doc = "Falling edges on input 3."]
    FALLING_EDGES_ON_INPUT_3,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::RISING_EDGES_ON_INPUT_0 => 0,
            CLKSELW::FALLING_EDGES_ON_INPUT_0 => 1,
            CLKSELW::RISING_EDGES_ON_INPUT_1 => 2,
            CLKSELW::FALLING_EDGES_ON_INPUT_1 => 3,
            CLKSELW::RISING_EDGES_ON_INPUT_2 => 4,
            CLKSELW::FALLING_EDGES_ON_INPUT_2 => 5,
            CLKSELW::RISING_EDGES_ON_INPUT_3 => 6,
            CLKSELW::FALLING_EDGES_ON_INPUT_3 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rising edges on input 0."]
    #[inline]
    pub fn rising_edges_on_input_0(self) -> &'a mut W {
        self.variant(CLKSELW::RISING_EDGES_ON_INPUT_0)
    }
    #[doc = "Falling edges on input 0."]
    #[inline]
    pub fn falling_edges_on_input_0(self) -> &'a mut W {
        self.variant(CLKSELW::FALLING_EDGES_ON_INPUT_0)
    }
    #[doc = "Rising edges on input 1."]
    #[inline]
    pub fn rising_edges_on_input_1(self) -> &'a mut W {
        self.variant(CLKSELW::RISING_EDGES_ON_INPUT_1)
    }
    #[doc = "Falling edges on input 1."]
    #[inline]
    pub fn falling_edges_on_input_1(self) -> &'a mut W {
        self.variant(CLKSELW::FALLING_EDGES_ON_INPUT_1)
    }
    #[doc = "Rising edges on input 2."]
    #[inline]
    pub fn rising_edges_on_input_2(self) -> &'a mut W {
        self.variant(CLKSELW::RISING_EDGES_ON_INPUT_2)
    }
    #[doc = "Falling edges on input 2."]
    #[inline]
    pub fn falling_edges_on_input_2(self) -> &'a mut W {
        self.variant(CLKSELW::FALLING_EDGES_ON_INPUT_2)
    }
    #[doc = "Rising edges on input 3."]
    #[inline]
    pub fn rising_edges_on_input_3(self) -> &'a mut W {
        self.variant(CLKSELW::RISING_EDGES_ON_INPUT_3)
    }
    #[doc = "Falling edges on input 3."]
    #[inline]
    pub fn falling_edges_on_input_3(self) -> &'a mut W {
        self.variant(CLKSELW::FALLING_EDGES_ON_INPUT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NORELAOD_LW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELAOD_LW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NORELOAD_HW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELOAD_HW<'a> {
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
#[doc = r" Proxy"]
pub struct _INSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _INSYNCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOLIMIT_LW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_LW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUTOLIMIT_HW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_HW<'a> {
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - SCT operation"]
    #[inline]
    pub fn unify(&self) -> UNIFYR {
        UNIFYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline]
    pub fn clkmode(&self) -> CLKMODER {
        CLKMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:6 - SCT clock select"]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn norelaod_l(&self) -> NORELAOD_LR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NORELAOD_LR { bits }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn noreload_h(&self) -> NORELOAD_HR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NORELOAD_HR { bits }
    }
    #[doc = "Bits 9:16 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,..., bit 16 = input 7). A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is synchronous to the SCT clock, keep its bit 0 for faster response. When the CKMODE field is 1x, the bit in this field, corresponding to the input selected by the CKSEL field, is not used."]
    #[inline]
    pub fn insync(&self) -> INSYNCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSYNCR { bits }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_l(&self) -> AUTOLIMIT_LR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOLIMIT_LR { bits }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_h(&self) -> AUTOLIMIT_HR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOLIMIT_HR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline]
    pub fn unify(&mut self) -> _UNIFYW {
        _UNIFYW { w: self }
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline]
    pub fn clkmode(&mut self) -> _CLKMODEW {
        _CLKMODEW { w: self }
    }
    #[doc = "Bits 3:6 - SCT clock select"]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn norelaod_l(&mut self) -> _NORELAOD_LW {
        _NORELAOD_LW { w: self }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn noreload_h(&mut self) -> _NORELOAD_HW {
        _NORELOAD_HW { w: self }
    }
    #[doc = "Bits 9:16 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,..., bit 16 = input 7). A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is synchronous to the SCT clock, keep its bit 0 for faster response. When the CKMODE field is 1x, the bit in this field, corresponding to the input selected by the CKSEL field, is not used."]
    #[inline]
    pub fn insync(&mut self) -> _INSYNCW {
        _INSYNCW { w: self }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_l(&mut self) -> _AUTOLIMIT_LW {
        _AUTOLIMIT_LW { w: self }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_h(&mut self) -> _AUTOLIMIT_HW {
        _AUTOLIMIT_HW { w: self }
    }
}
