#[doc = "Reader of register COMMAND"]
pub type R = crate::R<u16, super::COMMAND>;
#[doc = "Writer for register COMMAND"]
pub type W = crate::W<u16, super::COMMAND>;
#[doc = "Register COMMAND `reset()`'s with value 0"]
impl crate::ResetValue for super::COMMAND {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_IND`"]
pub type CMD_IND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD_IND`"]
pub struct CMD_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_TYPE_A {
    #[doc = "0: Normal"]
    VALUE1 = 0,
    #[doc = "1: Suspend"]
    VALUE2 = 1,
    #[doc = "2: Resume"]
    VALUE3 = 2,
    #[doc = "3: Abort"]
    VALUE4 = 3,
}
impl From<CMD_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD_TYPE`"]
pub type CMD_TYPE_R = crate::R<u8, CMD_TYPE_A>;
impl CMD_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_TYPE_A {
        match self.bits {
            0 => CMD_TYPE_A::VALUE1,
            1 => CMD_TYPE_A::VALUE2,
            2 => CMD_TYPE_A::VALUE3,
            3 => CMD_TYPE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_TYPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_TYPE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMD_TYPE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMD_TYPE_A::VALUE4
    }
}
#[doc = "Write proxy for field `CMD_TYPE`"]
pub struct CMD_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_TYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE1)
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE2)
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE3)
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_PRESENT_SELECT_A {
    #[doc = "0: No Data Present"]
    VALUE1 = 0,
    #[doc = "1: Data Present"]
    VALUE2 = 1,
}
impl From<DATA_PRESENT_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_PRESENT_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_PRESENT_SELECT`"]
pub type DATA_PRESENT_SELECT_R = crate::R<bool, DATA_PRESENT_SELECT_A>;
impl DATA_PRESENT_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_PRESENT_SELECT_A {
        match self.bits {
            false => DATA_PRESENT_SELECT_A::VALUE1,
            true => DATA_PRESENT_SELECT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_PRESENT_SELECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_PRESENT_SELECT_A::VALUE2
    }
}
#[doc = "Write proxy for field `DATA_PRESENT_SELECT`"]
pub struct DATA_PRESENT_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PRESENT_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_PRESENT_SELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_PRESENT_SELECT_A::VALUE1)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_PRESENT_SELECT_A::VALUE2)
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
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_IND_CHECK_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CMD_IND_CHECK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_IND_CHECK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD_IND_CHECK_EN`"]
pub type CMD_IND_CHECK_EN_R = crate::R<bool, CMD_IND_CHECK_EN_A>;
impl CMD_IND_CHECK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_IND_CHECK_EN_A {
        match self.bits {
            false => CMD_IND_CHECK_EN_A::VALUE1,
            true => CMD_IND_CHECK_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_IND_CHECK_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_IND_CHECK_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_IND_CHECK_EN`"]
pub struct CMD_IND_CHECK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IND_CHECK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_IND_CHECK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_IND_CHECK_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_IND_CHECK_EN_A::VALUE2)
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
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_CRC_CHECK_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CMD_CRC_CHECK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_CRC_CHECK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD_CRC_CHECK_EN`"]
pub type CMD_CRC_CHECK_EN_R = crate::R<bool, CMD_CRC_CHECK_EN_A>;
impl CMD_CRC_CHECK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_CRC_CHECK_EN_A {
        match self.bits {
            false => CMD_CRC_CHECK_EN_A::VALUE1,
            true => CMD_CRC_CHECK_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_CRC_CHECK_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_CRC_CHECK_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_CRC_CHECK_EN`"]
pub struct CMD_CRC_CHECK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CRC_CHECK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_CRC_CHECK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_CRC_CHECK_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_CRC_CHECK_EN_A::VALUE2)
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
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESP_TYPE_SELECT_A {
    #[doc = "0: No Response"]
    VALUE1 = 0,
    #[doc = "1: Response length 136"]
    VALUE2 = 1,
    #[doc = "2: Response length 48"]
    VALUE3 = 2,
    #[doc = "3: Response length 48 check Busy after response"]
    VALUE4 = 3,
}
impl From<RESP_TYPE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RESP_TYPE_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESP_TYPE_SELECT`"]
pub type RESP_TYPE_SELECT_R = crate::R<u8, RESP_TYPE_SELECT_A>;
impl RESP_TYPE_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESP_TYPE_SELECT_A {
        match self.bits {
            0 => RESP_TYPE_SELECT_A::VALUE1,
            1 => RESP_TYPE_SELECT_A::VALUE2,
            2 => RESP_TYPE_SELECT_A::VALUE3,
            3 => RESP_TYPE_SELECT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE4
    }
}
#[doc = "Write proxy for field `RESP_TYPE_SELECT`"]
pub struct RESP_TYPE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_TYPE_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESP_TYPE_SELECT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Response"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE1)
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE2)
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE3)
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmd_ind(&self) -> CMD_IND_R {
        CMD_IND_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn data_present_select(&self) -> DATA_PRESENT_SELECT_R {
        DATA_PRESENT_SELECT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmd_ind_check_en(&self) -> CMD_IND_CHECK_EN_R {
        CMD_IND_CHECK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmd_crc_check_en(&self) -> CMD_CRC_CHECK_EN_R {
        CMD_CRC_CHECK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    pub fn resp_type_select(&self) -> RESP_TYPE_SELECT_R {
        RESP_TYPE_SELECT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmd_ind(&mut self) -> CMD_IND_W {
        CMD_IND_W { w: self }
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W {
        CMD_TYPE_W { w: self }
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn data_present_select(&mut self) -> DATA_PRESENT_SELECT_W {
        DATA_PRESENT_SELECT_W { w: self }
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmd_ind_check_en(&mut self) -> CMD_IND_CHECK_EN_W {
        CMD_IND_CHECK_EN_W { w: self }
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmd_crc_check_en(&mut self) -> CMD_CRC_CHECK_EN_W {
        CMD_CRC_CHECK_EN_W { w: self }
    }
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    pub fn resp_type_select(&mut self) -> RESP_TYPE_SELECT_W {
        RESP_TYPE_SELECT_W { w: self }
    }
}
