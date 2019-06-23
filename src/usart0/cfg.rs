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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts  are disabled. When Enable is set again, CFG and most other control bits remain unchanged. For instance, when re-enabled, the USART will immediately generate a TxRdy interrupt if  enabled because the transmitter has been reset and is therefore available."]
    DISABLED_THE_USART_,
    #[doc = "Enabled. The USART is enabled for operation."]
    ENABLED_THE_USART_I,
}
impl ENABLER {
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
            ENABLER::DISABLED_THE_USART_ => false,
            ENABLER::ENABLED_THE_USART_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::DISABLED_THE_USART_,
            true => ENABLER::ENABLED_THE_USART_I,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THE_USART_`"]
    #[inline]
    pub fn is_disabled_the_usart_(&self) -> bool {
        *self == ENABLER::DISABLED_THE_USART_
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_USART_I`"]
    #[inline]
    pub fn is_enabled_the_usart_i(&self) -> bool {
        *self == ENABLER::ENABLED_THE_USART_I
    }
}
#[doc = "Possible values of the field `DATALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATALENR {
    #[doc = "7 bit Data length."]
    _7_BIT_DATA_LENGTH_,
    #[doc = "8 bit Data length."]
    _8_BIT_DATA_LENGTH_,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTRL register."]
    _9_BIT_DATA_LENGTH_T,
    #[doc = "Reserved."]
    RESERVED_,
}
impl DATALENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATALENR::_7_BIT_DATA_LENGTH_ => 0,
            DATALENR::_8_BIT_DATA_LENGTH_ => 1,
            DATALENR::_9_BIT_DATA_LENGTH_T => 2,
            DATALENR::RESERVED_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATALENR {
        match value {
            0 => DATALENR::_7_BIT_DATA_LENGTH_,
            1 => DATALENR::_8_BIT_DATA_LENGTH_,
            2 => DATALENR::_9_BIT_DATA_LENGTH_T,
            3 => DATALENR::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_7_BIT_DATA_LENGTH_`"]
    #[inline]
    pub fn is_7_bit_data_length_(&self) -> bool {
        *self == DATALENR::_7_BIT_DATA_LENGTH_
    }
    #[doc = "Checks if the value of the field is `_8_BIT_DATA_LENGTH_`"]
    #[inline]
    pub fn is_8_bit_data_length_(&self) -> bool {
        *self == DATALENR::_8_BIT_DATA_LENGTH_
    }
    #[doc = "Checks if the value of the field is `_9_BIT_DATA_LENGTH_T`"]
    #[inline]
    pub fn is_9_bit_data_length_t(&self) -> bool {
        *self == DATALENR::_9_BIT_DATA_LENGTH_T
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline]
    pub fn is_reserved_(&self) -> bool {
        *self == DATALENR::RESERVED_
    }
}
#[doc = "Possible values of the field `PARITYSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYSELR {
    #[doc = "No parity."]
    NO_PARITY_,
    #[doc = "Reserved."]
    RESERVED_,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY_ADDS_A_,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY_ADDS_A_B,
}
impl PARITYSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARITYSELR::NO_PARITY_ => 0,
            PARITYSELR::RESERVED_ => 1,
            PARITYSELR::EVEN_PARITY_ADDS_A_ => 2,
            PARITYSELR::ODD_PARITY_ADDS_A_B => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARITYSELR {
        match value {
            0 => PARITYSELR::NO_PARITY_,
            1 => PARITYSELR::RESERVED_,
            2 => PARITYSELR::EVEN_PARITY_ADDS_A_,
            3 => PARITYSELR::ODD_PARITY_ADDS_A_B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_`"]
    #[inline]
    pub fn is_no_parity_(&self) -> bool {
        *self == PARITYSELR::NO_PARITY_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline]
    pub fn is_reserved_(&self) -> bool {
        *self == PARITYSELR::RESERVED_
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY_ADDS_A_`"]
    #[inline]
    pub fn is_even_parity_adds_a_(&self) -> bool {
        *self == PARITYSELR::EVEN_PARITY_ADDS_A_
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY_ADDS_A_B`"]
    #[inline]
    pub fn is_odd_parity_adds_a_b(&self) -> bool {
        *self == PARITYSELR::ODD_PARITY_ADDS_A_B
    }
}
#[doc = "Possible values of the field `STOPLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPLENR {
    #[doc = "1 stop bit."]
    _1_STOP_BIT_,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    _2_STOP_BITS_THIS_SE,
}
impl STOPLENR {
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
            STOPLENR::_1_STOP_BIT_ => false,
            STOPLENR::_2_STOP_BITS_THIS_SE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPLENR {
        match value {
            false => STOPLENR::_1_STOP_BIT_,
            true => STOPLENR::_2_STOP_BITS_THIS_SE,
        }
    }
    #[doc = "Checks if the value of the field is `_1_STOP_BIT_`"]
    #[inline]
    pub fn is_1_stop_bit_(&self) -> bool {
        *self == STOPLENR::_1_STOP_BIT_
    }
    #[doc = "Checks if the value of the field is `_2_STOP_BITS_THIS_SE`"]
    #[inline]
    pub fn is_2_stop_bits_this_se(&self) -> bool {
        *self == STOPLENR::_2_STOP_BITS_THIS_SE
    }
}
#[doc = "Possible values of the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSENR {
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    NO_FLOW_CONTROL_THE,
    #[doc = "Flow control enabled. The transmitter uses external or internal CTS for flow control purposes."]
    FLOW_CONTROL_ENABLED,
}
impl CTSENR {
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
            CTSENR::NO_FLOW_CONTROL_THE => false,
            CTSENR::FLOW_CONTROL_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSENR {
        match value {
            false => CTSENR::NO_FLOW_CONTROL_THE,
            true => CTSENR::FLOW_CONTROL_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLOW_CONTROL_THE`"]
    #[inline]
    pub fn is_no_flow_control_the(&self) -> bool {
        *self == CTSENR::NO_FLOW_CONTROL_THE
    }
    #[doc = "Checks if the value of the field is `FLOW_CONTROL_ENABLED`"]
    #[inline]
    pub fn is_flow_control_enabled(&self) -> bool {
        *self == CTSENR::FLOW_CONTROL_ENABLED
    }
}
#[doc = "Possible values of the field `SYNCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCENR {
    #[doc = "Asynchronous mode is selected."]
    ASYNCHRONOUS_MODE_IS,
    #[doc = "Synchronous mode is selected."]
    SYNCHRONOUS_MODE_IS_,
}
impl SYNCENR {
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
            SYNCENR::ASYNCHRONOUS_MODE_IS => false,
            SYNCENR::SYNCHRONOUS_MODE_IS_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCENR {
        match value {
            false => SYNCENR::ASYNCHRONOUS_MODE_IS,
            true => SYNCENR::SYNCHRONOUS_MODE_IS_,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE_IS`"]
    #[inline]
    pub fn is_asynchronous_mode_is(&self) -> bool {
        *self == SYNCENR::ASYNCHRONOUS_MODE_IS
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE_IS_`"]
    #[inline]
    pub fn is_synchronous_mode_is_(&self) -> bool {
        *self == SYNCENR::SYNCHRONOUS_MODE_IS_
    }
}
#[doc = "Possible values of the field `CLKPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPOLR {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE_UN_RXD,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE_UN_RXD_,
}
impl CLKPOLR {
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
            CLKPOLR::FALLING_EDGE_UN_RXD => false,
            CLKPOLR::RISING_EDGE_UN_RXD_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKPOLR {
        match value {
            false => CLKPOLR::FALLING_EDGE_UN_RXD,
            true => CLKPOLR::RISING_EDGE_UN_RXD_,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_UN_RXD`"]
    #[inline]
    pub fn is_falling_edge_un_rxd(&self) -> bool {
        *self == CLKPOLR::FALLING_EDGE_UN_RXD
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_UN_RXD_`"]
    #[inline]
    pub fn is_rising_edge_un_rxd_(&self) -> bool {
        *self == CLKPOLR::RISING_EDGE_UN_RXD_
    }
}
#[doc = "Possible values of the field `SYNCMST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCMSTR {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE_WHEN_SYNCHRON,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master. In asynchronous mode, the baud rate clock will be output on SCLK if it is connected to a pin."]
    MASTER_WHEN_SYNCHRO,
}
impl SYNCMSTR {
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
            SYNCMSTR::SLAVE_WHEN_SYNCHRON => false,
            SYNCMSTR::MASTER_WHEN_SYNCHRO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCMSTR {
        match value {
            false => SYNCMSTR::SLAVE_WHEN_SYNCHRON,
            true => SYNCMSTR::MASTER_WHEN_SYNCHRO,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_WHEN_SYNCHRON`"]
    #[inline]
    pub fn is_slave_when_synchron(&self) -> bool {
        *self == SYNCMSTR::SLAVE_WHEN_SYNCHRON
    }
    #[doc = "Checks if the value of the field is `MASTER_WHEN_SYNCHRO`"]
    #[inline]
    pub fn is_master_when_synchro(&self) -> bool {
        *self == SYNCMSTR::MASTER_WHEN_SYNCHRO
    }
}
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK_MODE_THIS_,
}
impl LOOPR {
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
            LOOPR::NORMAL_OPERATION_ => false,
            LOOPR::LOOPBACK_MODE_THIS_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPR {
        match value {
            false => LOOPR::NORMAL_OPERATION_,
            true => LOOPR::LOOPBACK_MODE_THIS_,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline]
    pub fn is_normal_operation_(&self) -> bool {
        *self == LOOPR::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `LOOPBACK_MODE_THIS_`"]
    #[inline]
    pub fn is_loopback_mode_this_(&self) -> bool {
        *self == LOOPR::LOOPBACK_MODE_THIS_
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLEW {
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts  are disabled. When Enable is set again, CFG and most other control bits remain unchanged. For instance, when re-enabled, the USART will immediately generate a TxRdy interrupt if  enabled because the transmitter has been reset and is therefore available."]
    DISABLED_THE_USART_,
    #[doc = "Enabled. The USART is enabled for operation."]
    ENABLED_THE_USART_I,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::DISABLED_THE_USART_ => false,
            ENABLEW::ENABLED_THE_USART_I => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts are disabled. When Enable is set again, CFG and most other control bits remain unchanged. For instance, when re-enabled, the USART will immediately generate a TxRdy interrupt if enabled because the transmitter has been reset and is therefore available."]
    #[inline]
    pub fn disabled_the_usart_(self) -> &'a mut W {
        self.variant(ENABLEW::DISABLED_THE_USART_)
    }
    #[doc = "Enabled. The USART is enabled for operation."]
    #[inline]
    pub fn enabled_the_usart_i(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLED_THE_USART_I)
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
#[doc = "Values that can be written to the field `DATALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATALENW {
    #[doc = "7 bit Data length."]
    _7_BIT_DATA_LENGTH_,
    #[doc = "8 bit Data length."]
    _8_BIT_DATA_LENGTH_,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTRL register."]
    _9_BIT_DATA_LENGTH_T,
    #[doc = "Reserved."]
    RESERVED_,
}
impl DATALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATALENW::_7_BIT_DATA_LENGTH_ => 0,
            DATALENW::_8_BIT_DATA_LENGTH_ => 1,
            DATALENW::_9_BIT_DATA_LENGTH_T => 2,
            DATALENW::RESERVED_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATALENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATALENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATALENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "7 bit Data length."]
    #[inline]
    pub fn _7_bit_data_length_(self) -> &'a mut W {
        self.variant(DATALENW::_7_BIT_DATA_LENGTH_)
    }
    #[doc = "8 bit Data length."]
    #[inline]
    pub fn _8_bit_data_length_(self) -> &'a mut W {
        self.variant(DATALENW::_8_BIT_DATA_LENGTH_)
    }
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTRL register."]
    #[inline]
    pub fn _9_bit_data_length_t(self) -> &'a mut W {
        self.variant(DATALENW::_9_BIT_DATA_LENGTH_T)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(DATALENW::RESERVED_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PARITYSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYSELW {
    #[doc = "No parity."]
    NO_PARITY_,
    #[doc = "Reserved."]
    RESERVED_,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY_ADDS_A_,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY_ADDS_A_B,
}
impl PARITYSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARITYSELW::NO_PARITY_ => 0,
            PARITYSELW::RESERVED_ => 1,
            PARITYSELW::EVEN_PARITY_ADDS_A_ => 2,
            PARITYSELW::ODD_PARITY_ADDS_A_B => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No parity."]
    #[inline]
    pub fn no_parity_(self) -> &'a mut W {
        self.variant(PARITYSELW::NO_PARITY_)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(PARITYSELW::RESERVED_)
    }
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    #[inline]
    pub fn even_parity_adds_a_(self) -> &'a mut W {
        self.variant(PARITYSELW::EVEN_PARITY_ADDS_A_)
    }
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    #[inline]
    pub fn odd_parity_adds_a_b(self) -> &'a mut W {
        self.variant(PARITYSELW::ODD_PARITY_ADDS_A_B)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOPLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPLENW {
    #[doc = "1 stop bit."]
    _1_STOP_BIT_,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    _2_STOP_BITS_THIS_SE,
}
impl STOPLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPLENW::_1_STOP_BIT_ => false,
            STOPLENW::_2_STOP_BITS_THIS_SE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPLENW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 stop bit."]
    #[inline]
    pub fn _1_stop_bit_(self) -> &'a mut W {
        self.variant(STOPLENW::_1_STOP_BIT_)
    }
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    #[inline]
    pub fn _2_stop_bits_this_se(self) -> &'a mut W {
        self.variant(STOPLENW::_2_STOP_BITS_THIS_SE)
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
#[doc = "Values that can be written to the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSENW {
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    NO_FLOW_CONTROL_THE,
    #[doc = "Flow control enabled. The transmitter uses external or internal CTS for flow control purposes."]
    FLOW_CONTROL_ENABLED,
}
impl CTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSENW::NO_FLOW_CONTROL_THE => false,
            CTSENW::FLOW_CONTROL_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    #[inline]
    pub fn no_flow_control_the(self) -> &'a mut W {
        self.variant(CTSENW::NO_FLOW_CONTROL_THE)
    }
    #[doc = "Flow control enabled. The transmitter uses external or internal CTS for flow control purposes."]
    #[inline]
    pub fn flow_control_enabled(self) -> &'a mut W {
        self.variant(CTSENW::FLOW_CONTROL_ENABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCENW {
    #[doc = "Asynchronous mode is selected."]
    ASYNCHRONOUS_MODE_IS,
    #[doc = "Synchronous mode is selected."]
    SYNCHRONOUS_MODE_IS_,
}
impl SYNCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCENW::ASYNCHRONOUS_MODE_IS => false,
            SYNCENW::SYNCHRONOUS_MODE_IS_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous mode is selected."]
    #[inline]
    pub fn asynchronous_mode_is(self) -> &'a mut W {
        self.variant(SYNCENW::ASYNCHRONOUS_MODE_IS)
    }
    #[doc = "Synchronous mode is selected."]
    #[inline]
    pub fn synchronous_mode_is_(self) -> &'a mut W {
        self.variant(SYNCENW::SYNCHRONOUS_MODE_IS_)
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
#[doc = "Values that can be written to the field `CLKPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPOLW {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE_UN_RXD,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE_UN_RXD_,
}
impl CLKPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKPOLW::FALLING_EDGE_UN_RXD => false,
            CLKPOLW::RISING_EDGE_UN_RXD_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    #[inline]
    pub fn falling_edge_un_rxd(self) -> &'a mut W {
        self.variant(CLKPOLW::FALLING_EDGE_UN_RXD)
    }
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    #[inline]
    pub fn rising_edge_un_rxd_(self) -> &'a mut W {
        self.variant(CLKPOLW::RISING_EDGE_UN_RXD_)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCMST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCMSTW {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE_WHEN_SYNCHRON,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master. In asynchronous mode, the baud rate clock will be output on SCLK if it is connected to a pin."]
    MASTER_WHEN_SYNCHRO,
}
impl SYNCMSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCMSTW::SLAVE_WHEN_SYNCHRON => false,
            SYNCMSTW::MASTER_WHEN_SYNCHRO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCMSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCMSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCMSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    #[inline]
    pub fn slave_when_synchron(self) -> &'a mut W {
        self.variant(SYNCMSTW::SLAVE_WHEN_SYNCHRON)
    }
    #[doc = "Master. When synchronous mode is enabled, the USART is a master. In asynchronous mode, the baud rate clock will be output on SCLK if it is connected to a pin."]
    #[inline]
    pub fn master_when_synchro(self) -> &'a mut W {
        self.variant(SYNCMSTW::MASTER_WHEN_SYNCHRO)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPW {
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK_MODE_THIS_,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPW::NORMAL_OPERATION_ => false,
            LOOPW::LOOPBACK_MODE_THIS_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(LOOPW::NORMAL_OPERATION_)
    }
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    #[inline]
    pub fn loopback_mode_this_(self) -> &'a mut W {
        self.variant(LOOPW::LOOPBACK_MODE_THIS_)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USART Enable."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline]
    pub fn datalen(&self) -> DATALENR {
        DATALENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline]
    pub fn paritysel(&self) -> PARITYSELR {
        PARITYSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline]
    pub fn stoplen(&self) -> STOPLENR {
        STOPLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled. See Section 16.7.3 for more information."]
    #[inline]
    pub fn ctsen(&self) -> CTSENR {
        CTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline]
    pub fn syncen(&self) -> SYNCENR {
        SYNCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline]
    pub fn clkpol(&self) -> CLKPOLR {
        CLKPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline]
    pub fn syncmst(&self) -> SYNCMSTR {
        SYNCMSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        LOOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - USART Enable."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline]
    pub fn datalen(&mut self) -> _DATALENW {
        _DATALENW { w: self }
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline]
    pub fn paritysel(&mut self) -> _PARITYSELW {
        _PARITYSELW { w: self }
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline]
    pub fn stoplen(&mut self) -> _STOPLENW {
        _STOPLENW { w: self }
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled. See Section 16.7.3 for more information."]
    #[inline]
    pub fn ctsen(&mut self) -> _CTSENW {
        _CTSENW { w: self }
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline]
    pub fn syncen(&mut self) -> _SYNCENW {
        _SYNCENW { w: self }
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline]
    pub fn clkpol(&mut self) -> _CLKPOLW {
        _CLKPOLW { w: self }
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline]
    pub fn syncmst(&mut self) -> _SYNCMSTW {
        _SYNCMSTW { w: self }
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
}
