#[doc = "Reader of register INT_STATUS_NORM"]
pub type R = crate::R<u16, super::INT_STATUS_NORM>;
#[doc = "Writer for register INT_STATUS_NORM"]
pub type W = crate::W<u16, super::INT_STATUS_NORM>;
#[doc = "Register INT_STATUS_NORM `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STATUS_NORM {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_INT_A {
    #[doc = "0: No Error."]
    VALUE1,
    #[doc = "1: Error."]
    VALUE2,
}
impl From<ERR_INT_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_INT_A) -> Self {
        match variant {
            ERR_INT_A::VALUE1 => false,
            ERR_INT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ERR_INT`"]
pub type ERR_INT_R = crate::R<bool, ERR_INT_A>;
impl ERR_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_INT_A {
        match self.bits {
            false => ERR_INT_A::VALUE1,
            true => ERR_INT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERR_INT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERR_INT_A::VALUE2
    }
}
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_A {
    #[doc = "0: No Card Interrupt"]
    VALUE1,
    #[doc = "1: Generate Card Interrupt"]
    VALUE2,
}
impl From<CARD_INT_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INT_A) -> Self {
        match variant {
            CARD_INT_A::VALUE1 => false,
            CARD_INT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CARD_INT`"]
pub type CARD_INT_R = crate::R<bool, CARD_INT_A>;
impl CARD_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_INT_A {
        match self.bits {
            false => CARD_INT_A::VALUE1,
            true => CARD_INT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INT_A::VALUE2
    }
}
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_REMOVAL_A {
    #[doc = "0: Card State Stable or Debouncing"]
    VALUE1,
    #[doc = "1: Card Removed"]
    VALUE2,
}
impl From<CARD_REMOVAL_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_REMOVAL_A) -> Self {
        match variant {
            CARD_REMOVAL_A::VALUE1 => false,
            CARD_REMOVAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CARD_REMOVAL`"]
pub type CARD_REMOVAL_R = crate::R<bool, CARD_REMOVAL_A>;
impl CARD_REMOVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_REMOVAL_A {
        match self.bits {
            false => CARD_REMOVAL_A::VALUE1,
            true => CARD_REMOVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_REMOVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_REMOVAL_A::VALUE2
    }
}
#[doc = "Write proxy for field `CARD_REMOVAL`"]
pub struct CARD_REMOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_REMOVAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_REMOVAL_A::VALUE1)
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_REMOVAL_A::VALUE2)
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
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INS_A {
    #[doc = "0: Card State Stable or Debouncing"]
    VALUE1,
    #[doc = "1: Card Inserted"]
    VALUE2,
}
impl From<CARD_INS_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INS_A) -> Self {
        match variant {
            CARD_INS_A::VALUE1 => false,
            CARD_INS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CARD_INS`"]
pub type CARD_INS_R = crate::R<bool, CARD_INS_A>;
impl CARD_INS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_INS_A {
        match self.bits {
            false => CARD_INS_A::VALUE1,
            true => CARD_INS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_INS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_INS_A::VALUE2
    }
}
#[doc = "Write proxy for field `CARD_INS`"]
pub struct CARD_INS_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_INS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_INS_A::VALUE1)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_INS_A::VALUE2)
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
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_READ_READY_A {
    #[doc = "0: Not Ready to read Buffer."]
    VALUE1,
    #[doc = "1: Ready to read Buffer."]
    VALUE2,
}
impl From<BUFF_READ_READY_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_READ_READY_A) -> Self {
        match variant {
            BUFF_READ_READY_A::VALUE1 => false,
            BUFF_READ_READY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BUFF_READ_READY`"]
pub type BUFF_READ_READY_R = crate::R<bool, BUFF_READ_READY_A>;
impl BUFF_READ_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFF_READ_READY_A {
        match self.bits {
            false => BUFF_READ_READY_A::VALUE1,
            true => BUFF_READ_READY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_READ_READY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_READ_READY_A::VALUE2
    }
}
#[doc = "Write proxy for field `BUFF_READ_READY`"]
pub struct BUFF_READ_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_READ_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_READ_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready to read Buffer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_READ_READY_A::VALUE1)
    }
    #[doc = "Ready to read Buffer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_READ_READY_A::VALUE2)
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
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_WRITE_READY_A {
    #[doc = "0: Not Ready to Write Buffer."]
    VALUE1,
    #[doc = "1: Ready to Write Buffer."]
    VALUE2,
}
impl From<BUFF_WRITE_READY_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_WRITE_READY_A) -> Self {
        match variant {
            BUFF_WRITE_READY_A::VALUE1 => false,
            BUFF_WRITE_READY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BUFF_WRITE_READY`"]
pub type BUFF_WRITE_READY_R = crate::R<bool, BUFF_WRITE_READY_A>;
impl BUFF_WRITE_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFF_WRITE_READY_A {
        match self.bits {
            false => BUFF_WRITE_READY_A::VALUE1,
            true => BUFF_WRITE_READY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUFF_WRITE_READY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUFF_WRITE_READY_A::VALUE2
    }
}
#[doc = "Write proxy for field `BUFF_WRITE_READY`"]
pub struct BUFF_WRITE_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_WRITE_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_WRITE_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready to Write Buffer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READY_A::VALUE1)
    }
    #[doc = "Ready to Write Buffer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BUFF_WRITE_READY_A::VALUE2)
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
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_GAP_EVENT_A {
    #[doc = "0: No Block Gap Event"]
    VALUE1,
    #[doc = "1: Transaction stopped at Block Gap"]
    VALUE2,
}
impl From<BLOCK_GAP_EVENT_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_GAP_EVENT_A) -> Self {
        match variant {
            BLOCK_GAP_EVENT_A::VALUE1 => false,
            BLOCK_GAP_EVENT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BLOCK_GAP_EVENT`"]
pub type BLOCK_GAP_EVENT_R = crate::R<bool, BLOCK_GAP_EVENT_A>;
impl BLOCK_GAP_EVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_GAP_EVENT_A {
        match self.bits {
            false => BLOCK_GAP_EVENT_A::VALUE1,
            true => BLOCK_GAP_EVENT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_GAP_EVENT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_GAP_EVENT_A::VALUE2
    }
}
#[doc = "Write proxy for field `BLOCK_GAP_EVENT`"]
pub struct BLOCK_GAP_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_GAP_EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCK_GAP_EVENT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENT_A::VALUE1)
    }
    #[doc = "Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BLOCK_GAP_EVENT_A::VALUE2)
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
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_COMPLETE_A {
    #[doc = "0: No Data Transfer Complete"]
    VALUE1,
    #[doc = "1: Data Transfer Complete"]
    VALUE2,
}
impl From<TX_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_COMPLETE_A) -> Self {
        match variant {
            TX_COMPLETE_A::VALUE1 => false,
            TX_COMPLETE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TX_COMPLETE`"]
pub type TX_COMPLETE_R = crate::R<bool, TX_COMPLETE_A>;
impl TX_COMPLETE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_COMPLETE_A {
        match self.bits {
            false => TX_COMPLETE_A::VALUE1,
            true => TX_COMPLETE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_COMPLETE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_COMPLETE_A::VALUE2
    }
}
#[doc = "Write proxy for field `TX_COMPLETE`"]
pub struct TX_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_COMPLETE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_COMPLETE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_COMPLETE_A::VALUE1)
    }
    #[doc = "Data Transfer Complete"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_COMPLETE_A::VALUE2)
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
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_COMPLETE_A {
    #[doc = "0: No Command Complete"]
    VALUE1,
    #[doc = "1: Command Complete"]
    VALUE2,
}
impl From<CMD_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_COMPLETE_A) -> Self {
        match variant {
            CMD_COMPLETE_A::VALUE1 => false,
            CMD_COMPLETE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CMD_COMPLETE`"]
pub type CMD_COMPLETE_R = crate::R<bool, CMD_COMPLETE_A>;
impl CMD_COMPLETE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_COMPLETE_A {
        match self.bits {
            false => CMD_COMPLETE_A::VALUE1,
            true => CMD_COMPLETE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMPLETE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMPLETE_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMD_COMPLETE`"]
pub struct CMD_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_COMPLETE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_COMPLETE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_COMPLETE_A::VALUE1)
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_COMPLETE_A::VALUE2)
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
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn card_int(&self) -> CARD_INT_R {
        CARD_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn card_ins(&self) -> CARD_INS_R {
        CARD_INS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn buff_read_ready(&self) -> BUFF_READ_READY_R {
        BUFF_READ_READY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn buff_write_ready(&self) -> BUFF_WRITE_READY_R {
        BUFF_WRITE_READY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn block_gap_event(&self) -> BLOCK_GAP_EVENT_R {
        BLOCK_GAP_EVENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tx_complete(&self) -> TX_COMPLETE_R {
        TX_COMPLETE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CMD_COMPLETE_R {
        CMD_COMPLETE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W {
        CARD_REMOVAL_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn card_ins(&mut self) -> CARD_INS_W {
        CARD_INS_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn buff_read_ready(&mut self) -> BUFF_READ_READY_W {
        BUFF_READ_READY_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn buff_write_ready(&mut self) -> BUFF_WRITE_READY_W {
        BUFF_WRITE_READY_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn block_gap_event(&mut self) -> BLOCK_GAP_EVENT_W {
        BLOCK_GAP_EVENT_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tx_complete(&mut self) -> TX_COMPLETE_W {
        TX_COMPLETE_W { w: self }
    }
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&mut self) -> CMD_COMPLETE_W {
        CMD_COMPLETE_W { w: self }
    }
}
