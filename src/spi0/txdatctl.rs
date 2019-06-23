#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXDATCTL {
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
#[doc = r" Value of the field"]
pub struct TXDATR {
    bits: u16,
}
impl TXDATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `TXSSELN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSELNR {
    #[doc = "SSEL  asserted."]
    SSEL_ASSERTED_,
    #[doc = "SSEL not asserted."]
    SSEL_NOT_ASSERTED_,
}
impl TXSSELNR {
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
            TXSSELNR::SSEL_ASSERTED_ => false,
            TXSSELNR::SSEL_NOT_ASSERTED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSSELNR {
        match value {
            false => TXSSELNR::SSEL_ASSERTED_,
            true => TXSSELNR::SSEL_NOT_ASSERTED_,
        }
    }
    #[doc = "Checks if the value of the field is `SSEL_ASSERTED_`"]
    #[inline]
    pub fn is_ssel_asserted_(&self) -> bool {
        *self == TXSSELNR::SSEL_ASSERTED_
    }
    #[doc = "Checks if the value of the field is `SSEL_NOT_ASSERTED_`"]
    #[inline]
    pub fn is_ssel_not_asserted_(&self) -> bool {
        *self == TXSSELNR::SSEL_NOT_ASSERTED_
    }
}
#[doc = "Possible values of the field `EOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTR {
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    SSEL_NOT_DEASSERTED_,
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSELs will be deasserted at the end of this piece of data."]
    SSEL_DEASSERTED_THI,
}
impl EOTR {
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
            EOTR::SSEL_NOT_DEASSERTED_ => false,
            EOTR::SSEL_DEASSERTED_THI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOTR {
        match value {
            false => EOTR::SSEL_NOT_DEASSERTED_,
            true => EOTR::SSEL_DEASSERTED_THI,
        }
    }
    #[doc = "Checks if the value of the field is `SSEL_NOT_DEASSERTED_`"]
    #[inline]
    pub fn is_ssel_not_deasserted_(&self) -> bool {
        *self == EOTR::SSEL_NOT_DEASSERTED_
    }
    #[doc = "Checks if the value of the field is `SSEL_DEASSERTED_THI`"]
    #[inline]
    pub fn is_ssel_deasserted_thi(&self) -> bool {
        *self == EOTR::SSEL_DEASSERTED_THI
    }
}
#[doc = "Possible values of the field `EOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOFR {
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    DATA_NOT_EOF_THIS_P,
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    DATA_EOF_THIS_PIECE,
}
impl EOFR {
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
            EOFR::DATA_NOT_EOF_THIS_P => false,
            EOFR::DATA_EOF_THIS_PIECE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOFR {
        match value {
            false => EOFR::DATA_NOT_EOF_THIS_P,
            true => EOFR::DATA_EOF_THIS_PIECE,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_NOT_EOF_THIS_P`"]
    #[inline]
    pub fn is_data_not_eof_this_p(&self) -> bool {
        *self == EOFR::DATA_NOT_EOF_THIS_P
    }
    #[doc = "Checks if the value of the field is `DATA_EOF_THIS_PIECE`"]
    #[inline]
    pub fn is_data_eof_this_piece(&self) -> bool {
        *self == EOFR::DATA_EOF_THIS_PIECE
    }
}
#[doc = "Possible values of the field `RXIGNORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIGNORER {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ_RECEIVED_DATA_,
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE_RECEIVED_DATA,
}
impl RXIGNORER {
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
            RXIGNORER::READ_RECEIVED_DATA_ => false,
            RXIGNORER::IGNORE_RECEIVED_DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIGNORER {
        match value {
            false => RXIGNORER::READ_RECEIVED_DATA_,
            true => RXIGNORER::IGNORE_RECEIVED_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `READ_RECEIVED_DATA_`"]
    #[inline]
    pub fn is_read_received_data_(&self) -> bool {
        *self == RXIGNORER::READ_RECEIVED_DATA_
    }
    #[doc = "Checks if the value of the field is `IGNORE_RECEIVED_DATA`"]
    #[inline]
    pub fn is_ignore_received_data(&self) -> bool {
        *self == RXIGNORER::IGNORE_RECEIVED_DATA
    }
}
#[doc = r" Value of the field"]
pub struct FLENR {
    bits: u8,
}
impl FLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TXDATW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSELN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSELNW {
    #[doc = "SSEL  asserted."]
    SSEL_ASSERTED_,
    #[doc = "SSEL not asserted."]
    SSEL_NOT_ASSERTED_,
}
impl TXSSELNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSELNW::SSEL_ASSERTED_ => false,
            TXSSELNW::SSEL_NOT_ASSERTED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSSELNW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSELNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSSELNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL asserted."]
    #[inline]
    pub fn ssel_asserted_(self) -> &'a mut W {
        self.variant(TXSSELNW::SSEL_ASSERTED_)
    }
    #[doc = "SSEL not asserted."]
    #[inline]
    pub fn ssel_not_asserted_(self) -> &'a mut W {
        self.variant(TXSSELNW::SSEL_NOT_ASSERTED_)
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
#[doc = "Values that can be written to the field `EOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTW {
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    SSEL_NOT_DEASSERTED_,
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSELs will be deasserted at the end of this piece of data."]
    SSEL_DEASSERTED_THI,
}
impl EOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOTW::SSEL_NOT_DEASSERTED_ => false,
            EOTW::SSEL_DEASSERTED_THI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOTW<'a> {
    w: &'a mut W,
}
impl<'a> _EOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline]
    pub fn ssel_not_deasserted_(self) -> &'a mut W {
        self.variant(EOTW::SSEL_NOT_DEASSERTED_)
    }
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSELs will be deasserted at the end of this piece of data."]
    #[inline]
    pub fn ssel_deasserted_thi(self) -> &'a mut W {
        self.variant(EOTW::SSEL_DEASSERTED_THI)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOFW {
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    DATA_NOT_EOF_THIS_P,
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    DATA_EOF_THIS_PIECE,
}
impl EOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOFW::DATA_NOT_EOF_THIS_P => false,
            EOFW::DATA_EOF_THIS_PIECE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    #[inline]
    pub fn data_not_eof_this_p(self) -> &'a mut W {
        self.variant(EOFW::DATA_NOT_EOF_THIS_P)
    }
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    #[inline]
    pub fn data_eof_this_piece(self) -> &'a mut W {
        self.variant(EOFW::DATA_EOF_THIS_PIECE)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIGNORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIGNOREW {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ_RECEIVED_DATA_,
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE_RECEIVED_DATA,
}
impl RXIGNOREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIGNOREW::READ_RECEIVED_DATA_ => false,
            RXIGNOREW::IGNORE_RECEIVED_DATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIGNOREW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIGNOREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIGNOREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline]
    pub fn read_received_data_(self) -> &'a mut W {
        self.variant(RXIGNOREW::READ_RECEIVED_DATA_)
    }
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline]
    pub fn ignore_received_data(self) -> &'a mut W {
        self.variant(RXIGNOREW::IGNORE_RECEIVED_DATA)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:15 - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
    #[inline]
    pub fn txdat(&self) -> TXDATR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TXDATR { bits }
    }
    #[doc = "Bit 16 - Transmit Slave Select . This field controls what is output for SSEL in master mode. The active state of the SSEL function is configured by bits in the CFG register."]
    #[inline]
    pub fn txsseln(&self) -> TXSSELNR {
        TXSSELNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline]
    pub fn eot(&self) -> EOTR {
        EOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - End of Frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline]
    pub fn eof(&self) -> EOFR {
        EOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver to simplify the transmit process."]
    #[inline]
    pub fn rxignore(&self) -> RXIGNORER {
        RXIGNORER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Frame Length. Specifies the frame length from 1 to 16 bits. Note that frame lengths greater than 16 bits are supported by multiple sequential frames Note that if a 1-bit frame is selected, the master function will always insert a delay with a length of one SCK time following the single clock seen on the SCK pin. 0x0 = Data frame is 1 bit in length. 0x1 = Data frame is 1 bit in length. 0x2 = Data frame is 3 bits in length. ... 0xF = Data frame is 16 bits in length."]
    #[inline]
    pub fn flen(&self) -> FLENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLENR { bits }
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
    #[doc = "Bits 0:15 - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
    #[inline]
    pub fn txdat(&mut self) -> _TXDATW {
        _TXDATW { w: self }
    }
    #[doc = "Bit 16 - Transmit Slave Select . This field controls what is output for SSEL in master mode. The active state of the SSEL function is configured by bits in the CFG register."]
    #[inline]
    pub fn txsseln(&mut self) -> _TXSSELNW {
        _TXSSELNW { w: self }
    }
    #[doc = "Bit 20 - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline]
    pub fn eot(&mut self) -> _EOTW {
        _EOTW { w: self }
    }
    #[doc = "Bit 21 - End of Frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline]
    pub fn eof(&mut self) -> _EOFW {
        _EOFW { w: self }
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver to simplify the transmit process."]
    #[inline]
    pub fn rxignore(&mut self) -> _RXIGNOREW {
        _RXIGNOREW { w: self }
    }
    #[doc = "Bits 24:27 - Frame Length. Specifies the frame length from 1 to 16 bits. Note that frame lengths greater than 16 bits are supported by multiple sequential frames Note that if a 1-bit frame is selected, the master function will always insert a delay with a length of one SCK time following the single clock seen on the SCK pin. 0x0 = Data frame is 1 bit in length. 0x1 = Data frame is 1 bit in length. 0x2 = Data frame is 3 bits in length. ... 0xF = Data frame is 16 bits in length."]
    #[inline]
    pub fn flen(&mut self) -> _FLENW {
        _FLENW { w: self }
    }
}
