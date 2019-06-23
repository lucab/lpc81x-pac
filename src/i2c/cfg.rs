#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `MSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTENR {
    #[doc = "Disabled. The I2C Master function is disabled."]
    DISABLED_THE_I2C_MA,
    #[doc = "Enabled. The I2C Master function is enabled."]
    ENABLED_THE_I2C_MAS,
}
impl MSTENR {
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
            MSTENR::DISABLED_THE_I2C_MA => false,
            MSTENR::ENABLED_THE_I2C_MAS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTENR {
        match value {
            false => MSTENR::DISABLED_THE_I2C_MA,
            true => MSTENR::ENABLED_THE_I2C_MAS,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THE_I2C_MA`"]
    #[inline]
    pub fn is_disabled_the_i2c_ma(&self) -> bool {
        *self == MSTENR::DISABLED_THE_I2C_MA
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_I2C_MAS`"]
    #[inline]
    pub fn is_enabled_the_i2c_mas(&self) -> bool {
        *self == MSTENR::ENABLED_THE_I2C_MAS
    }
}
#[doc = "Possible values of the field `SLVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVENR {
    #[doc = "Disabled. The I2C slave function is disabled."]
    DISABLED_THE_I2C_SL,
    #[doc = "Enabled. The I2C slave function is enabled."]
    ENABLED_THE_I2C_SLA,
}
impl SLVENR {
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
            SLVENR::DISABLED_THE_I2C_SL => false,
            SLVENR::ENABLED_THE_I2C_SLA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVENR {
        match value {
            false => SLVENR::DISABLED_THE_I2C_SL,
            true => SLVENR::ENABLED_THE_I2C_SLA,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THE_I2C_SL`"]
    #[inline]
    pub fn is_disabled_the_i2c_sl(&self) -> bool {
        *self == SLVENR::DISABLED_THE_I2C_SL
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_I2C_SLA`"]
    #[inline]
    pub fn is_enabled_the_i2c_sla(&self) -> bool {
        *self == SLVENR::ENABLED_THE_I2C_SLA
    }
}
#[doc = "Possible values of the field `MONEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONENR {
    #[doc = "Disabled. The I2C monitor function is disabled."]
    DISABLED_THE_I2C_MO,
    #[doc = "Enabled. The I2C monitor function is enabled."]
    ENABLED_THE_I2C_MON,
}
impl MONENR {
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
            MONENR::DISABLED_THE_I2C_MO => false,
            MONENR::ENABLED_THE_I2C_MON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONENR {
        match value {
            false => MONENR::DISABLED_THE_I2C_MO,
            true => MONENR::ENABLED_THE_I2C_MON,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THE_I2C_MO`"]
    #[inline]
    pub fn is_disabled_the_i2c_mo(&self) -> bool {
        *self == MONENR::DISABLED_THE_I2C_MO
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_I2C_MON`"]
    #[inline]
    pub fn is_enabled_the_i2c_mon(&self) -> bool {
        *self == MONENR::ENABLED_THE_I2C_MON
    }
}
#[doc = "Possible values of the field `TIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTENR {
    #[doc = "Disabled. Time-out function is disabled."]
    DISABLED_TIME_OUT_F,
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one timeout will be used in a system."]
    ENABLED_TIME_OUT_FU,
}
impl TIMEOUTENR {
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
            TIMEOUTENR::DISABLED_TIME_OUT_F => false,
            TIMEOUTENR::ENABLED_TIME_OUT_FU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMEOUTENR {
        match value {
            false => TIMEOUTENR::DISABLED_TIME_OUT_F,
            true => TIMEOUTENR::ENABLED_TIME_OUT_FU,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_TIME_OUT_F`"]
    #[inline]
    pub fn is_disabled_time_out_f(&self) -> bool {
        *self == TIMEOUTENR::DISABLED_TIME_OUT_F
    }
    #[doc = "Checks if the value of the field is `ENABLED_TIME_OUT_FU`"]
    #[inline]
    pub fn is_enabled_time_out_fu(&self) -> bool {
        *self == TIMEOUTENR::ENABLED_TIME_OUT_FU
    }
}
#[doc = "Possible values of the field `MONCLKSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONCLKSTRR {
    #[doc = "Disabled. The monitor function will not perform clock stretching. Software may not always be able to read data provided by the monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    DISABLED_THE_MONITO,
    #[doc = "Enabled. The monitor function will perform clock stretching in order to ensure that software can read all incoming data supplied by the monitor function."]
    ENABLED_THE_MONITOR,
}
impl MONCLKSTRR {
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
            MONCLKSTRR::DISABLED_THE_MONITO => false,
            MONCLKSTRR::ENABLED_THE_MONITOR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONCLKSTRR {
        match value {
            false => MONCLKSTRR::DISABLED_THE_MONITO,
            true => MONCLKSTRR::ENABLED_THE_MONITOR,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THE_MONITO`"]
    #[inline]
    pub fn is_disabled_the_monito(&self) -> bool {
        *self == MONCLKSTRR::DISABLED_THE_MONITO
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_MONITOR`"]
    #[inline]
    pub fn is_enabled_the_monitor(&self) -> bool {
        *self == MONCLKSTRR::ENABLED_THE_MONITOR
    }
}
#[doc = "Values that can be written to the field `MSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTENW {
    #[doc = "Disabled. The I2C Master function is disabled."]
    DISABLED_THE_I2C_MA,
    #[doc = "Enabled. The I2C Master function is enabled."]
    ENABLED_THE_I2C_MAS,
}
impl MSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTENW::DISABLED_THE_I2C_MA => false,
            MSTENW::ENABLED_THE_I2C_MAS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The I2C Master function is disabled."]
    #[inline]
    pub fn disabled_the_i2c_ma(self) -> &'a mut W {
        self.variant(MSTENW::DISABLED_THE_I2C_MA)
    }
    #[doc = "Enabled. The I2C Master function is enabled."]
    #[inline]
    pub fn enabled_the_i2c_mas(self) -> &'a mut W {
        self.variant(MSTENW::ENABLED_THE_I2C_MAS)
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
#[doc = "Values that can be written to the field `SLVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVENW {
    #[doc = "Disabled. The I2C slave function is disabled."]
    DISABLED_THE_I2C_SL,
    #[doc = "Enabled. The I2C slave function is enabled."]
    ENABLED_THE_I2C_SLA,
}
impl SLVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVENW::DISABLED_THE_I2C_SL => false,
            SLVENW::ENABLED_THE_I2C_SLA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The I2C slave function is disabled."]
    #[inline]
    pub fn disabled_the_i2c_sl(self) -> &'a mut W {
        self.variant(SLVENW::DISABLED_THE_I2C_SL)
    }
    #[doc = "Enabled. The I2C slave function is enabled."]
    #[inline]
    pub fn enabled_the_i2c_sla(self) -> &'a mut W {
        self.variant(SLVENW::ENABLED_THE_I2C_SLA)
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
#[doc = "Values that can be written to the field `MONEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONENW {
    #[doc = "Disabled. The I2C monitor function is disabled."]
    DISABLED_THE_I2C_MO,
    #[doc = "Enabled. The I2C monitor function is enabled."]
    ENABLED_THE_I2C_MON,
}
impl MONENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONENW::DISABLED_THE_I2C_MO => false,
            MONENW::ENABLED_THE_I2C_MON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The I2C monitor function is disabled."]
    #[inline]
    pub fn disabled_the_i2c_mo(self) -> &'a mut W {
        self.variant(MONENW::DISABLED_THE_I2C_MO)
    }
    #[doc = "Enabled. The I2C monitor function is enabled."]
    #[inline]
    pub fn enabled_the_i2c_mon(self) -> &'a mut W {
        self.variant(MONENW::ENABLED_THE_I2C_MON)
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
#[doc = "Values that can be written to the field `TIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTENW {
    #[doc = "Disabled. Time-out function is disabled."]
    DISABLED_TIME_OUT_F,
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one timeout will be used in a system."]
    ENABLED_TIME_OUT_FU,
}
impl TIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMEOUTENW::DISABLED_TIME_OUT_F => false,
            TIMEOUTENW::ENABLED_TIME_OUT_FU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Time-out function is disabled."]
    #[inline]
    pub fn disabled_time_out_f(self) -> &'a mut W {
        self.variant(TIMEOUTENW::DISABLED_TIME_OUT_F)
    }
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one timeout will be used in a system."]
    #[inline]
    pub fn enabled_time_out_fu(self) -> &'a mut W {
        self.variant(TIMEOUTENW::ENABLED_TIME_OUT_FU)
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
#[doc = "Values that can be written to the field `MONCLKSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONCLKSTRW {
    #[doc = "Disabled. The monitor function will not perform clock stretching. Software may not always be able to read data provided by the monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    DISABLED_THE_MONITO,
    #[doc = "Enabled. The monitor function will perform clock stretching in order to ensure that software can read all incoming data supplied by the monitor function."]
    ENABLED_THE_MONITOR,
}
impl MONCLKSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONCLKSTRW::DISABLED_THE_MONITO => false,
            MONCLKSTRW::ENABLED_THE_MONITOR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONCLKSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MONCLKSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONCLKSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The monitor function will not perform clock stretching. Software may not always be able to read data provided by the monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    #[inline]
    pub fn disabled_the_monito(self) -> &'a mut W {
        self.variant(MONCLKSTRW::DISABLED_THE_MONITO)
    }
    #[doc = "Enabled. The monitor function will perform clock stretching in order to ensure that software can read all incoming data supplied by the monitor function."]
    #[inline]
    pub fn enabled_the_monitor(self) -> &'a mut W {
        self.variant(MONCLKSTRW::ENABLED_THE_MONITOR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline]
    pub fn msten(&self) -> MSTENR {
        MSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline]
    pub fn slven(&self) -> SLVENR {
        SLVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline]
    pub fn monen(&self) -> MONENR {
        MONENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, timeout flags will be automatically cleared."]
    #[inline]
    pub fn timeouten(&self) -> TIMEOUTENR {
        TIMEOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline]
    pub fn monclkstr(&self) -> MONCLKSTRR {
        MONCLKSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline]
    pub fn msten(&mut self) -> _MSTENW {
        _MSTENW { w: self }
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline]
    pub fn slven(&mut self) -> _SLVENW {
        _SLVENW { w: self }
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline]
    pub fn monen(&mut self) -> _MONENW {
        _MONENW { w: self }
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, timeout flags will be automatically cleared."]
    #[inline]
    pub fn timeouten(&mut self) -> _TIMEOUTENW {
        _TIMEOUTENW { w: self }
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline]
    pub fn monclkstr(&mut self) -> _MONCLKSTRW {
        _MONCLKSTRW { w: self }
    }
}
