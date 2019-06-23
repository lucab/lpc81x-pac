#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINENABLE0 {
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
#[doc = "Possible values of the field `ACMP_I1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I1_ENR {
    #[doc = "Enable ACMP_I1. This function is enabled on pin PIO0_0."]
    ENABLE_ACMP_I1_THIS,
    #[doc = "Disable ACMP_I1. GPIO function PIO0_0 (default) or any other movable function can be assigned to pin PIO0_0."]
    DISABLE_ACMP_I1_GPI,
}
impl ACMP_I1_ENR {
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
            ACMP_I1_ENR::ENABLE_ACMP_I1_THIS => false,
            ACMP_I1_ENR::DISABLE_ACMP_I1_GPI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I1_ENR {
        match value {
            false => ACMP_I1_ENR::ENABLE_ACMP_I1_THIS,
            true => ACMP_I1_ENR::DISABLE_ACMP_I1_GPI,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_ACMP_I1_THIS`"]
    #[inline]
    pub fn is_enable_acmp_i1_this(&self) -> bool {
        *self == ACMP_I1_ENR::ENABLE_ACMP_I1_THIS
    }
    #[doc = "Checks if the value of the field is `DISABLE_ACMP_I1_GPI`"]
    #[inline]
    pub fn is_disable_acmp_i1_gpi(&self) -> bool {
        *self == ACMP_I1_ENR::DISABLE_ACMP_I1_GPI
    }
}
#[doc = "Possible values of the field `ACMP_I2_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I2_ENR {
    #[doc = "Enable ACMP_I2. This function is enabled on pin PIO0_1."]
    ENABLE_ACMP_I2_THIS,
    #[doc = "Disable ACMP_I2. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin PIO0_1."]
    DISABLE_ACMP_I2_GPI,
}
impl ACMP_I2_ENR {
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
            ACMP_I2_ENR::ENABLE_ACMP_I2_THIS => false,
            ACMP_I2_ENR::DISABLE_ACMP_I2_GPI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I2_ENR {
        match value {
            false => ACMP_I2_ENR::ENABLE_ACMP_I2_THIS,
            true => ACMP_I2_ENR::DISABLE_ACMP_I2_GPI,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_ACMP_I2_THIS`"]
    #[inline]
    pub fn is_enable_acmp_i2_this(&self) -> bool {
        *self == ACMP_I2_ENR::ENABLE_ACMP_I2_THIS
    }
    #[doc = "Checks if the value of the field is `DISABLE_ACMP_I2_GPI`"]
    #[inline]
    pub fn is_disable_acmp_i2_gpi(&self) -> bool {
        *self == ACMP_I2_ENR::DISABLE_ACMP_I2_GPI
    }
}
#[doc = "Possible values of the field `SWCLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCLK_ENR {
    #[doc = "Enable SWCLK. This function is enabled on pin PIO0_3."]
    ENABLE_SWCLK_THIS_F,
    #[doc = "Disable SWCLK. GPIO function PIO0_3 is selected on this pin. Any other movable function can be assigned to pin PIO0_3."]
    DISABLE_SWCLK_GPIO_,
}
impl SWCLK_ENR {
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
            SWCLK_ENR::ENABLE_SWCLK_THIS_F => false,
            SWCLK_ENR::DISABLE_SWCLK_GPIO_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWCLK_ENR {
        match value {
            false => SWCLK_ENR::ENABLE_SWCLK_THIS_F,
            true => SWCLK_ENR::DISABLE_SWCLK_GPIO_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_SWCLK_THIS_F`"]
    #[inline]
    pub fn is_enable_swclk_this_f(&self) -> bool {
        *self == SWCLK_ENR::ENABLE_SWCLK_THIS_F
    }
    #[doc = "Checks if the value of the field is `DISABLE_SWCLK_GPIO_`"]
    #[inline]
    pub fn is_disable_swclk_gpio_(&self) -> bool {
        *self == SWCLK_ENR::DISABLE_SWCLK_GPIO_
    }
}
#[doc = "Possible values of the field `SWDIO_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDIO_ENR {
    #[doc = "Enable SWDIO. This function is enabled on pin PIO0_2."]
    ENABLE_SWDIO_THIS_F,
    #[doc = "Disable SWDIO. GPIO function PIO0_2 is selected on this pin. Any other movable function can be assigned to pin PIO0_2."]
    DISABLE_SWDIO_GPIO_,
}
impl SWDIO_ENR {
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
            SWDIO_ENR::ENABLE_SWDIO_THIS_F => false,
            SWDIO_ENR::DISABLE_SWDIO_GPIO_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWDIO_ENR {
        match value {
            false => SWDIO_ENR::ENABLE_SWDIO_THIS_F,
            true => SWDIO_ENR::DISABLE_SWDIO_GPIO_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_SWDIO_THIS_F`"]
    #[inline]
    pub fn is_enable_swdio_this_f(&self) -> bool {
        *self == SWDIO_ENR::ENABLE_SWDIO_THIS_F
    }
    #[doc = "Checks if the value of the field is `DISABLE_SWDIO_GPIO_`"]
    #[inline]
    pub fn is_disable_swdio_gpio_(&self) -> bool {
        *self == SWDIO_ENR::DISABLE_SWDIO_GPIO_
    }
}
#[doc = "Possible values of the field `XTALIN_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALIN_ENR {
    #[doc = "Enable XTALIN. This function is enabled on pin PIO0_8."]
    ENABLE_XTALIN_THIS_,
    #[doc = "Disable XTALIN. GPIO function PIO0_8 (default) or any other movable function can be assigned to pin PIO0_8."]
    DISABLE_XTALIN_GPIO,
}
impl XTALIN_ENR {
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
            XTALIN_ENR::ENABLE_XTALIN_THIS_ => false,
            XTALIN_ENR::DISABLE_XTALIN_GPIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALIN_ENR {
        match value {
            false => XTALIN_ENR::ENABLE_XTALIN_THIS_,
            true => XTALIN_ENR::DISABLE_XTALIN_GPIO,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_XTALIN_THIS_`"]
    #[inline]
    pub fn is_enable_xtalin_this_(&self) -> bool {
        *self == XTALIN_ENR::ENABLE_XTALIN_THIS_
    }
    #[doc = "Checks if the value of the field is `DISABLE_XTALIN_GPIO`"]
    #[inline]
    pub fn is_disable_xtalin_gpio(&self) -> bool {
        *self == XTALIN_ENR::DISABLE_XTALIN_GPIO
    }
}
#[doc = "Possible values of the field `XTALOUT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOUT_ENR {
    #[doc = "Enable XTALOUT. This function is enabled on pin PIO0_9."]
    ENABLE_XTALOUT_THIS,
    #[doc = "Disable XTALOUT. GPIO function PIO0_9 (default) or any other movable function can be assigned to pin PIO0_9."]
    DISABLE_XTALOUT_GPI,
}
impl XTALOUT_ENR {
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
            XTALOUT_ENR::ENABLE_XTALOUT_THIS => false,
            XTALOUT_ENR::DISABLE_XTALOUT_GPI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALOUT_ENR {
        match value {
            false => XTALOUT_ENR::ENABLE_XTALOUT_THIS,
            true => XTALOUT_ENR::DISABLE_XTALOUT_GPI,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_XTALOUT_THIS`"]
    #[inline]
    pub fn is_enable_xtalout_this(&self) -> bool {
        *self == XTALOUT_ENR::ENABLE_XTALOUT_THIS
    }
    #[doc = "Checks if the value of the field is `DISABLE_XTALOUT_GPI`"]
    #[inline]
    pub fn is_disable_xtalout_gpi(&self) -> bool {
        *self == XTALOUT_ENR::DISABLE_XTALOUT_GPI
    }
}
#[doc = "Possible values of the field `RESET_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_ENR {
    #[doc = "Enable RESET. This function is enabled on pin PIO0_5."]
    ENABLE_RESET_THIS_F,
    #[doc = "Disable RESET. GPIO function PIO0_5 is selected on this pin. Any other movable function can be assigned to pin PIO0_5."]
    DISABLE_RESET_GPIO_,
}
impl RESET_ENR {
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
            RESET_ENR::ENABLE_RESET_THIS_F => false,
            RESET_ENR::DISABLE_RESET_GPIO_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESET_ENR {
        match value {
            false => RESET_ENR::ENABLE_RESET_THIS_F,
            true => RESET_ENR::DISABLE_RESET_GPIO_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_RESET_THIS_F`"]
    #[inline]
    pub fn is_enable_reset_this_f(&self) -> bool {
        *self == RESET_ENR::ENABLE_RESET_THIS_F
    }
    #[doc = "Checks if the value of the field is `DISABLE_RESET_GPIO_`"]
    #[inline]
    pub fn is_disable_reset_gpio_(&self) -> bool {
        *self == RESET_ENR::DISABLE_RESET_GPIO_
    }
}
#[doc = "Possible values of the field `CLKIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKINR {
    #[doc = "Enable CLKIN. This function is enabled on pin PIO0_1."]
    ENABLE_CLKIN_THIS_F,
    #[doc = "Disable CLKIN. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin CLKIN."]
    DISABLE_CLKIN_GPIO_,
}
impl CLKINR {
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
            CLKINR::ENABLE_CLKIN_THIS_F => false,
            CLKINR::DISABLE_CLKIN_GPIO_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKINR {
        match value {
            false => CLKINR::ENABLE_CLKIN_THIS_F,
            true => CLKINR::DISABLE_CLKIN_GPIO_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_CLKIN_THIS_F`"]
    #[inline]
    pub fn is_enable_clkin_this_f(&self) -> bool {
        *self == CLKINR::ENABLE_CLKIN_THIS_F
    }
    #[doc = "Checks if the value of the field is `DISABLE_CLKIN_GPIO_`"]
    #[inline]
    pub fn is_disable_clkin_gpio_(&self) -> bool {
        *self == CLKINR::DISABLE_CLKIN_GPIO_
    }
}
#[doc = "Possible values of the field `VDDCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCMPR {
    #[doc = "Enable VDDCMP. This function is enabled on pin PIO0_6."]
    ENABLE_VDDCMP_THIS_,
    #[doc = "Disable VDDCMP. GPIO function PIO0_6 (default) or any other movable function can be assigned to pin PIO0_6."]
    DISABLE_VDDCMP_GPIO,
}
impl VDDCMPR {
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
            VDDCMPR::ENABLE_VDDCMP_THIS_ => false,
            VDDCMPR::DISABLE_VDDCMP_GPIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDCMPR {
        match value {
            false => VDDCMPR::ENABLE_VDDCMP_THIS_,
            true => VDDCMPR::DISABLE_VDDCMP_GPIO,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_VDDCMP_THIS_`"]
    #[inline]
    pub fn is_enable_vddcmp_this_(&self) -> bool {
        *self == VDDCMPR::ENABLE_VDDCMP_THIS_
    }
    #[doc = "Checks if the value of the field is `DISABLE_VDDCMP_GPIO`"]
    #[inline]
    pub fn is_disable_vddcmp_gpio(&self) -> bool {
        *self == VDDCMPR::DISABLE_VDDCMP_GPIO
    }
}
#[doc = "Values that can be written to the field `ACMP_I1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I1_ENW {
    #[doc = "Enable ACMP_I1. This function is enabled on pin PIO0_0."]
    ENABLE_ACMP_I1_THIS,
    #[doc = "Disable ACMP_I1. GPIO function PIO0_0 (default) or any other movable function can be assigned to pin PIO0_0."]
    DISABLE_ACMP_I1_GPI,
}
impl ACMP_I1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I1_ENW::ENABLE_ACMP_I1_THIS => false,
            ACMP_I1_ENW::DISABLE_ACMP_I1_GPI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_I1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_I1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_I1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable ACMP_I1. This function is enabled on pin PIO0_0."]
    #[inline]
    pub fn enable_acmp_i1_this(self) -> &'a mut W {
        self.variant(ACMP_I1_ENW::ENABLE_ACMP_I1_THIS)
    }
    #[doc = "Disable ACMP_I1. GPIO function PIO0_0 (default) or any other movable function can be assigned to pin PIO0_0."]
    #[inline]
    pub fn disable_acmp_i1_gpi(self) -> &'a mut W {
        self.variant(ACMP_I1_ENW::DISABLE_ACMP_I1_GPI)
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
#[doc = "Values that can be written to the field `ACMP_I2_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I2_ENW {
    #[doc = "Enable ACMP_I2. This function is enabled on pin PIO0_1."]
    ENABLE_ACMP_I2_THIS,
    #[doc = "Disable ACMP_I2. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin PIO0_1."]
    DISABLE_ACMP_I2_GPI,
}
impl ACMP_I2_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I2_ENW::ENABLE_ACMP_I2_THIS => false,
            ACMP_I2_ENW::DISABLE_ACMP_I2_GPI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_I2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_I2_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_I2_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable ACMP_I2. This function is enabled on pin PIO0_1."]
    #[inline]
    pub fn enable_acmp_i2_this(self) -> &'a mut W {
        self.variant(ACMP_I2_ENW::ENABLE_ACMP_I2_THIS)
    }
    #[doc = "Disable ACMP_I2. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin PIO0_1."]
    #[inline]
    pub fn disable_acmp_i2_gpi(self) -> &'a mut W {
        self.variant(ACMP_I2_ENW::DISABLE_ACMP_I2_GPI)
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
#[doc = "Values that can be written to the field `SWCLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCLK_ENW {
    #[doc = "Enable SWCLK. This function is enabled on pin PIO0_3."]
    ENABLE_SWCLK_THIS_F,
    #[doc = "Disable SWCLK. GPIO function PIO0_3 is selected on this pin. Any other movable function can be assigned to pin PIO0_3."]
    DISABLE_SWCLK_GPIO_,
}
impl SWCLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWCLK_ENW::ENABLE_SWCLK_THIS_F => false,
            SWCLK_ENW::DISABLE_SWCLK_GPIO_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWCLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWCLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWCLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable SWCLK. This function is enabled on pin PIO0_3."]
    #[inline]
    pub fn enable_swclk_this_f(self) -> &'a mut W {
        self.variant(SWCLK_ENW::ENABLE_SWCLK_THIS_F)
    }
    #[doc = "Disable SWCLK. GPIO function PIO0_3 is selected on this pin. Any other movable function can be assigned to pin PIO0_3."]
    #[inline]
    pub fn disable_swclk_gpio_(self) -> &'a mut W {
        self.variant(SWCLK_ENW::DISABLE_SWCLK_GPIO_)
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
#[doc = "Values that can be written to the field `SWDIO_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDIO_ENW {
    #[doc = "Enable SWDIO. This function is enabled on pin PIO0_2."]
    ENABLE_SWDIO_THIS_F,
    #[doc = "Disable SWDIO. GPIO function PIO0_2 is selected on this pin. Any other movable function can be assigned to pin PIO0_2."]
    DISABLE_SWDIO_GPIO_,
}
impl SWDIO_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWDIO_ENW::ENABLE_SWDIO_THIS_F => false,
            SWDIO_ENW::DISABLE_SWDIO_GPIO_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWDIO_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWDIO_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWDIO_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable SWDIO. This function is enabled on pin PIO0_2."]
    #[inline]
    pub fn enable_swdio_this_f(self) -> &'a mut W {
        self.variant(SWDIO_ENW::ENABLE_SWDIO_THIS_F)
    }
    #[doc = "Disable SWDIO. GPIO function PIO0_2 is selected on this pin. Any other movable function can be assigned to pin PIO0_2."]
    #[inline]
    pub fn disable_swdio_gpio_(self) -> &'a mut W {
        self.variant(SWDIO_ENW::DISABLE_SWDIO_GPIO_)
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
#[doc = "Values that can be written to the field `XTALIN_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALIN_ENW {
    #[doc = "Enable XTALIN. This function is enabled on pin PIO0_8."]
    ENABLE_XTALIN_THIS_,
    #[doc = "Disable XTALIN. GPIO function PIO0_8 (default) or any other movable function can be assigned to pin PIO0_8."]
    DISABLE_XTALIN_GPIO,
}
impl XTALIN_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALIN_ENW::ENABLE_XTALIN_THIS_ => false,
            XTALIN_ENW::DISABLE_XTALIN_GPIO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALIN_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALIN_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable XTALIN. This function is enabled on pin PIO0_8."]
    #[inline]
    pub fn enable_xtalin_this_(self) -> &'a mut W {
        self.variant(XTALIN_ENW::ENABLE_XTALIN_THIS_)
    }
    #[doc = "Disable XTALIN. GPIO function PIO0_8 (default) or any other movable function can be assigned to pin PIO0_8."]
    #[inline]
    pub fn disable_xtalin_gpio(self) -> &'a mut W {
        self.variant(XTALIN_ENW::DISABLE_XTALIN_GPIO)
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
#[doc = "Values that can be written to the field `XTALOUT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOUT_ENW {
    #[doc = "Enable XTALOUT. This function is enabled on pin PIO0_9."]
    ENABLE_XTALOUT_THIS,
    #[doc = "Disable XTALOUT. GPIO function PIO0_9 (default) or any other movable function can be assigned to pin PIO0_9."]
    DISABLE_XTALOUT_GPI,
}
impl XTALOUT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALOUT_ENW::ENABLE_XTALOUT_THIS => false,
            XTALOUT_ENW::DISABLE_XTALOUT_GPI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALOUT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALOUT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALOUT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable XTALOUT. This function is enabled on pin PIO0_9."]
    #[inline]
    pub fn enable_xtalout_this(self) -> &'a mut W {
        self.variant(XTALOUT_ENW::ENABLE_XTALOUT_THIS)
    }
    #[doc = "Disable XTALOUT. GPIO function PIO0_9 (default) or any other movable function can be assigned to pin PIO0_9."]
    #[inline]
    pub fn disable_xtalout_gpi(self) -> &'a mut W {
        self.variant(XTALOUT_ENW::DISABLE_XTALOUT_GPI)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESET_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_ENW {
    #[doc = "Enable RESET. This function is enabled on pin PIO0_5."]
    ENABLE_RESET_THIS_F,
    #[doc = "Disable RESET. GPIO function PIO0_5 is selected on this pin. Any other movable function can be assigned to pin PIO0_5."]
    DISABLE_RESET_GPIO_,
}
impl RESET_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESET_ENW::ENABLE_RESET_THIS_F => false,
            RESET_ENW::DISABLE_RESET_GPIO_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable RESET. This function is enabled on pin PIO0_5."]
    #[inline]
    pub fn enable_reset_this_f(self) -> &'a mut W {
        self.variant(RESET_ENW::ENABLE_RESET_THIS_F)
    }
    #[doc = "Disable RESET. GPIO function PIO0_5 is selected on this pin. Any other movable function can be assigned to pin PIO0_5."]
    #[inline]
    pub fn disable_reset_gpio_(self) -> &'a mut W {
        self.variant(RESET_ENW::DISABLE_RESET_GPIO_)
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
#[doc = "Values that can be written to the field `CLKIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKINW {
    #[doc = "Enable CLKIN. This function is enabled on pin PIO0_1."]
    ENABLE_CLKIN_THIS_F,
    #[doc = "Disable CLKIN. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin CLKIN."]
    DISABLE_CLKIN_GPIO_,
}
impl CLKINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKINW::ENABLE_CLKIN_THIS_F => false,
            CLKINW::DISABLE_CLKIN_GPIO_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKINW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable CLKIN. This function is enabled on pin PIO0_1."]
    #[inline]
    pub fn enable_clkin_this_f(self) -> &'a mut W {
        self.variant(CLKINW::ENABLE_CLKIN_THIS_F)
    }
    #[doc = "Disable CLKIN. GPIO function PIO0_1 (default) or any other movable function can be assigned to pin CLKIN."]
    #[inline]
    pub fn disable_clkin_gpio_(self) -> &'a mut W {
        self.variant(CLKINW::DISABLE_CLKIN_GPIO_)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VDDCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCMPW {
    #[doc = "Enable VDDCMP. This function is enabled on pin PIO0_6."]
    ENABLE_VDDCMP_THIS_,
    #[doc = "Disable VDDCMP. GPIO function PIO0_6 (default) or any other movable function can be assigned to pin PIO0_6."]
    DISABLE_VDDCMP_GPIO,
}
impl VDDCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VDDCMPW::ENABLE_VDDCMP_THIS_ => false,
            VDDCMPW::DISABLE_VDDCMP_GPIO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDDCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDDCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable VDDCMP. This function is enabled on pin PIO0_6."]
    #[inline]
    pub fn enable_vddcmp_this_(self) -> &'a mut W {
        self.variant(VDDCMPW::ENABLE_VDDCMP_THIS_)
    }
    #[doc = "Disable VDDCMP. GPIO function PIO0_6 (default) or any other movable function can be assigned to pin PIO0_6."]
    #[inline]
    pub fn disable_vddcmp_gpio(self) -> &'a mut W {
        self.variant(VDDCMPW::DISABLE_VDDCMP_GPIO)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn acmp_i1_en(&self) -> ACMP_I1_ENR {
        ACMP_I1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use ACMP_I2, disable the CLKIN function in bit 7 of this register and enable ACMP_I2."]
    #[inline]
    pub fn acmp_i2_en(&self) -> ACMP_I2_ENR {
        ACMP_I2_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline]
    pub fn swclk_en(&self) -> SWCLK_ENR {
        SWCLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline]
    pub fn swdio_en(&self) -> SWDIO_ENR {
        SWDIO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn xtalin_en(&self) -> XTALIN_ENR {
        XTALIN_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn xtalout_en(&self) -> XTALOUT_ENR {
        XTALOUT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline]
    pub fn reset_en(&self) -> RESET_ENR {
        RESET_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use CLKIN, disable ACMP_I2 in bit 1 of this register and enable CLKIN."]
    #[inline]
    pub fn clkin(&self) -> CLKINR {
        CLKINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn vddcmp(&self) -> VDDCMPR {
        VDDCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 435 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn acmp_i1_en(&mut self) -> _ACMP_I1_ENW {
        _ACMP_I1_ENW { w: self }
    }
    #[doc = "Bit 1 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use ACMP_I2, disable the CLKIN function in bit 7 of this register and enable ACMP_I2."]
    #[inline]
    pub fn acmp_i2_en(&mut self) -> _ACMP_I2_ENW {
        _ACMP_I2_ENW { w: self }
    }
    #[doc = "Bit 2 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline]
    pub fn swclk_en(&mut self) -> _SWCLK_ENW {
        _SWCLK_ENW { w: self }
    }
    #[doc = "Bit 3 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline]
    pub fn swdio_en(&mut self) -> _SWDIO_ENW {
        _SWDIO_ENW { w: self }
    }
    #[doc = "Bit 4 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn xtalin_en(&mut self) -> _XTALIN_ENW {
        _XTALIN_ENW { w: self }
    }
    #[doc = "Bit 5 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn xtalout_en(&mut self) -> _XTALOUT_ENW {
        _XTALOUT_ENW { w: self }
    }
    #[doc = "Bit 6 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. This function is selected by default."]
    #[inline]
    pub fn reset_en(&mut self) -> _RESET_ENW {
        _RESET_ENW { w: self }
    }
    #[doc = "Bit 7 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed-pin function is deselected and GPIO is assigned to this pin. Functions CLKIN and ACMP_I2 are connected to the same pin PIO0_1. To use CLKIN, disable ACMP_I2 in bit 1 of this register and enable CLKIN."]
    #[inline]
    pub fn clkin(&mut self) -> _CLKINW {
        _CLKINW { w: self }
    }
    #[doc = "Bit 8 - Enables fixed-pin function. Writing a 1 deselects the function and any movable function can be assigned to this pin. By default the fixed--pin function is deselected and GPIO is assigned to this pin."]
    #[inline]
    pub fn vddcmp(&mut self) -> _VDDCMPW {
        _VDDCMPW { w: self }
    }
}
