#[doc = "Reader of register INT_STATUS_ERR"]
pub type R = crate::R<u16, super::INT_STATUS_ERR>;
#[doc = "Writer for register INT_STATUS_ERR"]
pub type W = crate::W<u16, super::INT_STATUS_ERR>;
#[doc = "Register INT_STATUS_ERR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STATUS_ERR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Ceata Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEATA_ERR_A {
    #[doc = "0: no error"]
    VALUE1,
    #[doc = "1: error"]
    VALUE2,
}
impl From<CEATA_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CEATA_ERR_A) -> Self {
        match variant {
            CEATA_ERR_A::VALUE1 => false,
            CEATA_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CEATA_ERR`"]
pub type CEATA_ERR_R = crate::R<bool, CEATA_ERR_A>;
impl CEATA_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEATA_ERR_A {
        match self.bits {
            false => CEATA_ERR_A::VALUE1,
            true => CEATA_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEATA_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEATA_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEATA_ERR`"]
pub struct CEATA_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CEATA_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEATA_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEATA_ERR_A::VALUE1)
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEATA_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: Error"]
    VALUE2,
}
impl From<ACMD_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_ERR_A) -> Self {
        match variant {
            ACMD_ERR_A::VALUE1 => false,
            ACMD_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ACMD_ERR`"]
pub type ACMD_ERR_R = crate::R<bool, ACMD_ERR_A>;
impl ACMD_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD_ERR_A {
        match self.bits {
            false => ACMD_ERR_A::VALUE1,
            true => ACMD_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACMD_ERR`"]
pub struct ACMD_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMD_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMD_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACMD_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACMD_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRENT_LIMIT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: Power Fail"]
    VALUE2,
}
impl From<CURRENT_LIMIT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CURRENT_LIMIT_ERR_A) -> Self {
        match variant {
            CURRENT_LIMIT_ERR_A::VALUE1 => false,
            CURRENT_LIMIT_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CURRENT_LIMIT_ERR`"]
pub type CURRENT_LIMIT_ERR_R = crate::R<bool, CURRENT_LIMIT_ERR_A>;
impl CURRENT_LIMIT_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURRENT_LIMIT_ERR_A {
        match self.bits {
            false => CURRENT_LIMIT_ERR_A::VALUE1,
            true => CURRENT_LIMIT_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CURRENT_LIMIT_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CURRENT_LIMIT_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `CURRENT_LIMIT_ERR`"]
pub struct CURRENT_LIMIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_LIMIT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURRENT_LIMIT_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERR_A::VALUE1)
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_END_BIT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: Error"]
    VALUE2,
}
impl From<DATA_END_BIT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_END_BIT_ERR_A) -> Self {
        match variant {
            DATA_END_BIT_ERR_A::VALUE1 => false,
            DATA_END_BIT_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DATA_END_BIT_ERR`"]
pub type DATA_END_BIT_ERR_R = crate::R<bool, DATA_END_BIT_ERR_A>;
impl DATA_END_BIT_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_END_BIT_ERR_A {
        match self.bits {
            false => DATA_END_BIT_ERR_A::VALUE1,
            true => DATA_END_BIT_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_END_BIT_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_END_BIT_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `DATA_END_BIT_ERR`"]
pub struct DATA_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_END_BIT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_END_BIT_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_CRC_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: Error"]
    VALUE2,
}
impl From<DATA_CRC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_CRC_ERR_A) -> Self {
        match variant {
            DATA_CRC_ERR_A::VALUE1 => false,
            DATA_CRC_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DATA_CRC_ERR`"]
pub type DATA_CRC_ERR_R = crate::R<bool, DATA_CRC_ERR_A>;
impl DATA_CRC_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_CRC_ERR_A {
        match self.bits {
            false => DATA_CRC_ERR_A::VALUE1,
            true => DATA_CRC_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_CRC_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_CRC_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `DATA_CRC_ERR`"]
pub struct DATA_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CRC_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_CRC_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_CRC_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_CRC_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_TIMEOUT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: Timeout"]
    VALUE2,
}
impl From<DATA_TIMEOUT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_TIMEOUT_ERR_A) -> Self {
        match variant {
            DATA_TIMEOUT_ERR_A::VALUE1 => false,
            DATA_TIMEOUT_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DATA_TIMEOUT_ERR`"]
pub type DATA_TIMEOUT_ERR_R = crate::R<bool, DATA_TIMEOUT_ERR_A>;
impl DATA_TIMEOUT_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_TIMEOUT_ERR_A {
        match self.bits {
            false => DATA_TIMEOUT_ERR_A::VALUE1,
            true => DATA_TIMEOUT_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_TIMEOUT_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_TIMEOUT_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `DATA_TIMEOUT_ERR`"]
pub struct DATA_TIMEOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TIMEOUT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_TIMEOUT_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERR_A::VALUE1)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_IND_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: Error"]
    VALUE2,
}
impl From<CMD_IND_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_IND_ERR_A) -> Self {
        match variant {
            CMD_IND_ERR_A::VALUE1 => false,
            CMD_IND_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMD_IND_ERR`"]
pub type CMD_IND_ERR_R = crate::R<bool, CMD_IND_ERR_A>;
impl CMD_IND_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_IND_ERR_A {
        match self.bits {
            false => CMD_IND_ERR_A::VALUE1,
            true => CMD_IND_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_IND_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_IND_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_IND_ERR`"]
pub struct CMD_IND_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IND_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_IND_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_IND_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_IND_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_END_BIT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: End Bit Error Generated"]
    VALUE2,
}
impl From<CMD_END_BIT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_END_BIT_ERR_A) -> Self {
        match variant {
            CMD_END_BIT_ERR_A::VALUE1 => false,
            CMD_END_BIT_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMD_END_BIT_ERR`"]
pub type CMD_END_BIT_ERR_R = crate::R<bool, CMD_END_BIT_ERR_A>;
impl CMD_END_BIT_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_END_BIT_ERR_A {
        match self.bits {
            false => CMD_END_BIT_ERR_A::VALUE1,
            true => CMD_END_BIT_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_END_BIT_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_END_BIT_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_END_BIT_ERR`"]
pub struct CMD_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_END_BIT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_END_BIT_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERR_A::VALUE1)
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_CRC_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: CRC Error Generated"]
    VALUE2,
}
impl From<CMD_CRC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_CRC_ERR_A) -> Self {
        match variant {
            CMD_CRC_ERR_A::VALUE1 => false,
            CMD_CRC_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMD_CRC_ERR`"]
pub type CMD_CRC_ERR_R = crate::R<bool, CMD_CRC_ERR_A>;
impl CMD_CRC_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_CRC_ERR_A {
        match self.bits {
            false => CMD_CRC_ERR_A::VALUE1,
            true => CMD_CRC_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_CRC_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_CRC_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_CRC_ERR`"]
pub struct CMD_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CRC_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_CRC_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_CRC_ERR_A::VALUE1)
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_CRC_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_TIMEOUT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1,
    #[doc = "1: Timeout"]
    VALUE2,
}
impl From<CMD_TIMEOUT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_TIMEOUT_ERR_A) -> Self {
        match variant {
            CMD_TIMEOUT_ERR_A::VALUE1 => false,
            CMD_TIMEOUT_ERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMD_TIMEOUT_ERR`"]
pub type CMD_TIMEOUT_ERR_R = crate::R<bool, CMD_TIMEOUT_ERR_A>;
impl CMD_TIMEOUT_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_TIMEOUT_ERR_A {
        match self.bits {
            false => CMD_TIMEOUT_ERR_A::VALUE1,
            true => CMD_TIMEOUT_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_TIMEOUT_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_TIMEOUT_ERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_TIMEOUT_ERR`"]
pub struct CMD_TIMEOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TIMEOUT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_TIMEOUT_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERR_A::VALUE1)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline(always)]
    pub fn ceata_err(&self) -> CEATA_ERR_R {
        CEATA_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    pub fn acmd_err(&self) -> ACMD_ERR_R {
        ACMD_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    pub fn current_limit_err(&self) -> CURRENT_LIMIT_ERR_R {
        CURRENT_LIMIT_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&self) -> DATA_END_BIT_ERR_R {
        DATA_END_BIT_ERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    pub fn data_crc_err(&self) -> DATA_CRC_ERR_R {
        DATA_CRC_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    pub fn data_timeout_err(&self) -> DATA_TIMEOUT_ERR_R {
        DATA_TIMEOUT_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    pub fn cmd_ind_err(&self) -> CMD_IND_ERR_R {
        CMD_IND_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CMD_END_BIT_ERR_R {
        CMD_END_BIT_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CMD_CRC_ERR_R {
        CMD_CRC_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmd_timeout_err(&self) -> CMD_TIMEOUT_ERR_R {
        CMD_TIMEOUT_ERR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline(always)]
    pub fn ceata_err(&mut self) -> CEATA_ERR_W {
        CEATA_ERR_W { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    pub fn acmd_err(&mut self) -> ACMD_ERR_W {
        ACMD_ERR_W { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    pub fn current_limit_err(&mut self) -> CURRENT_LIMIT_ERR_W {
        CURRENT_LIMIT_ERR_W { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&mut self) -> DATA_END_BIT_ERR_W {
        DATA_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    pub fn data_crc_err(&mut self) -> DATA_CRC_ERR_W {
        DATA_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    pub fn data_timeout_err(&mut self) -> DATA_TIMEOUT_ERR_W {
        DATA_TIMEOUT_ERR_W { w: self }
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    pub fn cmd_ind_err(&mut self) -> CMD_IND_ERR_W {
        CMD_IND_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&mut self) -> CMD_END_BIT_ERR_W {
        CMD_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    pub fn cmd_crc_err(&mut self) -> CMD_CRC_ERR_W {
        CMD_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmd_timeout_err(&mut self) -> CMD_TIMEOUT_ERR_W {
        CMD_TIMEOUT_ERR_W { w: self }
    }
}
