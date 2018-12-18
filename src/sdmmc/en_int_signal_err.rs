#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::EN_INT_SIGNAL_ERR {
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
#[doc = "Possible values of the field `CEATA_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEATA_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CEATA_ERR_ENR {
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
            CEATA_ERR_ENR::VALUE1 => false,
            CEATA_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEATA_ERR_ENR {
        match value {
            false => CEATA_ERR_ENR::VALUE1,
            true => CEATA_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEATA_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEATA_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `TARGET_RESP_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARGET_RESP_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl TARGET_RESP_ERR_ENR {
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
            TARGET_RESP_ERR_ENR::VALUE1 => false,
            TARGET_RESP_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TARGET_RESP_ERR_ENR {
        match value {
            false => TARGET_RESP_ERR_ENR::VALUE1,
            true => TARGET_RESP_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TARGET_RESP_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TARGET_RESP_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ACMD_ERR_ENR {
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
            ACMD_ERR_ENR::VALUE1 => false,
            ACMD_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD_ERR_ENR {
        match value {
            false => ACMD_ERR_ENR::VALUE1,
            true => ACMD_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CURRENT_LIMIT_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRENT_LIMIT_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CURRENT_LIMIT_ERR_ENR {
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
            CURRENT_LIMIT_ERR_ENR::VALUE1 => false,
            CURRENT_LIMIT_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CURRENT_LIMIT_ERR_ENR {
        match value {
            false => CURRENT_LIMIT_ERR_ENR::VALUE1,
            true => CURRENT_LIMIT_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CURRENT_LIMIT_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CURRENT_LIMIT_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `DATA_END_BIT_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_END_BIT_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl DATA_END_BIT_ERR_ENR {
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
            DATA_END_BIT_ERR_ENR::VALUE1 => false,
            DATA_END_BIT_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_END_BIT_ERR_ENR {
        match value {
            false => DATA_END_BIT_ERR_ENR::VALUE1,
            true => DATA_END_BIT_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_END_BIT_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_END_BIT_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `DATA_CRC_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_CRC_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl DATA_CRC_ERR_ENR {
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
            DATA_CRC_ERR_ENR::VALUE1 => false,
            DATA_CRC_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_CRC_ERR_ENR {
        match value {
            false => DATA_CRC_ERR_ENR::VALUE1,
            true => DATA_CRC_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_CRC_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_CRC_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `DATA_TIMEOUT_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_TIMEOUT_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl DATA_TIMEOUT_ERR_ENR {
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
            DATA_TIMEOUT_ERR_ENR::VALUE1 => false,
            DATA_TIMEOUT_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_TIMEOUT_ERR_ENR {
        match value {
            false => DATA_TIMEOUT_ERR_ENR::VALUE1,
            true => DATA_TIMEOUT_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_TIMEOUT_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_TIMEOUT_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_IND_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_IND_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_IND_ERR_ENR {
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
            CMD_IND_ERR_ENR::VALUE1 => false,
            CMD_IND_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_IND_ERR_ENR {
        match value {
            false => CMD_IND_ERR_ENR::VALUE1,
            true => CMD_IND_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_IND_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_IND_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_END_BIT_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_END_BIT_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_END_BIT_ERR_ENR {
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
            CMD_END_BIT_ERR_ENR::VALUE1 => false,
            CMD_END_BIT_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_END_BIT_ERR_ENR {
        match value {
            false => CMD_END_BIT_ERR_ENR::VALUE1,
            true => CMD_END_BIT_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_END_BIT_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_END_BIT_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_CRC_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_CRC_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_CRC_ERR_ENR {
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
            CMD_CRC_ERR_ENR::VALUE1 => false,
            CMD_CRC_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_CRC_ERR_ENR {
        match value {
            false => CMD_CRC_ERR_ENR::VALUE1,
            true => CMD_CRC_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_CRC_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_CRC_ERR_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_TIMEOUT_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_TIMEOUT_ERR_ENR {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_TIMEOUT_ERR_ENR {
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
            CMD_TIMEOUT_ERR_ENR::VALUE1 => false,
            CMD_TIMEOUT_ERR_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_TIMEOUT_ERR_ENR {
        match value {
            false => CMD_TIMEOUT_ERR_ENR::VALUE1,
            true => CMD_TIMEOUT_ERR_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_TIMEOUT_ERR_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_TIMEOUT_ERR_ENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CEATA_ERR_EN`"]
pub enum CEATA_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CEATA_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEATA_ERR_ENW::VALUE1 => false,
            CEATA_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEATA_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CEATA_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEATA_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEATA_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEATA_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `TARGET_RESP_ERR_EN`"]
pub enum TARGET_RESP_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl TARGET_RESP_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TARGET_RESP_ERR_ENW::VALUE1 => false,
            TARGET_RESP_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TARGET_RESP_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TARGET_RESP_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TARGET_RESP_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TARGET_RESP_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TARGET_RESP_ERR_ENW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMD_ERR_EN`"]
pub enum ACMD_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ACMD_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMD_ERR_ENW::VALUE1 => false,
            ACMD_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMD_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMD_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMD_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACMD_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACMD_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `CURRENT_LIMIT_ERR_EN`"]
pub enum CURRENT_LIMIT_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CURRENT_LIMIT_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CURRENT_LIMIT_ERR_ENW::VALUE1 => false,
            CURRENT_LIMIT_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CURRENT_LIMIT_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CURRENT_LIMIT_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CURRENT_LIMIT_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `DATA_END_BIT_ERR_EN`"]
pub enum DATA_END_BIT_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl DATA_END_BIT_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_END_BIT_ERR_ENW::VALUE1 => false,
            DATA_END_BIT_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_END_BIT_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_END_BIT_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_END_BIT_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `DATA_CRC_ERR_EN`"]
pub enum DATA_CRC_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl DATA_CRC_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_CRC_ERR_ENW::VALUE1 => false,
            DATA_CRC_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_CRC_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_CRC_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_CRC_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_CRC_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_CRC_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `DATA_TIMEOUT_ERR_EN`"]
pub enum DATA_TIMEOUT_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl DATA_TIMEOUT_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_TIMEOUT_ERR_ENW::VALUE1 => false,
            DATA_TIMEOUT_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_TIMEOUT_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_TIMEOUT_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_TIMEOUT_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD_IND_ERR_EN`"]
pub enum CMD_IND_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_IND_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_IND_ERR_ENW::VALUE1 => false,
            CMD_IND_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_IND_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_IND_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_IND_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_IND_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_IND_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD_END_BIT_ERR_EN`"]
pub enum CMD_END_BIT_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_END_BIT_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_END_BIT_ERR_ENW::VALUE1 => false,
            CMD_END_BIT_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_END_BIT_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_END_BIT_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_END_BIT_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD_CRC_ERR_EN`"]
pub enum CMD_CRC_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_CRC_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_CRC_ERR_ENW::VALUE1 => false,
            CMD_CRC_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_CRC_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_CRC_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_CRC_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_CRC_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_CRC_ERR_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `CMD_TIMEOUT_ERR_EN`"]
pub enum CMD_TIMEOUT_ERR_ENW {
    #[doc = "Masked"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl CMD_TIMEOUT_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_TIMEOUT_ERR_ENW::VALUE1 => false,
            CMD_TIMEOUT_ERR_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_TIMEOUT_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_TIMEOUT_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_TIMEOUT_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERR_ENW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERR_ENW::VALUE2)
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
    #[doc = "Bit 13 - Ceata Error Signal Enable"]
    #[inline]
    pub fn ceata_err_en(&self) -> CEATA_ERR_ENR {
        CEATA_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable"]
    #[inline]
    pub fn target_resp_err_en(&self) -> TARGET_RESP_ERR_ENR {
        TARGET_RESP_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Auto CMD12 Error Signal Enable"]
    #[inline]
    pub fn acmd_err_en(&self) -> ACMD_ERR_ENR {
        ACMD_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline]
    pub fn current_limit_err_en(&self) -> CURRENT_LIMIT_ERR_ENR {
        CURRENT_LIMIT_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline]
    pub fn data_end_bit_err_en(&self) -> DATA_END_BIT_ERR_ENR {
        DATA_END_BIT_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline]
    pub fn data_crc_err_en(&self) -> DATA_CRC_ERR_ENR {
        DATA_CRC_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline]
    pub fn data_timeout_err_en(&self) -> DATA_TIMEOUT_ERR_ENR {
        DATA_TIMEOUT_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline]
    pub fn cmd_ind_err_en(&self) -> CMD_IND_ERR_ENR {
        CMD_IND_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline]
    pub fn cmd_end_bit_err_en(&self) -> CMD_END_BIT_ERR_ENR {
        CMD_END_BIT_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline]
    pub fn cmd_crc_err_en(&self) -> CMD_CRC_ERR_ENR {
        CMD_CRC_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline]
    pub fn cmd_timeout_err_en(&self) -> CMD_TIMEOUT_ERR_ENR {
        CMD_TIMEOUT_ERR_ENR::_from({
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
    #[doc = "Bit 13 - Ceata Error Signal Enable"]
    #[inline]
    pub fn ceata_err_en(&mut self) -> _CEATA_ERR_ENW {
        _CEATA_ERR_ENW { w: self }
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable"]
    #[inline]
    pub fn target_resp_err_en(&mut self) -> _TARGET_RESP_ERR_ENW {
        _TARGET_RESP_ERR_ENW { w: self }
    }
    #[doc = "Bit 8 - Auto CMD12 Error Signal Enable"]
    #[inline]
    pub fn acmd_err_en(&mut self) -> _ACMD_ERR_ENW {
        _ACMD_ERR_ENW { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline]
    pub fn current_limit_err_en(&mut self) -> _CURRENT_LIMIT_ERR_ENW {
        _CURRENT_LIMIT_ERR_ENW { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline]
    pub fn data_end_bit_err_en(&mut self) -> _DATA_END_BIT_ERR_ENW {
        _DATA_END_BIT_ERR_ENW { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline]
    pub fn data_crc_err_en(&mut self) -> _DATA_CRC_ERR_ENW {
        _DATA_CRC_ERR_ENW { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline]
    pub fn data_timeout_err_en(&mut self) -> _DATA_TIMEOUT_ERR_ENW {
        _DATA_TIMEOUT_ERR_ENW { w: self }
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline]
    pub fn cmd_ind_err_en(&mut self) -> _CMD_IND_ERR_ENW {
        _CMD_IND_ERR_ENW { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline]
    pub fn cmd_end_bit_err_en(&mut self) -> _CMD_END_BIT_ERR_ENW {
        _CMD_END_BIT_ERR_ENW { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline]
    pub fn cmd_crc_err_en(&mut self) -> _CMD_CRC_ERR_ENW {
        _CMD_CRC_ERR_ENW { w: self }
    }
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline]
    pub fn cmd_timeout_err_en(&mut self) -> _CMD_TIMEOUT_ERR_ENW {
        _CMD_TIMEOUT_ERR_ENW { w: self }
    }
}
