#[doc = "Reader of register EN_INT_SIGNAL_NORM"]
pub type R = crate::R<u16, super::EN_INT_SIGNAL_NORM>;
#[doc = "Writer for register EN_INT_SIGNAL_NORM"]
pub type W = crate::W<u16, super::EN_INT_SIGNAL_NORM>;
#[doc = "Register EN_INT_SIGNAL_NORM `reset()`'s with value 0"]
impl crate::ResetValue for super::EN_INT_SIGNAL_NORM {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIXED_TO_0`"]
pub type FIXED_TO_0_R = crate::R<bool, bool>;
#[doc = "Card Interrupt Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CARD_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARD_INT_EN`"]
pub type CARD_INT_EN_R = crate::R<bool, CARD_INT_EN_A>;
impl CARD_INT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_INT_EN_A {
        match self.bits {
            false => CARD_INT_EN_A::VALUE1,
            true => CARD_INT_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INT_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INT_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CARD_INT_EN`"]
pub struct CARD_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_INT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_INT_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_INT_EN_A::VALUE2)
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
#[doc = "Card Removal Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_REMOVAL_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CARD_REMOVAL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_REMOVAL_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARD_REMOVAL_EN`"]
pub type CARD_REMOVAL_EN_R = crate::R<bool, CARD_REMOVAL_EN_A>;
impl CARD_REMOVAL_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_REMOVAL_EN_A {
        match self.bits {
            false => CARD_REMOVAL_EN_A::VALUE1,
            true => CARD_REMOVAL_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_REMOVAL_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_REMOVAL_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CARD_REMOVAL_EN`"]
pub struct CARD_REMOVAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_REMOVAL_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_REMOVAL_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_REMOVAL_EN_A::VALUE2)
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
#[doc = "Card Insertion Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INS_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CARD_INS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARD_INS_EN`"]
pub type CARD_INS_EN_R = crate::R<bool, CARD_INS_EN_A>;
impl CARD_INS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_INS_EN_A {
        match self.bits {
            false => CARD_INS_EN_A::VALUE1,
            true => CARD_INS_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INS_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INS_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CARD_INS_EN`"]
pub struct CARD_INS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_INS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_INS_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_INS_EN_A::VALUE2)
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
#[doc = "Buffer Read Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_READ_READY_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<BUFF_READ_READY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_READ_READY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUFF_READ_READY_EN`"]
pub type BUFF_READ_READY_EN_R = crate::R<bool, BUFF_READ_READY_EN_A>;
impl BUFF_READ_READY_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFF_READ_READY_EN_A {
        match self.bits {
            false => BUFF_READ_READY_EN_A::VALUE1,
            true => BUFF_READ_READY_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_READ_READY_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_READ_READY_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `BUFF_READ_READY_EN`"]
pub struct BUFF_READ_READY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_READ_READY_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_READ_READY_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_READ_READY_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_READ_READY_EN_A::VALUE2)
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
#[doc = "Buffer Write Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_WRITE_READY_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<BUFF_WRITE_READY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_WRITE_READY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUFF_WRITE_READY_EN`"]
pub type BUFF_WRITE_READY_EN_R = crate::R<bool, BUFF_WRITE_READY_EN_A>;
impl BUFF_WRITE_READY_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFF_WRITE_READY_EN_A {
        match self.bits {
            false => BUFF_WRITE_READY_EN_A::VALUE1,
            true => BUFF_WRITE_READY_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_WRITE_READY_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_WRITE_READY_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `BUFF_WRITE_READY_EN`"]
pub struct BUFF_WRITE_READY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_WRITE_READY_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_WRITE_READY_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READY_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READY_EN_A::VALUE2)
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
#[doc = "Block Gap Event Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_GAP_EVENT_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<BLOCK_GAP_EVENT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_GAP_EVENT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLOCK_GAP_EVENT_EN`"]
pub type BLOCK_GAP_EVENT_EN_R = crate::R<bool, BLOCK_GAP_EVENT_EN_A>;
impl BLOCK_GAP_EVENT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_GAP_EVENT_EN_A {
        match self.bits {
            false => BLOCK_GAP_EVENT_EN_A::VALUE1,
            true => BLOCK_GAP_EVENT_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_GAP_EVENT_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_GAP_EVENT_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `BLOCK_GAP_EVENT_EN`"]
pub struct BLOCK_GAP_EVENT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_GAP_EVENT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCK_GAP_EVENT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENT_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENT_EN_A::VALUE2)
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
#[doc = "Transfer Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_COMPLETE_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<TX_COMPLETE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_COMPLETE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_COMPLETE_EN`"]
pub type TX_COMPLETE_EN_R = crate::R<bool, TX_COMPLETE_EN_A>;
impl TX_COMPLETE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_COMPLETE_EN_A {
        match self.bits {
            false => TX_COMPLETE_EN_A::VALUE1,
            true => TX_COMPLETE_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_COMPLETE_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_COMPLETE_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `TX_COMPLETE_EN`"]
pub struct TX_COMPLETE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_COMPLETE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_COMPLETE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_COMPLETE_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_COMPLETE_EN_A::VALUE2)
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
#[doc = "Command Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_COMPLETE_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CMD_COMPLETE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_COMPLETE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD_COMPLETE_EN`"]
pub type CMD_COMPLETE_EN_R = crate::R<bool, CMD_COMPLETE_EN_A>;
impl CMD_COMPLETE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_COMPLETE_EN_A {
        match self.bits {
            false => CMD_COMPLETE_EN_A::VALUE1,
            true => CMD_COMPLETE_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMPLETE_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMPLETE_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_COMPLETE_EN`"]
pub struct CMD_COMPLETE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_COMPLETE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_COMPLETE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_COMPLETE_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_COMPLETE_EN_A::VALUE2)
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
    #[doc = "Bit 15 - Fixed to 0"]
    #[inline(always)]
    pub fn fixed_to_0(&self) -> FIXED_TO_0_R {
        FIXED_TO_0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn card_int_en(&self) -> CARD_INT_EN_R {
        CARD_INT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn card_removal_en(&self) -> CARD_REMOVAL_EN_R {
        CARD_REMOVAL_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn card_ins_en(&self) -> CARD_INS_EN_R {
        CARD_INS_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn buff_read_ready_en(&self) -> BUFF_READ_READY_EN_R {
        BUFF_READ_READY_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn buff_write_ready_en(&self) -> BUFF_WRITE_READY_EN_R {
        BUFF_WRITE_READY_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn block_gap_event_en(&self) -> BLOCK_GAP_EVENT_EN_R {
        BLOCK_GAP_EVENT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn tx_complete_en(&self) -> TX_COMPLETE_EN_R {
        TX_COMPLETE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmd_complete_en(&self) -> CMD_COMPLETE_EN_R {
        CMD_COMPLETE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn card_int_en(&mut self) -> CARD_INT_EN_W {
        CARD_INT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn card_removal_en(&mut self) -> CARD_REMOVAL_EN_W {
        CARD_REMOVAL_EN_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn card_ins_en(&mut self) -> CARD_INS_EN_W {
        CARD_INS_EN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn buff_read_ready_en(&mut self) -> BUFF_READ_READY_EN_W {
        BUFF_READ_READY_EN_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn buff_write_ready_en(&mut self) -> BUFF_WRITE_READY_EN_W {
        BUFF_WRITE_READY_EN_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn block_gap_event_en(&mut self) -> BLOCK_GAP_EVENT_EN_W {
        BLOCK_GAP_EVENT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn tx_complete_en(&mut self) -> TX_COMPLETE_EN_W {
        TX_COMPLETE_EN_W { w: self }
    }
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmd_complete_en(&mut self) -> CMD_COMPLETE_EN_W {
        CMD_COMPLETE_EN_W { w: self }
    }
}
