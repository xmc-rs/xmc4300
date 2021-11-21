#[doc = "Register `INT_STATUS_NORM` reader"]
pub struct R(crate::R<INT_STATUS_NORM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_NORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_NORM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_NORM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STATUS_NORM` writer"]
pub struct W(crate::W<INT_STATUS_NORM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STATUS_NORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_STATUS_NORM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STATUS_NORM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_INT_A {
    #[doc = "0: No Error."]
    VALUE1 = 0,
    #[doc = "1: Error."]
    VALUE2 = 1,
}
impl From<ERR_INT_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_INT` reader - Error Interrupt"]
pub struct ERR_INT_R(crate::FieldReader<bool, ERR_INT_A>);
impl ERR_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_INT_R(crate::FieldReader::new(bits))
    }
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
        **self == ERR_INT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERR_INT_A::VALUE2
    }
}
impl core::ops::Deref for ERR_INT_R {
    type Target = crate::FieldReader<bool, ERR_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_A {
    #[doc = "0: No Card Interrupt"]
    VALUE1 = 0,
    #[doc = "1: Generate Card Interrupt"]
    VALUE2 = 1,
}
impl From<CARD_INT_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INT` reader - Card Interrupt"]
pub struct CARD_INT_R(crate::FieldReader<bool, CARD_INT_A>);
impl CARD_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_INT_R(crate::FieldReader::new(bits))
    }
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
        **self == CARD_INT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_INT_A::VALUE2
    }
}
impl core::ops::Deref for CARD_INT_R {
    type Target = crate::FieldReader<bool, CARD_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_REMOVAL_A {
    #[doc = "0: Card State Stable or Debouncing"]
    VALUE1 = 0,
    #[doc = "1: Card Removed"]
    VALUE2 = 1,
}
impl From<CARD_REMOVAL_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_REMOVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_REMOVAL` reader - Card Removal"]
pub struct CARD_REMOVAL_R(crate::FieldReader<bool, CARD_REMOVAL_A>);
impl CARD_REMOVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_REMOVAL_R(crate::FieldReader::new(bits))
    }
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
        **self == CARD_REMOVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_REMOVAL_A::VALUE2
    }
}
impl core::ops::Deref for CARD_REMOVAL_R {
    type Target = crate::FieldReader<bool, CARD_REMOVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_REMOVAL` writer - Card Removal"]
pub struct CARD_REMOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_REMOVAL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INS_A {
    #[doc = "0: Card State Stable or Debouncing"]
    VALUE1 = 0,
    #[doc = "1: Card Inserted"]
    VALUE2 = 1,
}
impl From<CARD_INS_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INS` reader - Card Insertion"]
pub struct CARD_INS_R(crate::FieldReader<bool, CARD_INS_A>);
impl CARD_INS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_INS_R(crate::FieldReader::new(bits))
    }
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
        **self == CARD_INS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_INS_A::VALUE2
    }
}
impl core::ops::Deref for CARD_INS_R {
    type Target = crate::FieldReader<bool, CARD_INS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_INS` writer - Card Insertion"]
pub struct CARD_INS_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_INS_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_READ_READY_A {
    #[doc = "0: Not Ready to read Buffer."]
    VALUE1 = 0,
    #[doc = "1: Ready to read Buffer."]
    VALUE2 = 1,
}
impl From<BUFF_READ_READY_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_READ_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_READ_READY` reader - Buffer Read Ready"]
pub struct BUFF_READ_READY_R(crate::FieldReader<bool, BUFF_READ_READY_A>);
impl BUFF_READ_READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFF_READ_READY_R(crate::FieldReader::new(bits))
    }
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
        **self == BUFF_READ_READY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUFF_READ_READY_A::VALUE2
    }
}
impl core::ops::Deref for BUFF_READ_READY_R {
    type Target = crate::FieldReader<bool, BUFF_READ_READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_READ_READY` writer - Buffer Read Ready"]
pub struct BUFF_READ_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_READ_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_READ_READY_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFF_WRITE_READY_A {
    #[doc = "0: Not Ready to Write Buffer."]
    VALUE1 = 0,
    #[doc = "1: Ready to Write Buffer."]
    VALUE2 = 1,
}
impl From<BUFF_WRITE_READY_A> for bool {
    #[inline(always)]
    fn from(variant: BUFF_WRITE_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFF_WRITE_READY` reader - Buffer Write Ready"]
pub struct BUFF_WRITE_READY_R(crate::FieldReader<bool, BUFF_WRITE_READY_A>);
impl BUFF_WRITE_READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFF_WRITE_READY_R(crate::FieldReader::new(bits))
    }
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
        **self == BUFF_WRITE_READY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUFF_WRITE_READY_A::VALUE2
    }
}
impl core::ops::Deref for BUFF_WRITE_READY_R {
    type Target = crate::FieldReader<bool, BUFF_WRITE_READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_WRITE_READY` writer - Buffer Write Ready"]
pub struct BUFF_WRITE_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_WRITE_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_WRITE_READY_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_GAP_EVENT_A {
    #[doc = "0: No Block Gap Event"]
    VALUE1 = 0,
    #[doc = "1: Transaction stopped at Block Gap"]
    VALUE2 = 1,
}
impl From<BLOCK_GAP_EVENT_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_GAP_EVENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCK_GAP_EVENT` reader - Block Gap Event"]
pub struct BLOCK_GAP_EVENT_R(crate::FieldReader<bool, BLOCK_GAP_EVENT_A>);
impl BLOCK_GAP_EVENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLOCK_GAP_EVENT_R(crate::FieldReader::new(bits))
    }
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
        **self == BLOCK_GAP_EVENT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BLOCK_GAP_EVENT_A::VALUE2
    }
}
impl core::ops::Deref for BLOCK_GAP_EVENT_R {
    type Target = crate::FieldReader<bool, BLOCK_GAP_EVENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_GAP_EVENT` writer - Block Gap Event"]
pub struct BLOCK_GAP_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_GAP_EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCK_GAP_EVENT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_COMPLETE_A {
    #[doc = "0: No Data Transfer Complete"]
    VALUE1 = 0,
    #[doc = "1: Data Transfer Complete"]
    VALUE2 = 1,
}
impl From<TX_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_COMPLETE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_COMPLETE` reader - Transfer Complete"]
pub struct TX_COMPLETE_R(crate::FieldReader<bool, TX_COMPLETE_A>);
impl TX_COMPLETE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_COMPLETE_R(crate::FieldReader::new(bits))
    }
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
        **self == TX_COMPLETE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TX_COMPLETE_A::VALUE2
    }
}
impl core::ops::Deref for TX_COMPLETE_R {
    type Target = crate::FieldReader<bool, TX_COMPLETE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_COMPLETE` writer - Transfer Complete"]
pub struct TX_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_COMPLETE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_COMPLETE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_COMPLETE_A {
    #[doc = "0: No Command Complete"]
    VALUE1 = 0,
    #[doc = "1: Command Complete"]
    VALUE2 = 1,
}
impl From<CMD_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_COMPLETE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_COMPLETE` reader - Command Complete"]
pub struct CMD_COMPLETE_R(crate::FieldReader<bool, CMD_COMPLETE_A>);
impl CMD_COMPLETE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_COMPLETE_R(crate::FieldReader::new(bits))
    }
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
        **self == CMD_COMPLETE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_COMPLETE_A::VALUE2
    }
}
impl core::ops::Deref for CMD_COMPLETE_R {
    type Target = crate::FieldReader<bool, CMD_COMPLETE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_COMPLETE` writer - Command Complete"]
pub struct CMD_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_COMPLETE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_COMPLETE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status_norm](index.html) module"]
pub struct INT_STATUS_NORM_SPEC;
impl crate::RegisterSpec for INT_STATUS_NORM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [int_status_norm::R](R) reader structure"]
impl crate::Readable for INT_STATUS_NORM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_status_norm::W](W) writer structure"]
impl crate::Writable for INT_STATUS_NORM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_STATUS_NORM to value 0"]
impl crate::Resettable for INT_STATUS_NORM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
