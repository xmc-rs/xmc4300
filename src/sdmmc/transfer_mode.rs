#[doc = "Reader of register TRANSFER_MODE"]
pub type R = crate::R<u16, super::TRANSFER_MODE>;
#[doc = "Writer for register TRANSFER_MODE"]
pub type W = crate::W<u16, super::TRANSFER_MODE>;
#[doc = "Register TRANSFER_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::TRANSFER_MODE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Command Completion Signal Enable for CE-ATA Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_COMP_ATA_A {
    #[doc = "1: Device will send command completion Signal"]
    VALUE1 = 1,
    #[doc = "0: Device will not send command completion Signal"]
    VALUE2 = 0,
}
impl From<CMD_COMP_ATA_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_COMP_ATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD_COMP_ATA`"]
pub type CMD_COMP_ATA_R = crate::R<bool, CMD_COMP_ATA_A>;
impl CMD_COMP_ATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_COMP_ATA_A {
        match self.bits {
            true => CMD_COMP_ATA_A::VALUE1,
            false => CMD_COMP_ATA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMP_ATA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMP_ATA_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_COMP_ATA`"]
pub struct CMD_COMP_ATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_COMP_ATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_COMP_ATA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Device will send command completion Signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_COMP_ATA_A::VALUE1)
    }
    #[doc = "Device will not send command completion Signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_COMP_ATA_A::VALUE2)
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
#[doc = "Multi / Single Block Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTI_BLOCK_SELECT_A {
    #[doc = "0: Single Block"]
    VALUE1 = 0,
    #[doc = "1: Multiple Block"]
    VALUE2 = 1,
}
impl From<MULTI_BLOCK_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MULTI_BLOCK_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MULTI_BLOCK_SELECT`"]
pub type MULTI_BLOCK_SELECT_R = crate::R<bool, MULTI_BLOCK_SELECT_A>;
impl MULTI_BLOCK_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULTI_BLOCK_SELECT_A {
        match self.bits {
            false => MULTI_BLOCK_SELECT_A::VALUE1,
            true => MULTI_BLOCK_SELECT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MULTI_BLOCK_SELECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MULTI_BLOCK_SELECT_A::VALUE2
    }
}
#[doc = "Write proxy for field `MULTI_BLOCK_SELECT`"]
pub struct MULTI_BLOCK_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTI_BLOCK_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULTI_BLOCK_SELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MULTI_BLOCK_SELECT_A::VALUE1)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MULTI_BLOCK_SELECT_A::VALUE2)
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
#[doc = "Data Transfer Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DIR_SELECT_A {
    #[doc = "0: Write (Host to Card)"]
    VALUE1 = 0,
    #[doc = "1: Read (Card to Host)"]
    VALUE2 = 1,
}
impl From<TX_DIR_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DIR_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_DIR_SELECT`"]
pub type TX_DIR_SELECT_R = crate::R<bool, TX_DIR_SELECT_A>;
impl TX_DIR_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DIR_SELECT_A {
        match self.bits {
            false => TX_DIR_SELECT_A::VALUE1,
            true => TX_DIR_SELECT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_DIR_SELECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_DIR_SELECT_A::VALUE2
    }
}
#[doc = "Write proxy for field `TX_DIR_SELECT`"]
pub struct TX_DIR_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DIR_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DIR_SELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_DIR_SELECT_A::VALUE1)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_DIR_SELECT_A::VALUE2)
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
#[doc = "Auto CMD Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACMD_EN_A {
    #[doc = "0: Auto Command Disabled"]
    VALUE1 = 0,
    #[doc = "1: Auto CMD12 Enable"]
    VALUE2 = 1,
}
impl From<ACMD_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMD_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACMD_EN`"]
pub type ACMD_EN_R = crate::R<u8, ACMD_EN_A>;
impl ACMD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACMD_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACMD_EN_A::VALUE1),
            1 => Val(ACMD_EN_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ACMD_EN`"]
pub struct ACMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMD_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACMD_EN_A::VALUE1)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACMD_EN_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_COUNT_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<BLOCK_COUNT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_COUNT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLOCK_COUNT_EN`"]
pub type BLOCK_COUNT_EN_R = crate::R<bool, BLOCK_COUNT_EN_A>;
impl BLOCK_COUNT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_COUNT_EN_A {
        match self.bits {
            false => BLOCK_COUNT_EN_A::VALUE1,
            true => BLOCK_COUNT_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_COUNT_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_COUNT_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `BLOCK_COUNT_EN`"]
pub struct BLOCK_COUNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_COUNT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCK_COUNT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BLOCK_COUNT_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BLOCK_COUNT_EN_A::VALUE2)
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
impl R {
    #[doc = "Bit 6 - Command Completion Signal Enable for CE-ATA Device"]
    #[inline(always)]
    pub fn cmd_comp_ata(&self) -> CMD_COMP_ATA_R {
        CMD_COMP_ATA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    pub fn multi_block_select(&self) -> MULTI_BLOCK_SELECT_R {
        MULTI_BLOCK_SELECT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn tx_dir_select(&self) -> TX_DIR_SELECT_R {
        TX_DIR_SELECT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Auto CMD Enable"]
    #[inline(always)]
    pub fn acmd_en(&self) -> ACMD_EN_R {
        ACMD_EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn block_count_en(&self) -> BLOCK_COUNT_EN_R {
        BLOCK_COUNT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Command Completion Signal Enable for CE-ATA Device"]
    #[inline(always)]
    pub fn cmd_comp_ata(&mut self) -> CMD_COMP_ATA_W {
        CMD_COMP_ATA_W { w: self }
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    pub fn multi_block_select(&mut self) -> MULTI_BLOCK_SELECT_W {
        MULTI_BLOCK_SELECT_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn tx_dir_select(&mut self) -> TX_DIR_SELECT_W {
        TX_DIR_SELECT_W { w: self }
    }
    #[doc = "Bits 2:3 - Auto CMD Enable"]
    #[inline(always)]
    pub fn acmd_en(&mut self) -> ACMD_EN_W {
        ACMD_EN_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn block_count_en(&mut self) -> BLOCK_COUNT_EN_W {
        BLOCK_COUNT_EN_W { w: self }
    }
}
