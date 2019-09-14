#[doc = "Writer for register FORCE_EVENT_ERR_STATUS"]
pub type W = crate::W<u16, super::FORCE_EVENT_ERR_STATUS>;
#[doc = "Register FORCE_EVENT_ERR_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCE_EVENT_ERR_STATUS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force Event for Ceata Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_CEATA_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_CEATA_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CEATA_ERR_AW) -> Self {
        match variant {
            FE_CEATA_ERR_AW::VALUE1 => false,
            FE_CEATA_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_CEATA_ERR`"]
pub struct FE_CEATA_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_CEATA_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_CEATA_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CEATA_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CEATA_ERR_AW::VALUE2)
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
#[doc = "Force event for Target Response Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_TARGET_RESPONSE_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_TARGET_RESPONSE_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_TARGET_RESPONSE_ERR_AW) -> Self {
        match variant {
            FE_TARGET_RESPONSE_ERR_AW::VALUE1 => false,
            FE_TARGET_RESPONSE_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_TARGET_RESPONSE_ERR`"]
pub struct FE_TARGET_RESPONSE_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_TARGET_RESPONSE_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_TARGET_RESPONSE_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_TARGET_RESPONSE_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_TARGET_RESPONSE_ERR_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_ACMD12_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_ACMD12_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD12_ERR_AW) -> Self {
        match variant {
            FE_ACMD12_ERR_AW::VALUE1 => false,
            FE_ACMD12_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_ACMD12_ERR`"]
pub struct FE_ACMD12_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_ACMD12_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_ACMD12_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD12_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD12_ERR_AW::VALUE2)
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
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_CURRENT_LIMIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_CURRENT_LIMIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CURRENT_LIMIT_ERR_AW) -> Self {
        match variant {
            FE_CURRENT_LIMIT_ERR_AW::VALUE1 => false,
            FE_CURRENT_LIMIT_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_CURRENT_LIMIT_ERR`"]
pub struct FE_CURRENT_LIMIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_CURRENT_LIMIT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_CURRENT_LIMIT_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CURRENT_LIMIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CURRENT_LIMIT_ERR_AW::VALUE2)
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
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_DATA_END_BIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_DATA_END_BIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_DATA_END_BIT_ERR_AW) -> Self {
        match variant {
            FE_DATA_END_BIT_ERR_AW::VALUE1 => false,
            FE_DATA_END_BIT_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_DATA_END_BIT_ERR`"]
pub struct FE_DATA_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_DATA_END_BIT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_DATA_END_BIT_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_END_BIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_END_BIT_ERR_AW::VALUE2)
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
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_DATA_CRC_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_DATA_CRC_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_DATA_CRC_ERR_AW) -> Self {
        match variant {
            FE_DATA_CRC_ERR_AW::VALUE1 => false,
            FE_DATA_CRC_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_DATA_CRC_ERR`"]
pub struct FE_DATA_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_DATA_CRC_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_DATA_CRC_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_CRC_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_CRC_ERR_AW::VALUE2)
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
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_DATA_TIMEOUT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_DATA_TIMEOUT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_DATA_TIMEOUT_ERR_AW) -> Self {
        match variant {
            FE_DATA_TIMEOUT_ERR_AW::VALUE1 => false,
            FE_DATA_TIMEOUT_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_DATA_TIMEOUT_ERR`"]
pub struct FE_DATA_TIMEOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_DATA_TIMEOUT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_DATA_TIMEOUT_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_TIMEOUT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_TIMEOUT_ERR_AW::VALUE2)
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
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_CMD_IND_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_CMD_IND_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_IND_ERR_AW) -> Self {
        match variant {
            FE_CMD_IND_ERR_AW::VALUE1 => false,
            FE_CMD_IND_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_CMD_IND_ERR`"]
pub struct FE_CMD_IND_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_CMD_IND_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_CMD_IND_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_IND_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_IND_ERR_AW::VALUE2)
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
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_CMD_END_BIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_CMD_END_BIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_END_BIT_ERR_AW) -> Self {
        match variant {
            FE_CMD_END_BIT_ERR_AW::VALUE1 => false,
            FE_CMD_END_BIT_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_CMD_END_BIT_ERR`"]
pub struct FE_CMD_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_CMD_END_BIT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_CMD_END_BIT_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_END_BIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_END_BIT_ERR_AW::VALUE2)
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
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_CMD_CRC_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_CMD_CRC_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_CRC_ERR_AW) -> Self {
        match variant {
            FE_CMD_CRC_ERR_AW::VALUE1 => false,
            FE_CMD_CRC_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_CMD_CRC_ERR`"]
pub struct FE_CMD_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_CMD_CRC_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_CMD_CRC_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_CRC_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_CRC_ERR_AW::VALUE2)
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
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_CMD_TIMEOUT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1,
    #[doc = "1: Interrupt is generated"]
    VALUE2,
}
impl From<FE_CMD_TIMEOUT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_TIMEOUT_ERR_AW) -> Self {
        match variant {
            FE_CMD_TIMEOUT_ERR_AW::VALUE1 => false,
            FE_CMD_TIMEOUT_ERR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FE_CMD_TIMEOUT_ERR`"]
pub struct FE_CMD_TIMEOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_CMD_TIMEOUT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_CMD_TIMEOUT_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_TIMEOUT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_TIMEOUT_ERR_AW::VALUE2)
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
    #[doc = "Bit 13 - Force Event for Ceata Error"]
    #[inline(always)]
    pub fn fe_ceata_err(&mut self) -> FE_CEATA_ERR_W {
        FE_CEATA_ERR_W { w: self }
    }
    #[doc = "Bit 12 - Force event for Target Response Error"]
    #[inline(always)]
    pub fn fe_target_response_err(&mut self) -> FE_TARGET_RESPONSE_ERR_W {
        FE_TARGET_RESPONSE_ERR_W { w: self }
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn fe_acmd12_err(&mut self) -> FE_ACMD12_ERR_W {
        FE_ACMD12_ERR_W { w: self }
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn fe_current_limit_err(&mut self) -> FE_CURRENT_LIMIT_ERR_W {
        FE_CURRENT_LIMIT_ERR_W { w: self }
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn fe_data_end_bit_err(&mut self) -> FE_DATA_END_BIT_ERR_W {
        FE_DATA_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn fe_data_crc_err(&mut self) -> FE_DATA_CRC_ERR_W {
        FE_DATA_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn fe_data_timeout_err(&mut self) -> FE_DATA_TIMEOUT_ERR_W {
        FE_DATA_TIMEOUT_ERR_W { w: self }
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn fe_cmd_ind_err(&mut self) -> FE_CMD_IND_ERR_W {
        FE_CMD_IND_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn fe_cmd_end_bit_err(&mut self) -> FE_CMD_END_BIT_ERR_W {
        FE_CMD_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn fe_cmd_crc_err(&mut self) -> FE_CMD_CRC_ERR_W {
        FE_CMD_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn fe_cmd_timeout_err(&mut self) -> FE_CMD_TIMEOUT_ERR_W {
        FE_CMD_TIMEOUT_ERR_W { w: self }
    }
}
