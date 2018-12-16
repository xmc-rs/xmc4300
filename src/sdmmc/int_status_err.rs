#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::INT_STATUS_ERR {
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
#[doc = "Possible values of the field `CEATA_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEATA_ERRR {
    #[doc = "no error"]
    VALUE1,
    #[doc = "error"]
    VALUE2,
}
impl CEATA_ERRR {
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
            CEATA_ERRR::VALUE1 => false,
            CEATA_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEATA_ERRR {
        match value {
            false => CEATA_ERRR::VALUE1,
            true => CEATA_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEATA_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEATA_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl ACMD_ERRR {
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
            ACMD_ERRR::VALUE1 => false,
            ACMD_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD_ERRR {
        match value {
            false => ACMD_ERRR::VALUE1,
            true => ACMD_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `CURRENT_LIMIT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRENT_LIMIT_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Power Fail"]
    VALUE2,
}
impl CURRENT_LIMIT_ERRR {
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
            CURRENT_LIMIT_ERRR::VALUE1 => false,
            CURRENT_LIMIT_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CURRENT_LIMIT_ERRR {
        match value {
            false => CURRENT_LIMIT_ERRR::VALUE1,
            true => CURRENT_LIMIT_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CURRENT_LIMIT_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CURRENT_LIMIT_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `DATA_END_BIT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_END_BIT_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl DATA_END_BIT_ERRR {
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
            DATA_END_BIT_ERRR::VALUE1 => false,
            DATA_END_BIT_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_END_BIT_ERRR {
        match value {
            false => DATA_END_BIT_ERRR::VALUE1,
            true => DATA_END_BIT_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_END_BIT_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_END_BIT_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `DATA_CRC_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_CRC_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl DATA_CRC_ERRR {
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
            DATA_CRC_ERRR::VALUE1 => false,
            DATA_CRC_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_CRC_ERRR {
        match value {
            false => DATA_CRC_ERRR::VALUE1,
            true => DATA_CRC_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_CRC_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_CRC_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `DATA_TIMEOUT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_TIMEOUT_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Timeout"]
    VALUE2,
}
impl DATA_TIMEOUT_ERRR {
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
            DATA_TIMEOUT_ERRR::VALUE1 => false,
            DATA_TIMEOUT_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_TIMEOUT_ERRR {
        match value {
            false => DATA_TIMEOUT_ERRR::VALUE1,
            true => DATA_TIMEOUT_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_TIMEOUT_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_TIMEOUT_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_IND_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_IND_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl CMD_IND_ERRR {
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
            CMD_IND_ERRR::VALUE1 => false,
            CMD_IND_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_IND_ERRR {
        match value {
            false => CMD_IND_ERRR::VALUE1,
            true => CMD_IND_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_IND_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_IND_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_END_BIT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_END_BIT_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "End Bit Error Generated"]
    VALUE2,
}
impl CMD_END_BIT_ERRR {
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
            CMD_END_BIT_ERRR::VALUE1 => false,
            CMD_END_BIT_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_END_BIT_ERRR {
        match value {
            false => CMD_END_BIT_ERRR::VALUE1,
            true => CMD_END_BIT_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_END_BIT_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_END_BIT_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_CRC_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_CRC_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "CRC Error Generated"]
    VALUE2,
}
impl CMD_CRC_ERRR {
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
            CMD_CRC_ERRR::VALUE1 => false,
            CMD_CRC_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_CRC_ERRR {
        match value {
            false => CMD_CRC_ERRR::VALUE1,
            true => CMD_CRC_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_CRC_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_CRC_ERRR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_TIMEOUT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_TIMEOUT_ERRR {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Timeout"]
    VALUE2,
}
impl CMD_TIMEOUT_ERRR {
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
            CMD_TIMEOUT_ERRR::VALUE1 => false,
            CMD_TIMEOUT_ERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_TIMEOUT_ERRR {
        match value {
            false => CMD_TIMEOUT_ERRR::VALUE1,
            true => CMD_TIMEOUT_ERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_TIMEOUT_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_TIMEOUT_ERRR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CEATA_ERR`"]
pub enum CEATA_ERRW {
    #[doc = "no error"]
    VALUE1,
    #[doc = "error"]
    VALUE2,
}
impl CEATA_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEATA_ERRW::VALUE1 => false,
            CEATA_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEATA_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CEATA_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEATA_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEATA_ERRW::VALUE1)
    }
    #[doc = "error"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEATA_ERRW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMD_ERR`"]
pub enum ACMD_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl ACMD_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMD_ERRW::VALUE1 => false,
            ACMD_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMD_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMD_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMD_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACMD_ERRW::VALUE1)
    }
    #[doc = "Error"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACMD_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CURRENT_LIMIT_ERR`"]
pub enum CURRENT_LIMIT_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Power Fail"]
    VALUE2,
}
impl CURRENT_LIMIT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CURRENT_LIMIT_ERRW::VALUE1 => false,
            CURRENT_LIMIT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CURRENT_LIMIT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CURRENT_LIMIT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CURRENT_LIMIT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERRW::VALUE1)
    }
    #[doc = "Power Fail"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_END_BIT_ERR`"]
pub enum DATA_END_BIT_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl DATA_END_BIT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_END_BIT_ERRW::VALUE1 => false,
            DATA_END_BIT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_END_BIT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_END_BIT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_END_BIT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERRW::VALUE1)
    }
    #[doc = "Error"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_CRC_ERR`"]
pub enum DATA_CRC_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl DATA_CRC_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_CRC_ERRW::VALUE1 => false,
            DATA_CRC_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_CRC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_CRC_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_CRC_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_CRC_ERRW::VALUE1)
    }
    #[doc = "Error"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_CRC_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `DATA_TIMEOUT_ERR`"]
pub enum DATA_TIMEOUT_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Timeout"]
    VALUE2,
}
impl DATA_TIMEOUT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_TIMEOUT_ERRW::VALUE1 => false,
            DATA_TIMEOUT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_TIMEOUT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_TIMEOUT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_TIMEOUT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERRW::VALUE1)
    }
    #[doc = "Timeout"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD_IND_ERR`"]
pub enum CMD_IND_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Error"]
    VALUE2,
}
impl CMD_IND_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_IND_ERRW::VALUE1 => false,
            CMD_IND_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_IND_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_IND_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_IND_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_IND_ERRW::VALUE1)
    }
    #[doc = "Error"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_IND_ERRW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD_END_BIT_ERR`"]
pub enum CMD_END_BIT_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "End Bit Error Generated"]
    VALUE2,
}
impl CMD_END_BIT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_END_BIT_ERRW::VALUE1 => false,
            CMD_END_BIT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_END_BIT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_END_BIT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_END_BIT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERRW::VALUE1)
    }
    #[doc = "End Bit Error Generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD_CRC_ERR`"]
pub enum CMD_CRC_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "CRC Error Generated"]
    VALUE2,
}
impl CMD_CRC_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_CRC_ERRW::VALUE1 => false,
            CMD_CRC_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_CRC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_CRC_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_CRC_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_CRC_ERRW::VALUE1)
    }
    #[doc = "CRC Error Generated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_CRC_ERRW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD_TIMEOUT_ERR`"]
pub enum CMD_TIMEOUT_ERRW {
    #[doc = "No Error"]
    VALUE1,
    #[doc = "Timeout"]
    VALUE2,
}
impl CMD_TIMEOUT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_TIMEOUT_ERRW::VALUE1 => false,
            CMD_TIMEOUT_ERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_TIMEOUT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_TIMEOUT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_TIMEOUT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERRW::VALUE1)
    }
    #[doc = "Timeout"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERRW::VALUE2)
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
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline]
    pub fn ceata_err(&self) -> CEATA_ERRR {
        CEATA_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline]
    pub fn acmd_err(&self) -> ACMD_ERRR {
        ACMD_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline]
    pub fn current_limit_err(&self) -> CURRENT_LIMIT_ERRR {
        CURRENT_LIMIT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline]
    pub fn data_end_bit_err(&self) -> DATA_END_BIT_ERRR {
        DATA_END_BIT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline]
    pub fn data_crc_err(&self) -> DATA_CRC_ERRR {
        DATA_CRC_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline]
    pub fn data_timeout_err(&self) -> DATA_TIMEOUT_ERRR {
        DATA_TIMEOUT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline]
    pub fn cmd_ind_err(&self) -> CMD_IND_ERRR {
        CMD_IND_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline]
    pub fn cmd_end_bit_err(&self) -> CMD_END_BIT_ERRR {
        CMD_END_BIT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline]
    pub fn cmd_crc_err(&self) -> CMD_CRC_ERRR {
        CMD_CRC_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline]
    pub fn cmd_timeout_err(&self) -> CMD_TIMEOUT_ERRR {
        CMD_TIMEOUT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline]
    pub fn ceata_err(&mut self) -> _CEATA_ERRW {
        _CEATA_ERRW { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline]
    pub fn acmd_err(&mut self) -> _ACMD_ERRW {
        _ACMD_ERRW { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline]
    pub fn current_limit_err(&mut self) -> _CURRENT_LIMIT_ERRW {
        _CURRENT_LIMIT_ERRW { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline]
    pub fn data_end_bit_err(&mut self) -> _DATA_END_BIT_ERRW {
        _DATA_END_BIT_ERRW { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline]
    pub fn data_crc_err(&mut self) -> _DATA_CRC_ERRW {
        _DATA_CRC_ERRW { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline]
    pub fn data_timeout_err(&mut self) -> _DATA_TIMEOUT_ERRW {
        _DATA_TIMEOUT_ERRW { w: self }
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline]
    pub fn cmd_ind_err(&mut self) -> _CMD_IND_ERRW {
        _CMD_IND_ERRW { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline]
    pub fn cmd_end_bit_err(&mut self) -> _CMD_END_BIT_ERRW {
        _CMD_END_BIT_ERRW { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline]
    pub fn cmd_crc_err(&mut self) -> _CMD_CRC_ERRW {
        _CMD_CRC_ERRW { w: self }
    }
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline]
    pub fn cmd_timeout_err(&mut self) -> _CMD_TIMEOUT_ERRW {
        _CMD_TIMEOUT_ERRW { w: self }
    }
}
