#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXDAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXDATR {
    bits: u16,
}
impl RXDATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXSSELNR {
    bits: bool,
}
impl RXSSELNR {
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
pub struct SOTR {
    bits: bool,
}
impl SOTR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Receiver Data. This contains the next piece of received data. The number of bits that are used depends on the FLen setting in TXCTL / TXDATCTL."]
    #[inline]
    pub fn rxdat(&self) -> RXDATR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RXDATR { bits }
    }
    #[doc = "Bit 16 - Slave Select for receive. This field allows the state of the SSEL pin to be saved along with received data. The value will reflect the SSEL pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline]
    pub fn rxsseln(&self) -> RXSSELNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXSSELNR { bits }
    }
    #[doc = "Bit 20 - Start of Transfer flag. This flag will be 1 if this is the first frame after SSEL went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the frame length is greater than 16 bit."]
    #[inline]
    pub fn sot(&self) -> SOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOTR { bits }
    }
}
