#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCTR {
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
        R { bits: self.register.get() }
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
#[doc = "Possible values of the field `SDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIRR {
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    VALUE1,
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    VALUE2,
}
impl SDIRR {
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
            SDIRR::VALUE1 => false,
            SDIRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDIRR {
        match value {
            false => SDIRR::VALUE1,
            true => SDIRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDIRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDIRR::VALUE2
    }
}
#[doc = "Possible values of the field `PDL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDLR {
    #[doc = "The passive data level is 0."]
    VALUE1,
    #[doc = "The passive data level is 1."]
    VALUE2,
}
impl PDLR {
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
            PDLR::VALUE1 => false,
            PDLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDLR {
        match value {
            false => PDLR::VALUE1,
            true => PDLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDLR::VALUE2
    }
}
#[doc = "Possible values of the field `DSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSMR {
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    VALUE1,
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\] respectively."]
    VALUE3,
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\] respectively."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSMR::VALUE1 => 0,
            DSMR::VALUE3 => 2,
            DSMR::VALUE4 => 3,
            DSMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSMR {
        match value {
            0 => DSMR::VALUE1,
            2 => DSMR::VALUE3,
            3 => DSMR::VALUE4,
            i => DSMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DSMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DSMR::VALUE4
    }
}
#[doc = "Possible values of the field `HPCDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCDIRR {
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    VALUE1,
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    VALUE2,
}
impl HPCDIRR {
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
            HPCDIRR::VALUE1 => false,
            HPCDIRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPCDIRR {
        match value {
            false => HPCDIRR::VALUE1,
            true => HPCDIRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HPCDIRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HPCDIRR::VALUE2
    }
}
#[doc = "Possible values of the field `DOCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOCFGR {
    #[doc = "DOUTx = shift data value"]
    VALUE1,
    #[doc = "DOUTx = inverted shift data value"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DOCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DOCFGR::VALUE1 => 0,
            DOCFGR::VALUE2 => 1,
            DOCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DOCFGR {
        match value {
            0 => DOCFGR::VALUE1,
            1 => DOCFGR::VALUE2,
            i => DOCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DOCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DOCFGR::VALUE2
    }
}
#[doc = "Possible values of the field `TRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRMR {
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    VALUE1,
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    VALUE2,
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    VALUE3,
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    VALUE4,
}
impl TRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRMR::VALUE1 => 0,
            TRMR::VALUE2 => 1,
            TRMR::VALUE3 => 2,
            TRMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRMR {
        match value {
            0 => TRMR::VALUE1,
            1 => TRMR::VALUE2,
            2 => TRMR::VALUE3,
            3 => TRMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TRMR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct FLER {
    bits: u8,
}
impl FLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLER {
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    VALUE1,
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    VALUE2,
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    VALUE3,
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLER::VALUE1 => 0,
            WLER::VALUE2 => 1,
            WLER::VALUE3 => 14,
            WLER::VALUE4 => 15,
            WLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLER {
        match value {
            0 => WLER::VALUE1,
            1 => WLER::VALUE2,
            14 => WLER::VALUE3,
            15 => WLER::VALUE4,
            i => WLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WLER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == WLER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == WLER::VALUE4
    }
}
#[doc = "Values that can be written to the field `SDIR`"]
pub enum SDIRW {
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    VALUE1,
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    VALUE2,
}
impl SDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDIRW::VALUE1 => false,
            SDIRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDIRW::VALUE1)
    }
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDIRW::VALUE2)
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
#[doc = "Values that can be written to the field `PDL`"]
pub enum PDLW {
    #[doc = "The passive data level is 0."]
    VALUE1,
    #[doc = "The passive data level is 1."]
    VALUE2,
}
impl PDLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDLW::VALUE1 => false,
            PDLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDLW<'a> {
    w: &'a mut W,
}
impl<'a> _PDLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The passive data level is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDLW::VALUE1)
    }
    #[doc = "The passive data level is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDLW::VALUE2)
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
#[doc = "Values that can be written to the field `DSM`"]
pub enum DSMW {
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    VALUE1,
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\] respectively."]
    VALUE3,
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\] respectively."]
    VALUE4,
}
impl DSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSMW::VALUE1 => 0,
            DSMW::VALUE3 => 2,
            DSMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSMW<'a> {
    w: &'a mut W,
}
impl<'a> _DSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSMW::VALUE1)
    }
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\] respectively."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSMW::VALUE3)
    }
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\] respectively."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DSMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPCDIR`"]
pub enum HPCDIRW {
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    VALUE1,
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    VALUE2,
}
impl HPCDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPCDIRW::VALUE1 => false,
            HPCDIRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPCDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _HPCDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPCDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HPCDIRW::VALUE1)
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HPCDIRW::VALUE2)
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
#[doc = "Values that can be written to the field `DOCFG`"]
pub enum DOCFGW {
    #[doc = "DOUTx = shift data value"]
    VALUE1,
    #[doc = "DOUTx = inverted shift data value"]
    VALUE2,
}
impl DOCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DOCFGW::VALUE1 => 0,
            DOCFGW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _DOCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DOUTx = shift data value"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DOCFGW::VALUE1)
    }
    #[doc = "DOUTx = inverted shift data value"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DOCFGW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRM`"]
pub enum TRMW {
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    VALUE1,
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    VALUE2,
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    VALUE3,
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    VALUE4,
}
impl TRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRMW::VALUE1 => 0,
            TRMW::VALUE2 => 1,
            TRMW::VALUE3 => 2,
            TRMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRMW::VALUE1)
    }
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRMW::VALUE2)
    }
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRMW::VALUE3)
    }
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRMW::VALUE4)
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
#[doc = r" Proxy"]
pub struct _FLEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WLE`"]
pub enum WLEW {
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    VALUE1,
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    VALUE2,
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    VALUE3,
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    VALUE4,
}
impl WLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLEW::VALUE1 => 0,
            WLEW::VALUE2 => 1,
            WLEW::VALUE3 => 14,
            WLEW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WLEW::VALUE1)
    }
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WLEW::VALUE2)
    }
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(WLEW::VALUE3)
    }
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(WLEW::VALUE4)
    }
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
    #[doc = "Bit 0 - Shift Direction"]
    #[inline]
    pub fn sdir(&self) -> SDIRR {
        SDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline]
    pub fn pdl(&self) -> PDLR {
        PDLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline]
    pub fn dsm(&self) -> DSMR {
        DSMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline]
    pub fn hpcdir(&self) -> HPCDIRR {
        HPCDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline]
    pub fn docfg(&self) -> DOCFGR {
        DOCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline]
    pub fn trm(&self) -> TRMR {
        TRMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline]
    pub fn fle(&self) -> FLER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLER { bits }
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline]
    pub fn wle(&self) -> WLER {
        WLER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Shift Direction"]
    #[inline]
    pub fn sdir(&mut self) -> _SDIRW {
        _SDIRW { w: self }
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline]
    pub fn pdl(&mut self) -> _PDLW {
        _PDLW { w: self }
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline]
    pub fn dsm(&mut self) -> _DSMW {
        _DSMW { w: self }
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline]
    pub fn hpcdir(&mut self) -> _HPCDIRW {
        _HPCDIRW { w: self }
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline]
    pub fn docfg(&mut self) -> _DOCFGW {
        _DOCFGW { w: self }
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline]
    pub fn trm(&mut self) -> _TRMW {
        _TRMW { w: self }
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline]
    pub fn fle(&mut self) -> _FLEW {
        _FLEW { w: self }
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline]
    pub fn wle(&mut self) -> _WLEW {
        _WLEW { w: self }
    }
}
