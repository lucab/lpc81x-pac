#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `MSTPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGR {
    #[doc = "No service needed. The Master function does not currently need service."]
    NO_SERVICE_NEEDED_T,
    #[doc = "Service needed. The Master function needs service. Information on what is needed can be found in the adjacent MSTSTATE field."]
    SERVICE_NEEDED_THE_,
}
impl MSTPENDINGR {
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
            MSTPENDINGR::NO_SERVICE_NEEDED_T => false,
            MSTPENDINGR::SERVICE_NEEDED_THE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTPENDINGR {
        match value {
            false => MSTPENDINGR::NO_SERVICE_NEEDED_T,
            true => MSTPENDINGR::SERVICE_NEEDED_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SERVICE_NEEDED_T`"]
    #[inline]
    pub fn is_no_service_needed_t(&self) -> bool {
        *self == MSTPENDINGR::NO_SERVICE_NEEDED_T
    }
    #[doc = "Checks if the value of the field is `SERVICE_NEEDED_THE_`"]
    #[inline]
    pub fn is_service_needed_the_(&self) -> bool {
        *self == MSTPENDINGR::SERVICE_NEEDED_THE_
    }
}
#[doc = "Possible values of the field `MSTSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTATER {
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    IDLE_THE_MASTER_FUN,
    #[doc = "Receive ready. Received data  available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    RECEIVE_READY_RECEI,
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TRANSMIT_READY_DATA,
    #[doc = "Address. Slave Nacked address."]
    ADDRESS_SLAVE_NACKE,
    #[doc = "Data. Slave Nacked transmitted data."]
    DATA_SLAVE_NACKED_T,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSTSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSTSTATER::IDLE_THE_MASTER_FUN => 0,
            MSTSTATER::RECEIVE_READY_RECEI => 1,
            MSTSTATER::TRANSMIT_READY_DATA => 2,
            MSTSTATER::ADDRESS_SLAVE_NACKE => 3,
            MSTSTATER::DATA_SLAVE_NACKED_T => 4,
            MSTSTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSTSTATER {
        match value {
            0 => MSTSTATER::IDLE_THE_MASTER_FUN,
            1 => MSTSTATER::RECEIVE_READY_RECEI,
            2 => MSTSTATER::TRANSMIT_READY_DATA,
            3 => MSTSTATER::ADDRESS_SLAVE_NACKE,
            4 => MSTSTATER::DATA_SLAVE_NACKED_T,
            i => MSTSTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THE_MASTER_FUN`"]
    #[inline]
    pub fn is_idle_the_master_fun(&self) -> bool {
        *self == MSTSTATER::IDLE_THE_MASTER_FUN
    }
    #[doc = "Checks if the value of the field is `RECEIVE_READY_RECEI`"]
    #[inline]
    pub fn is_receive_ready_recei(&self) -> bool {
        *self == MSTSTATER::RECEIVE_READY_RECEI
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_READY_DATA`"]
    #[inline]
    pub fn is_transmit_ready_data(&self) -> bool {
        *self == MSTSTATER::TRANSMIT_READY_DATA
    }
    #[doc = "Checks if the value of the field is `ADDRESS_SLAVE_NACKE`"]
    #[inline]
    pub fn is_address_slave_nacke(&self) -> bool {
        *self == MSTSTATER::ADDRESS_SLAVE_NACKE
    }
    #[doc = "Checks if the value of the field is `DATA_SLAVE_NACKED_T`"]
    #[inline]
    pub fn is_data_slave_nacked_t(&self) -> bool {
        *self == MSTSTATER::DATA_SLAVE_NACKED_T
    }
}
#[doc = "Possible values of the field `MSTARBLOSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSR {
    #[doc = "No loss. No Arbitration Loss has occurred."]
    NO_LOSS_NO_ARBITRAT,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS_TH,
}
impl MSTARBLOSSR {
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
            MSTARBLOSSR::NO_LOSS_NO_ARBITRAT => false,
            MSTARBLOSSR::ARBITRATION_LOSS_TH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTARBLOSSR {
        match value {
            false => MSTARBLOSSR::NO_LOSS_NO_ARBITRAT,
            true => MSTARBLOSSR::ARBITRATION_LOSS_TH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_LOSS_NO_ARBITRAT`"]
    #[inline]
    pub fn is_no_loss_no_arbitrat(&self) -> bool {
        *self == MSTARBLOSSR::NO_LOSS_NO_ARBITRAT
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOSS_TH`"]
    #[inline]
    pub fn is_arbitration_loss_th(&self) -> bool {
        *self == MSTARBLOSSR::ARBITRATION_LOSS_TH
    }
}
#[doc = "Possible values of the field `MSTSTSTPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRR {
    #[doc = "No Start/Stop Error has occurred."]
    NO_STARTSTOP_ERROR_,
    #[doc = "Start/stop error has occurred. The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    STARTSTOP_ERROR_HAS,
}
impl MSTSTSTPERRR {
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
            MSTSTSTPERRR::NO_STARTSTOP_ERROR_ => false,
            MSTSTSTPERRR::STARTSTOP_ERROR_HAS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTSTPERRR {
        match value {
            false => MSTSTSTPERRR::NO_STARTSTOP_ERROR_,
            true => MSTSTSTPERRR::STARTSTOP_ERROR_HAS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STARTSTOP_ERROR_`"]
    #[inline]
    pub fn is_no_startstop_error_(&self) -> bool {
        *self == MSTSTSTPERRR::NO_STARTSTOP_ERROR_
    }
    #[doc = "Checks if the value of the field is `STARTSTOP_ERROR_HAS`"]
    #[inline]
    pub fn is_startstop_error_has(&self) -> bool {
        *self == MSTSTSTPERRR::STARTSTOP_ERROR_HAS
    }
}
#[doc = "Possible values of the field `SLVPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGR {
    #[doc = "No service needed. The Slave function does not currently need service."]
    NO_SERVICE_NEEDED_T,
    #[doc = "Service needed. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    SERVICE_NEEDED_THE_,
}
impl SLVPENDINGR {
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
            SLVPENDINGR::NO_SERVICE_NEEDED_T => false,
            SLVPENDINGR::SERVICE_NEEDED_THE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVPENDINGR {
        match value {
            false => SLVPENDINGR::NO_SERVICE_NEEDED_T,
            true => SLVPENDINGR::SERVICE_NEEDED_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SERVICE_NEEDED_T`"]
    #[inline]
    pub fn is_no_service_needed_t(&self) -> bool {
        *self == SLVPENDINGR::NO_SERVICE_NEEDED_T
    }
    #[doc = "Checks if the value of the field is `SERVICE_NEEDED_THE_`"]
    #[inline]
    pub fn is_service_needed_the_(&self) -> bool {
        *self == SLVPENDINGR::SERVICE_NEEDED_THE_
    }
}
#[doc = "Possible values of the field `SLVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSTATER {
    #[doc = "Received. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    RECEIVED_ADDRESS_PL,
    #[doc = "Data available. Received data is available (Slave Receiver mode)."]
    DATA_AVAILABLE_RECE,
    #[doc = "Data ready for transmit. Data can be transmitted (Slave Transmitter mode)."]
    DATA_READY_FOR_TRANS,
    #[doc = "Reserved."]
    RESERVED_,
}
impl SLVSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLVSTATER::RECEIVED_ADDRESS_PL => 0,
            SLVSTATER::DATA_AVAILABLE_RECE => 1,
            SLVSTATER::DATA_READY_FOR_TRANS => 2,
            SLVSTATER::RESERVED_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLVSTATER {
        match value {
            0 => SLVSTATER::RECEIVED_ADDRESS_PL,
            1 => SLVSTATER::DATA_AVAILABLE_RECE,
            2 => SLVSTATER::DATA_READY_FOR_TRANS,
            3 => SLVSTATER::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED_ADDRESS_PL`"]
    #[inline]
    pub fn is_received_address_pl(&self) -> bool {
        *self == SLVSTATER::RECEIVED_ADDRESS_PL
    }
    #[doc = "Checks if the value of the field is `DATA_AVAILABLE_RECE`"]
    #[inline]
    pub fn is_data_available_rece(&self) -> bool {
        *self == SLVSTATER::DATA_AVAILABLE_RECE
    }
    #[doc = "Checks if the value of the field is `DATA_READY_FOR_TRANS`"]
    #[inline]
    pub fn is_data_ready_for_trans(&self) -> bool {
        *self == SLVSTATER::DATA_READY_FOR_TRANS
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline]
    pub fn is_reserved_(&self) -> bool {
        *self == SLVSTATER::RESERVED_
    }
}
#[doc = "Possible values of the field `SLVNOTSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRR {
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING_THE_SLAV,
    #[doc = "Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING_THE_,
}
impl SLVNOTSTRR {
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
            SLVNOTSTRR::STRETCHING_THE_SLAV => false,
            SLVNOTSTRR::NOT_STRETCHING_THE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVNOTSTRR {
        match value {
            false => SLVNOTSTRR::STRETCHING_THE_SLAV,
            true => SLVNOTSTRR::NOT_STRETCHING_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `STRETCHING_THE_SLAV`"]
    #[inline]
    pub fn is_stretching_the_slav(&self) -> bool {
        *self == SLVNOTSTRR::STRETCHING_THE_SLAV
    }
    #[doc = "Checks if the value of the field is `NOT_STRETCHING_THE_`"]
    #[inline]
    pub fn is_not_stretching_the_(&self) -> bool {
        *self == SLVNOTSTRR::NOT_STRETCHING_THE_
    }
}
#[doc = "Possible values of the field `SLVIDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVIDXR {
    #[doc = "Slave address 0 was matched."]
    SLAVE_ADDRESS_0_WAS_,
    #[doc = "Slave address 1 was matched."]
    SLAVE_ADDRESS_1_WAS_,
    #[doc = "Slave address 2 was matched."]
    SLAVE_ADDRESS_2_WAS_,
    #[doc = "Slave address 3 was matched."]
    SLAVE_ADDRESS_3_WAS_,
}
impl SLVIDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLVIDXR::SLAVE_ADDRESS_0_WAS_ => 0,
            SLVIDXR::SLAVE_ADDRESS_1_WAS_ => 1,
            SLVIDXR::SLAVE_ADDRESS_2_WAS_ => 2,
            SLVIDXR::SLAVE_ADDRESS_3_WAS_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLVIDXR {
        match value {
            0 => SLVIDXR::SLAVE_ADDRESS_0_WAS_,
            1 => SLVIDXR::SLAVE_ADDRESS_1_WAS_,
            2 => SLVIDXR::SLAVE_ADDRESS_2_WAS_,
            3 => SLVIDXR::SLAVE_ADDRESS_3_WAS_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_0_WAS_`"]
    #[inline]
    pub fn is_slave_address_0_was_(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_0_WAS_
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_1_WAS_`"]
    #[inline]
    pub fn is_slave_address_1_was_(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_1_WAS_
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_2_WAS_`"]
    #[inline]
    pub fn is_slave_address_2_was_(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_2_WAS_
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_3_WAS_`"]
    #[inline]
    pub fn is_slave_address_3_was_(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_3_WAS_
    }
}
#[doc = "Possible values of the field `SLVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSELR {
    #[doc = "Not selected. The Slave function is not currently selected."]
    NOT_SELECTED_THE_SL,
    #[doc = "Selected. The Slave function is currently selected."]
    SELECTED_THE_SLAVE_,
}
impl SLVSELR {
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
            SLVSELR::NOT_SELECTED_THE_SL => false,
            SLVSELR::SELECTED_THE_SLAVE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVSELR {
        match value {
            false => SLVSELR::NOT_SELECTED_THE_SL,
            true => SLVSELR::SELECTED_THE_SLAVE_,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED_THE_SL`"]
    #[inline]
    pub fn is_not_selected_the_sl(&self) -> bool {
        *self == SLVSELR::NOT_SELECTED_THE_SL
    }
    #[doc = "Checks if the value of the field is `SELECTED_THE_SLAVE_`"]
    #[inline]
    pub fn is_selected_the_slave_(&self) -> bool {
        *self == SLVSELR::SELECTED_THE_SLAVE_
    }
}
#[doc = "Possible values of the field `SLVDESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELR {
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NOT_DESELECTED_THE_,
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    DESELECTED_THE_SLAV,
}
impl SLVDESELR {
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
            SLVDESELR::NOT_DESELECTED_THE_ => false,
            SLVDESELR::DESELECTED_THE_SLAV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVDESELR {
        match value {
            false => SLVDESELR::NOT_DESELECTED_THE_,
            true => SLVDESELR::DESELECTED_THE_SLAV,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DESELECTED_THE_`"]
    #[inline]
    pub fn is_not_deselected_the_(&self) -> bool {
        *self == SLVDESELR::NOT_DESELECTED_THE_
    }
    #[doc = "Checks if the value of the field is `DESELECTED_THE_SLAV`"]
    #[inline]
    pub fn is_deselected_the_slav(&self) -> bool {
        *self == SLVDESELR::DESELECTED_THE_SLAV
    }
}
#[doc = "Possible values of the field `MONRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYR {
    #[doc = "No data. The Monitor function does not currently have data available."]
    NO_DATA_THE_MONITOR,
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING_THE_MO,
}
impl MONRDYR {
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
            MONRDYR::NO_DATA_THE_MONITOR => false,
            MONRDYR::DATA_WAITING_THE_MO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONRDYR {
        match value {
            false => MONRDYR::NO_DATA_THE_MONITOR,
            true => MONRDYR::DATA_WAITING_THE_MO,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA_THE_MONITOR`"]
    #[inline]
    pub fn is_no_data_the_monitor(&self) -> bool {
        *self == MONRDYR::NO_DATA_THE_MONITOR
    }
    #[doc = "Checks if the value of the field is `DATA_WAITING_THE_MO`"]
    #[inline]
    pub fn is_data_waiting_the_mo(&self) -> bool {
        *self == MONRDYR::DATA_WAITING_THE_MO
    }
}
#[doc = "Possible values of the field `MONOV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVR {
    #[doc = "No overrun. Monitor data has not overrun."]
    NO_OVERRUN_MONITOR_,
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    OVERRUN_A_MONITOR_D,
}
impl MONOVR {
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
            MONOVR::NO_OVERRUN_MONITOR_ => false,
            MONOVR::OVERRUN_A_MONITOR_D => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONOVR {
        match value {
            false => MONOVR::NO_OVERRUN_MONITOR_,
            true => MONOVR::OVERRUN_A_MONITOR_D,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN_MONITOR_`"]
    #[inline]
    pub fn is_no_overrun_monitor_(&self) -> bool {
        *self == MONOVR::NO_OVERRUN_MONITOR_
    }
    #[doc = "Checks if the value of the field is `OVERRUN_A_MONITOR_D`"]
    #[inline]
    pub fn is_overrun_a_monitor_d(&self) -> bool {
        *self == MONOVR::OVERRUN_A_MONITOR_D
    }
}
#[doc = "Possible values of the field `MONACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONACTIVER {
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    INACTIVE_THE_MONITO,
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    ACTIVE_THE_MONITOR_,
}
impl MONACTIVER {
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
            MONACTIVER::INACTIVE_THE_MONITO => false,
            MONACTIVER::ACTIVE_THE_MONITOR_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONACTIVER {
        match value {
            false => MONACTIVER::INACTIVE_THE_MONITO,
            true => MONACTIVER::ACTIVE_THE_MONITOR_,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_THE_MONITO`"]
    #[inline]
    pub fn is_inactive_the_monito(&self) -> bool {
        *self == MONACTIVER::INACTIVE_THE_MONITO
    }
    #[doc = "Checks if the value of the field is `ACTIVE_THE_MONITOR_`"]
    #[inline]
    pub fn is_active_the_monitor_(&self) -> bool {
        *self == MONACTIVER::ACTIVE_THE_MONITOR_
    }
}
#[doc = "Possible values of the field `MONIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLER {
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NOT_IDLE_THE_I2C_BU,
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    IDLE_THE_I2C_BUS_HA,
}
impl MONIDLER {
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
            MONIDLER::NOT_IDLE_THE_I2C_BU => false,
            MONIDLER::IDLE_THE_I2C_BUS_HA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONIDLER {
        match value {
            false => MONIDLER::NOT_IDLE_THE_I2C_BU,
            true => MONIDLER::IDLE_THE_I2C_BUS_HA,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IDLE_THE_I2C_BU`"]
    #[inline]
    pub fn is_not_idle_the_i2c_bu(&self) -> bool {
        *self == MONIDLER::NOT_IDLE_THE_I2C_BU
    }
    #[doc = "Checks if the value of the field is `IDLE_THE_I2C_BUS_HA`"]
    #[inline]
    pub fn is_idle_the_i2c_bus_ha(&self) -> bool {
        *self == MONIDLER::IDLE_THE_I2C_BUS_HA
    }
}
#[doc = "Possible values of the field `EVENTTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTR {
    #[doc = "No time-out. I2C bus events have not caused a timeout."]
    NO_TIME_OUT_I2C_BUS,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the I2C Timeout register."]
    EVENT_TIME_OUT_THE_,
}
impl EVENTTIMEOUTR {
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
            EVENTTIMEOUTR::NO_TIME_OUT_I2C_BUS => false,
            EVENTTIMEOUTR::EVENT_TIME_OUT_THE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVENTTIMEOUTR {
        match value {
            false => EVENTTIMEOUTR::NO_TIME_OUT_I2C_BUS,
            true => EVENTTIMEOUTR::EVENT_TIME_OUT_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_OUT_I2C_BUS`"]
    #[inline]
    pub fn is_no_time_out_i2c_bus(&self) -> bool {
        *self == EVENTTIMEOUTR::NO_TIME_OUT_I2C_BUS
    }
    #[doc = "Checks if the value of the field is `EVENT_TIME_OUT_THE_`"]
    #[inline]
    pub fn is_event_time_out_the_(&self) -> bool {
        *self == EVENTTIMEOUTR::EVENT_TIME_OUT_THE_
    }
}
#[doc = "Possible values of the field `SCLTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTR {
    #[doc = "No time-out. SCL low time has not caused a timeout."]
    NO_TIME_OUT_SCL_LOW,
    #[doc = "Time-out. SCL low time has caused a timeout."]
    TIME_OUT_SCL_LOW_TI,
}
impl SCLTIMEOUTR {
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
            SCLTIMEOUTR::NO_TIME_OUT_SCL_LOW => false,
            SCLTIMEOUTR::TIME_OUT_SCL_LOW_TI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLTIMEOUTR {
        match value {
            false => SCLTIMEOUTR::NO_TIME_OUT_SCL_LOW,
            true => SCLTIMEOUTR::TIME_OUT_SCL_LOW_TI,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_OUT_SCL_LOW`"]
    #[inline]
    pub fn is_no_time_out_scl_low(&self) -> bool {
        *self == SCLTIMEOUTR::NO_TIME_OUT_SCL_LOW
    }
    #[doc = "Checks if the value of the field is `TIME_OUT_SCL_LOW_TI`"]
    #[inline]
    pub fn is_time_out_scl_low_ti(&self) -> bool {
        *self == SCLTIMEOUTR::TIME_OUT_SCL_LOW_TI
    }
}
#[doc = "Values that can be written to the field `MSTPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGW {
    #[doc = "No service needed. The Master function does not currently need service."]
    NO_SERVICE_NEEDED_T,
    #[doc = "Service needed. The Master function needs service. Information on what is needed can be found in the adjacent MSTSTATE field."]
    SERVICE_NEEDED_THE_,
}
impl MSTPENDINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTPENDINGW::NO_SERVICE_NEEDED_T => false,
            MSTPENDINGW::SERVICE_NEEDED_THE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTPENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTPENDINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTPENDINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No service needed. The Master function does not currently need service."]
    #[inline]
    pub fn no_service_needed_t(self) -> &'a mut W {
        self.variant(MSTPENDINGW::NO_SERVICE_NEEDED_T)
    }
    #[doc = "Service needed. The Master function needs service. Information on what is needed can be found in the adjacent MSTSTATE field."]
    #[inline]
    pub fn service_needed_the_(self) -> &'a mut W {
        self.variant(MSTPENDINGW::SERVICE_NEEDED_THE_)
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
#[doc = "Values that can be written to the field `MSTSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTATEW {
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    IDLE_THE_MASTER_FUN,
    #[doc = "Receive ready. Received data  available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    RECEIVE_READY_RECEI,
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TRANSMIT_READY_DATA,
    #[doc = "Address. Slave Nacked address."]
    ADDRESS_SLAVE_NACKE,
    #[doc = "Data. Slave Nacked transmitted data."]
    DATA_SLAVE_NACKED_T,
}
impl MSTSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSTSTATEW::IDLE_THE_MASTER_FUN => 0,
            MSTSTATEW::RECEIVE_READY_RECEI => 1,
            MSTSTATEW::TRANSMIT_READY_DATA => 2,
            MSTSTATEW::ADDRESS_SLAVE_NACKE => 3,
            MSTSTATEW::DATA_SLAVE_NACKED_T => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    #[inline]
    pub fn idle_the_master_fun(self) -> &'a mut W {
        self.variant(MSTSTATEW::IDLE_THE_MASTER_FUN)
    }
    #[doc = "Receive ready. Received data available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    #[inline]
    pub fn receive_ready_recei(self) -> &'a mut W {
        self.variant(MSTSTATEW::RECEIVE_READY_RECEI)
    }
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    #[inline]
    pub fn transmit_ready_data(self) -> &'a mut W {
        self.variant(MSTSTATEW::TRANSMIT_READY_DATA)
    }
    #[doc = "Address. Slave Nacked address."]
    #[inline]
    pub fn address_slave_nacke(self) -> &'a mut W {
        self.variant(MSTSTATEW::ADDRESS_SLAVE_NACKE)
    }
    #[doc = "Data. Slave Nacked transmitted data."]
    #[inline]
    pub fn data_slave_nacked_t(self) -> &'a mut W {
        self.variant(MSTSTATEW::DATA_SLAVE_NACKED_T)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTARBLOSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSW {
    #[doc = "No loss. No Arbitration Loss has occurred."]
    NO_LOSS_NO_ARBITRAT,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS_TH,
}
impl MSTARBLOSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSW::NO_LOSS_NO_ARBITRAT => false,
            MSTARBLOSSW::ARBITRATION_LOSS_TH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTARBLOSSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTARBLOSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTARBLOSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No loss. No Arbitration Loss has occurred."]
    #[inline]
    pub fn no_loss_no_arbitrat(self) -> &'a mut W {
        self.variant(MSTARBLOSSW::NO_LOSS_NO_ARBITRAT)
    }
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    #[inline]
    pub fn arbitration_loss_th(self) -> &'a mut W {
        self.variant(MSTARBLOSSW::ARBITRATION_LOSS_TH)
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
#[doc = "Values that can be written to the field `MSTSTSTPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRW {
    #[doc = "No Start/Stop Error has occurred."]
    NO_STARTSTOP_ERROR_,
    #[doc = "Start/stop error has occurred. The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    STARTSTOP_ERROR_HAS,
}
impl MSTSTSTPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRW::NO_STARTSTOP_ERROR_ => false,
            MSTSTSTPERRW::STARTSTOP_ERROR_HAS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTSTPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTSTPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTSTPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Start/Stop Error has occurred."]
    #[inline]
    pub fn no_startstop_error_(self) -> &'a mut W {
        self.variant(MSTSTSTPERRW::NO_STARTSTOP_ERROR_)
    }
    #[doc = "Start/stop error has occurred. The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    #[inline]
    pub fn startstop_error_has(self) -> &'a mut W {
        self.variant(MSTSTSTPERRW::STARTSTOP_ERROR_HAS)
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
#[doc = "Values that can be written to the field `SLVPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGW {
    #[doc = "No service needed. The Slave function does not currently need service."]
    NO_SERVICE_NEEDED_T,
    #[doc = "Service needed. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    SERVICE_NEEDED_THE_,
}
impl SLVPENDINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVPENDINGW::NO_SERVICE_NEEDED_T => false,
            SLVPENDINGW::SERVICE_NEEDED_THE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVPENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVPENDINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVPENDINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No service needed. The Slave function does not currently need service."]
    #[inline]
    pub fn no_service_needed_t(self) -> &'a mut W {
        self.variant(SLVPENDINGW::NO_SERVICE_NEEDED_T)
    }
    #[doc = "Service needed. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    #[inline]
    pub fn service_needed_the_(self) -> &'a mut W {
        self.variant(SLVPENDINGW::SERVICE_NEEDED_THE_)
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
#[doc = "Values that can be written to the field `SLVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSTATEW {
    #[doc = "Received. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    RECEIVED_ADDRESS_PL,
    #[doc = "Data available. Received data is available (Slave Receiver mode)."]
    DATA_AVAILABLE_RECE,
    #[doc = "Data ready for transmit. Data can be transmitted (Slave Transmitter mode)."]
    DATA_READY_FOR_TRANS,
    #[doc = "Reserved."]
    RESERVED_,
}
impl SLVSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLVSTATEW::RECEIVED_ADDRESS_PL => 0,
            SLVSTATEW::DATA_AVAILABLE_RECE => 1,
            SLVSTATEW::DATA_READY_FOR_TRANS => 2,
            SLVSTATEW::RESERVED_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVSTATEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Received. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    #[inline]
    pub fn received_address_pl(self) -> &'a mut W {
        self.variant(SLVSTATEW::RECEIVED_ADDRESS_PL)
    }
    #[doc = "Data available. Received data is available (Slave Receiver mode)."]
    #[inline]
    pub fn data_available_rece(self) -> &'a mut W {
        self.variant(SLVSTATEW::DATA_AVAILABLE_RECE)
    }
    #[doc = "Data ready for transmit. Data can be transmitted (Slave Transmitter mode)."]
    #[inline]
    pub fn data_ready_for_trans(self) -> &'a mut W {
        self.variant(SLVSTATEW::DATA_READY_FOR_TRANS)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(SLVSTATEW::RESERVED_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVNOTSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRW {
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING_THE_SLAV,
    #[doc = "Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING_THE_,
}
impl SLVNOTSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNOTSTRW::STRETCHING_THE_SLAV => false,
            SLVNOTSTRW::NOT_STRETCHING_THE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVNOTSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNOTSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVNOTSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    #[inline]
    pub fn stretching_the_slav(self) -> &'a mut W {
        self.variant(SLVNOTSTRW::STRETCHING_THE_SLAV)
    }
    #[doc = "Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    #[inline]
    pub fn not_stretching_the_(self) -> &'a mut W {
        self.variant(SLVNOTSTRW::NOT_STRETCHING_THE_)
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
#[doc = "Values that can be written to the field `SLVIDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVIDXW {
    #[doc = "Slave address 0 was matched."]
    SLAVE_ADDRESS_0_WAS_,
    #[doc = "Slave address 1 was matched."]
    SLAVE_ADDRESS_1_WAS_,
    #[doc = "Slave address 2 was matched."]
    SLAVE_ADDRESS_2_WAS_,
    #[doc = "Slave address 3 was matched."]
    SLAVE_ADDRESS_3_WAS_,
}
impl SLVIDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLVIDXW::SLAVE_ADDRESS_0_WAS_ => 0,
            SLVIDXW::SLAVE_ADDRESS_1_WAS_ => 1,
            SLVIDXW::SLAVE_ADDRESS_2_WAS_ => 2,
            SLVIDXW::SLAVE_ADDRESS_3_WAS_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVIDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVIDXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slave address 0 was matched."]
    #[inline]
    pub fn slave_address_0_was_(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_0_WAS_)
    }
    #[doc = "Slave address 1 was matched."]
    #[inline]
    pub fn slave_address_1_was_(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_1_WAS_)
    }
    #[doc = "Slave address 2 was matched."]
    #[inline]
    pub fn slave_address_2_was_(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_2_WAS_)
    }
    #[doc = "Slave address 3 was matched."]
    #[inline]
    pub fn slave_address_3_was_(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_3_WAS_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSELW {
    #[doc = "Not selected. The Slave function is not currently selected."]
    NOT_SELECTED_THE_SL,
    #[doc = "Selected. The Slave function is currently selected."]
    SELECTED_THE_SLAVE_,
}
impl SLVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVSELW::NOT_SELECTED_THE_SL => false,
            SLVSELW::SELECTED_THE_SLAVE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. The Slave function is not currently selected."]
    #[inline]
    pub fn not_selected_the_sl(self) -> &'a mut W {
        self.variant(SLVSELW::NOT_SELECTED_THE_SL)
    }
    #[doc = "Selected. The Slave function is currently selected."]
    #[inline]
    pub fn selected_the_slave_(self) -> &'a mut W {
        self.variant(SLVSELW::SELECTED_THE_SLAVE_)
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
#[doc = "Values that can be written to the field `SLVDESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELW {
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NOT_DESELECTED_THE_,
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    DESELECTED_THE_SLAV,
}
impl SLVDESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDESELW::NOT_DESELECTED_THE_ => false,
            SLVDESELW::DESELECTED_THE_SLAV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVDESELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVDESELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    #[inline]
    pub fn not_deselected_the_(self) -> &'a mut W {
        self.variant(SLVDESELW::NOT_DESELECTED_THE_)
    }
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    #[inline]
    pub fn deselected_the_slav(self) -> &'a mut W {
        self.variant(SLVDESELW::DESELECTED_THE_SLAV)
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
#[doc = "Values that can be written to the field `MONRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYW {
    #[doc = "No data. The Monitor function does not currently have data available."]
    NO_DATA_THE_MONITOR,
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING_THE_MO,
}
impl MONRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONRDYW::NO_DATA_THE_MONITOR => false,
            MONRDYW::DATA_WAITING_THE_MO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No data. The Monitor function does not currently have data available."]
    #[inline]
    pub fn no_data_the_monitor(self) -> &'a mut W {
        self.variant(MONRDYW::NO_DATA_THE_MONITOR)
    }
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    #[inline]
    pub fn data_waiting_the_mo(self) -> &'a mut W {
        self.variant(MONRDYW::DATA_WAITING_THE_MO)
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
#[doc = "Values that can be written to the field `MONOV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVW {
    #[doc = "No overrun. Monitor data has not overrun."]
    NO_OVERRUN_MONITOR_,
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    OVERRUN_A_MONITOR_D,
}
impl MONOVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONOVW::NO_OVERRUN_MONITOR_ => false,
            MONOVW::OVERRUN_A_MONITOR_D => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONOVW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONOVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun. Monitor data has not overrun."]
    #[inline]
    pub fn no_overrun_monitor_(self) -> &'a mut W {
        self.variant(MONOVW::NO_OVERRUN_MONITOR_)
    }
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    #[inline]
    pub fn overrun_a_monitor_d(self) -> &'a mut W {
        self.variant(MONOVW::OVERRUN_A_MONITOR_D)
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
#[doc = "Values that can be written to the field `MONACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONACTIVEW {
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    INACTIVE_THE_MONITO,
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    ACTIVE_THE_MONITOR_,
}
impl MONACTIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONACTIVEW::INACTIVE_THE_MONITO => false,
            MONACTIVEW::ACTIVE_THE_MONITOR_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _MONACTIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONACTIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    #[inline]
    pub fn inactive_the_monito(self) -> &'a mut W {
        self.variant(MONACTIVEW::INACTIVE_THE_MONITO)
    }
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    #[inline]
    pub fn active_the_monitor_(self) -> &'a mut W {
        self.variant(MONACTIVEW::ACTIVE_THE_MONITOR_)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEW {
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NOT_IDLE_THE_I2C_BU,
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    IDLE_THE_I2C_BUS_HA,
}
impl MONIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONIDLEW::NOT_IDLE_THE_I2C_BU => false,
            MONIDLEW::IDLE_THE_I2C_BUS_HA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MONIDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONIDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    #[inline]
    pub fn not_idle_the_i2c_bu(self) -> &'a mut W {
        self.variant(MONIDLEW::NOT_IDLE_THE_I2C_BU)
    }
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    #[inline]
    pub fn idle_the_i2c_bus_ha(self) -> &'a mut W {
        self.variant(MONIDLEW::IDLE_THE_I2C_BUS_HA)
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
#[doc = "Values that can be written to the field `EVENTTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTW {
    #[doc = "No time-out. I2C bus events have not caused a timeout."]
    NO_TIME_OUT_I2C_BUS,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the I2C Timeout register."]
    EVENT_TIME_OUT_THE_,
}
impl EVENTTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTW::NO_TIME_OUT_I2C_BUS => false,
            EVENTTIMEOUTW::EVENT_TIME_OUT_THE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVENTTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVENTTIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time-out. I2C bus events have not caused a timeout."]
    #[inline]
    pub fn no_time_out_i2c_bus(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTW::NO_TIME_OUT_I2C_BUS)
    }
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the I2C Timeout register."]
    #[inline]
    pub fn event_time_out_the_(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTW::EVENT_TIME_OUT_THE_)
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
#[doc = "Values that can be written to the field `SCLTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTW {
    #[doc = "No time-out. SCL low time has not caused a timeout."]
    NO_TIME_OUT_SCL_LOW,
    #[doc = "Time-out. SCL low time has caused a timeout."]
    TIME_OUT_SCL_LOW_TI,
}
impl SCLTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTW::NO_TIME_OUT_SCL_LOW => false,
            SCLTIMEOUTW::TIME_OUT_SCL_LOW_TI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLTIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time-out. SCL low time has not caused a timeout."]
    #[inline]
    pub fn no_time_out_scl_low(self) -> &'a mut W {
        self.variant(SCLTIMEOUTW::NO_TIME_OUT_SCL_LOW)
    }
    #[doc = "Time-out. SCL low time has caused a timeout."]
    #[inline]
    pub fn time_out_scl_low_ti(self) -> &'a mut W {
        self.variant(SCLTIMEOUTW::TIME_OUT_SCL_LOW_TI)
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
    #[doc = "Bit 0 - Master Pending. Indicates whether the Master function needs software service. This flag will cause an interrupt when set if enabled via the INTENSET register. The MSTPENDING flag is automatically cleared when a 1 is written to the MSTCONTINUE bit in the MSTCTL register."]
    #[inline]
    pub fn mstpending(&self) -> MSTPENDINGR {
        MSTPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Master State code. Each value of this field indicates a specific required service for the Master function. All other values are reserved."]
    #[inline]
    pub fn mststate(&self) -> MSTSTATER {
        MSTSTATER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline]
    pub fn mstarbloss(&self) -> MSTARBLOSSR {
        MSTARBLOSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MstContinue."]
    #[inline]
    pub fn mstststperr(&self) -> MSTSTSTPERRR {
        MSTSTSTPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Slave Pending. Indicates whether the Slave function needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the MSTCTL register."]
    #[inline]
    pub fn slvpending(&self) -> SLVPENDINGR {
        SLVPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved."]
    #[inline]
    pub fn slvstate(&self) -> SLVSTATER {
        SLVSTATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline]
    pub fn slvnotstr(&self) -> SLVNOTSTRR {
        SLVNOTSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline]
    pub fn slvidx(&self) -> SLVIDXR {
        SLVIDXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to Nack a matched address, or when there is a Stop detected on the bus. SLVSEL is not cleared if software Nacks data."]
    #[inline]
    pub fn slvsel(&self) -> SLVSELR {
        SLVSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn slvdesel(&self) -> SLVDESELR {
        SLVDESELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline]
    pub fn monrdy(&self) -> MONRDYR {
        MONRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline]
    pub fn monov(&self) -> MONOVR {
        MONOVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Monitor Active flag. This flag indicates when the Monitor function considers the I2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline]
    pub fn monactive(&self) -> MONACTIVER {
        MONACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register . The flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn monidle(&self) -> MONIDLER {
        MONIDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The case of SCL remaining low longer than TIMEOUT is not reported by this flag, it is reported in by the SCL Time-out flag. The flag is cleared by writing a 1 to this bit."]
    #[inline]
    pub fn eventtimeout(&self) -> EVENTTIMEOUTR {
        EVENTTIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline]
    pub fn scltimeout(&self) -> SCLTIMEOUTR {
        SCLTIMEOUTR::_from({
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
        W { bits: 2049 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Pending. Indicates whether the Master function needs software service. This flag will cause an interrupt when set if enabled via the INTENSET register. The MSTPENDING flag is automatically cleared when a 1 is written to the MSTCONTINUE bit in the MSTCTL register."]
    #[inline]
    pub fn mstpending(&mut self) -> _MSTPENDINGW {
        _MSTPENDINGW { w: self }
    }
    #[doc = "Bits 1:3 - Master State code. Each value of this field indicates a specific required service for the Master function. All other values are reserved."]
    #[inline]
    pub fn mststate(&mut self) -> _MSTSTATEW {
        _MSTSTATEW { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline]
    pub fn mstarbloss(&mut self) -> _MSTARBLOSSW {
        _MSTARBLOSSW { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MstContinue."]
    #[inline]
    pub fn mstststperr(&mut self) -> _MSTSTSTPERRW {
        _MSTSTSTPERRW { w: self }
    }
    #[doc = "Bit 8 - Slave Pending. Indicates whether the Slave function needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the MSTCTL register."]
    #[inline]
    pub fn slvpending(&mut self) -> _SLVPENDINGW {
        _SLVPENDINGW { w: self }
    }
    #[doc = "Bits 9:10 - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved."]
    #[inline]
    pub fn slvstate(&mut self) -> _SLVSTATEW {
        _SLVSTATEW { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline]
    pub fn slvnotstr(&mut self) -> _SLVNOTSTRW {
        _SLVNOTSTRW { w: self }
    }
    #[doc = "Bits 12:13 - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline]
    pub fn slvidx(&mut self) -> _SLVIDXW {
        _SLVIDXW { w: self }
    }
    #[doc = "Bit 14 - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to Nack a matched address, or when there is a Stop detected on the bus. SLVSEL is not cleared if software Nacks data."]
    #[inline]
    pub fn slvsel(&mut self) -> _SLVSELW {
        _SLVSELW { w: self }
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn slvdesel(&mut self) -> _SLVDESELW {
        _SLVDESELW { w: self }
    }
    #[doc = "Bit 16 - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline]
    pub fn monrdy(&mut self) -> _MONRDYW {
        _MONRDYW { w: self }
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline]
    pub fn monov(&mut self) -> _MONOVW {
        _MONOVW { w: self }
    }
    #[doc = "Bit 18 - Monitor Active flag. This flag indicates when the Monitor function considers the I2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline]
    pub fn monactive(&mut self) -> _MONACTIVEW {
        _MONACTIVEW { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register . The flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn monidle(&mut self) -> _MONIDLEW {
        _MONIDLEW { w: self }
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The case of SCL remaining low longer than TIMEOUT is not reported by this flag, it is reported in by the SCL Time-out flag. The flag is cleared by writing a 1 to this bit."]
    #[inline]
    pub fn eventtimeout(&mut self) -> _EVENTTIMEOUTW {
        _EVENTTIMEOUTW { w: self }
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline]
    pub fn scltimeout(&mut self) -> _SCLTIMEOUTW {
        _SCLTIMEOUTW { w: self }
    }
}
