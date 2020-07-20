#[doc = "Writer for register FORCE_EVENT_ACMD_ERR_STATUS"]
pub type W = crate::W<u16, super::FORCE_EVENT_ACMD_ERR_STATUS>;
#[doc = "Register FORCE_EVENT_ACMD_ERR_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCE_EVENT_ACMD_ERR_STATUS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force Event for CMD not issued by Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_CMD_NOT_ISSUED_ACMD12_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CMD_NOT_ISSUED_ACMD12_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_NOT_ISSUED_ACMD12_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FE_CMD_NOT_ISSUED_ACMD12_ERR`"]
pub struct FE_CMD_NOT_ISSUED_ACMD12_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_CMD_NOT_ISSUED_ACMD12_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_CMD_NOT_ISSUED_ACMD12_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_NOT_ISSUED_ACMD12_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_NOT_ISSUED_ACMD12_ERR_AW::VALUE2)
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
#[doc = "Force Event for Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_ACMD_IND_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_IND_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_IND_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FE_ACMD_IND_ERR`"]
pub struct FE_ACMD_IND_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_ACMD_IND_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_ACMD_IND_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_IND_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_IND_ERR_AW::VALUE2)
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
#[doc = "Force Event for Auto CMD End bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_ACMD_END_BIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_END_BIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_END_BIT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FE_ACMD_END_BIT_ERR`"]
pub struct FE_ACMD_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_ACMD_END_BIT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_ACMD_END_BIT_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_END_BIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_END_BIT_ERR_AW::VALUE2)
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
#[doc = "Force Event for Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_ACMD_CRC_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_CRC_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_CRC_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FE_ACMD_CRC_ERR`"]
pub struct FE_ACMD_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_ACMD_CRC_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_ACMD_CRC_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_CRC_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_CRC_ERR_AW::VALUE2)
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
#[doc = "Force Event for Auto CMD timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_ACMD_TIMEOUT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_TIMEOUT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_TIMEOUT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FE_ACMD_TIMEOUT_ERR`"]
pub struct FE_ACMD_TIMEOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_ACMD_TIMEOUT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_ACMD_TIMEOUT_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_TIMEOUT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_TIMEOUT_ERR_AW::VALUE2)
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
#[doc = "Force Event for Auto CMD12 NOT Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_ACMD_NOT_EXEC_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_NOT_EXEC_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_NOT_EXEC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FE_ACMD_NOT_EXEC`"]
pub struct FE_ACMD_NOT_EXEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_ACMD_NOT_EXEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_ACMD_NOT_EXEC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_NOT_EXEC_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_NOT_EXEC_AW::VALUE2)
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
impl W {
    #[doc = "Bit 7 - Force Event for CMD not issued by Auto CMD12 Error"]
    #[inline(always)]
    pub fn fe_cmd_not_issued_acmd12_err(&mut self) -> FE_CMD_NOT_ISSUED_ACMD12_ERR_W {
        FE_CMD_NOT_ISSUED_ACMD12_ERR_W { w: self }
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn fe_acmd_ind_err(&mut self) -> FE_ACMD_IND_ERR_W {
        FE_ACMD_IND_ERR_W { w: self }
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End bit Error"]
    #[inline(always)]
    pub fn fe_acmd_end_bit_err(&mut self) -> FE_ACMD_END_BIT_ERR_W {
        FE_ACMD_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn fe_acmd_crc_err(&mut self) -> FE_ACMD_CRC_ERR_W {
        FE_ACMD_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Force Event for Auto CMD timeout Error"]
    #[inline(always)]
    pub fn fe_acmd_timeout_err(&mut self) -> FE_ACMD_TIMEOUT_ERR_W {
        FE_ACMD_TIMEOUT_ERR_W { w: self }
    }
    #[doc = "Bit 0 - Force Event for Auto CMD12 NOT Executed"]
    #[inline(always)]
    pub fn fe_acmd_not_exec(&mut self) -> FE_ACMD_NOT_EXEC_W {
        FE_ACMD_NOT_EXEC_W { w: self }
    }
}
