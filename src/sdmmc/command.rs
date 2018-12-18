#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::COMMAND {
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
#[doc = r" Value of the field"]
pub struct CMD_INDR {
    bits: u8,
}
impl CMD_INDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMD_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_TYPER {
    #[doc = "Normal"]
    VALUE1,
    #[doc = "Suspend"]
    VALUE2,
    #[doc = "Resume"]
    VALUE3,
    #[doc = "Abort"]
    VALUE4,
}
impl CMD_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMD_TYPER::VALUE1 => 0,
            CMD_TYPER::VALUE2 => 1,
            CMD_TYPER::VALUE3 => 2,
            CMD_TYPER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMD_TYPER {
        match value {
            0 => CMD_TYPER::VALUE1,
            1 => CMD_TYPER::VALUE2,
            2 => CMD_TYPER::VALUE3,
            3 => CMD_TYPER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_TYPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_TYPER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMD_TYPER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CMD_TYPER::VALUE4
    }
}
#[doc = "Possible values of the field `DATA_PRESENT_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_PRESENT_SELECTR {
    #[doc = "No Data Present"]
    VALUE1,
    #[doc = "Data Present"]
    VALUE2,
}
impl DATA_PRESENT_SELECTR {
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
            DATA_PRESENT_SELECTR::VALUE1 => false,
            DATA_PRESENT_SELECTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_PRESENT_SELECTR {
        match value {
            false => DATA_PRESENT_SELECTR::VALUE1,
            true => DATA_PRESENT_SELECTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_PRESENT_SELECTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_PRESENT_SELECTR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_IND_CHECK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_IND_CHECK_ENR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl CMD_IND_CHECK_ENR {
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
            CMD_IND_CHECK_ENR::VALUE1 => false,
            CMD_IND_CHECK_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_IND_CHECK_ENR {
        match value {
            false => CMD_IND_CHECK_ENR::VALUE1,
            true => CMD_IND_CHECK_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_IND_CHECK_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_IND_CHECK_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_CRC_CHECK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_CRC_CHECK_ENR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl CMD_CRC_CHECK_ENR {
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
            CMD_CRC_CHECK_ENR::VALUE1 => false,
            CMD_CRC_CHECK_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_CRC_CHECK_ENR {
        match value {
            false => CMD_CRC_CHECK_ENR::VALUE1,
            true => CMD_CRC_CHECK_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_CRC_CHECK_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_CRC_CHECK_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `RESP_TYPE_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESP_TYPE_SELECTR {
    #[doc = "No Response"]
    VALUE1,
    #[doc = "Response length 136"]
    VALUE2,
    #[doc = "Response length 48"]
    VALUE3,
    #[doc = "Response length 48 check Busy after response"]
    VALUE4,
}
impl RESP_TYPE_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESP_TYPE_SELECTR::VALUE1 => 0,
            RESP_TYPE_SELECTR::VALUE2 => 1,
            RESP_TYPE_SELECTR::VALUE3 => 2,
            RESP_TYPE_SELECTR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESP_TYPE_SELECTR {
        match value {
            0 => RESP_TYPE_SELECTR::VALUE1,
            1 => RESP_TYPE_SELECTR::VALUE2,
            2 => RESP_TYPE_SELECTR::VALUE3,
            3 => RESP_TYPE_SELECTR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESP_TYPE_SELECTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESP_TYPE_SELECTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RESP_TYPE_SELECTR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RESP_TYPE_SELECTR::VALUE4
    }
}
#[doc = r" Proxy"]
pub struct _CMD_INDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_INDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD_TYPE`"]
pub enum CMD_TYPEW {
    #[doc = "Normal"]
    VALUE1,
    #[doc = "Suspend"]
    VALUE2,
    #[doc = "Resume"]
    VALUE3,
    #[doc = "Abort"]
    VALUE4,
}
impl CMD_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMD_TYPEW::VALUE1 => 0,
            CMD_TYPEW::VALUE2 => 1,
            CMD_TYPEW::VALUE3 => 2,
            CMD_TYPEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_TYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_TYPEW::VALUE1)
    }
    #[doc = "Suspend"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_TYPEW::VALUE2)
    }
    #[doc = "Resume"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMD_TYPEW::VALUE3)
    }
    #[doc = "Abort"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMD_TYPEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_PRESENT_SELECT`"]
pub enum DATA_PRESENT_SELECTW {
    #[doc = "No Data Present"]
    VALUE1,
    #[doc = "Data Present"]
    VALUE2,
}
impl DATA_PRESENT_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_PRESENT_SELECTW::VALUE1 => false,
            DATA_PRESENT_SELECTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_PRESENT_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_PRESENT_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_PRESENT_SELECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Data Present"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_PRESENT_SELECTW::VALUE1)
    }
    #[doc = "Data Present"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_PRESENT_SELECTW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD_IND_CHECK_EN`"]
pub enum CMD_IND_CHECK_ENW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl CMD_IND_CHECK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_IND_CHECK_ENW::VALUE1 => false,
            CMD_IND_CHECK_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_IND_CHECK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_IND_CHECK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_IND_CHECK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_IND_CHECK_ENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_IND_CHECK_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD_CRC_CHECK_EN`"]
pub enum CMD_CRC_CHECK_ENW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl CMD_CRC_CHECK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_CRC_CHECK_ENW::VALUE1 => false,
            CMD_CRC_CHECK_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_CRC_CHECK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_CRC_CHECK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_CRC_CHECK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_CRC_CHECK_ENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_CRC_CHECK_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESP_TYPE_SELECT`"]
pub enum RESP_TYPE_SELECTW {
    #[doc = "No Response"]
    VALUE1,
    #[doc = "Response length 136"]
    VALUE2,
    #[doc = "Response length 48"]
    VALUE3,
    #[doc = "Response length 48 check Busy after response"]
    VALUE4,
}
impl RESP_TYPE_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESP_TYPE_SELECTW::VALUE1 => 0,
            RESP_TYPE_SELECTW::VALUE2 => 1,
            RESP_TYPE_SELECTW::VALUE3 => 2,
            RESP_TYPE_SELECTW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESP_TYPE_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESP_TYPE_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESP_TYPE_SELECTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Response"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECTW::VALUE1)
    }
    #[doc = "Response length 136"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECTW::VALUE2)
    }
    #[doc = "Response length 48"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECTW::VALUE3)
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECTW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline]
    pub fn cmd_ind(&self) -> CMD_INDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CMD_INDR { bits }
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline]
    pub fn cmd_type(&self) -> CMD_TYPER {
        CMD_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline]
    pub fn data_present_select(&self) -> DATA_PRESENT_SELECTR {
        DATA_PRESENT_SELECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline]
    pub fn cmd_ind_check_en(&self) -> CMD_IND_CHECK_ENR {
        CMD_IND_CHECK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline]
    pub fn cmd_crc_check_en(&self) -> CMD_CRC_CHECK_ENR {
        CMD_CRC_CHECK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline]
    pub fn resp_type_select(&self) -> RESP_TYPE_SELECTR {
        RESP_TYPE_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline]
    pub fn cmd_ind(&mut self) -> _CMD_INDW {
        _CMD_INDW { w: self }
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline]
    pub fn cmd_type(&mut self) -> _CMD_TYPEW {
        _CMD_TYPEW { w: self }
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline]
    pub fn data_present_select(&mut self) -> _DATA_PRESENT_SELECTW {
        _DATA_PRESENT_SELECTW { w: self }
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline]
    pub fn cmd_ind_check_en(&mut self) -> _CMD_IND_CHECK_ENW {
        _CMD_IND_CHECK_ENW { w: self }
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline]
    pub fn cmd_crc_check_en(&mut self) -> _CMD_CRC_CHECK_ENW {
        _CMD_CRC_CHECK_ENW { w: self }
    }
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline]
    pub fn resp_type_select(&mut self) -> _RESP_TYPE_SELECTW {
        _RESP_TYPE_SELECTW { w: self }
    }
}
