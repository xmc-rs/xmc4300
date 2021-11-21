#[doc = "Register `EN_INT_SIGNAL_NORM` reader"]
pub struct R(crate::R<EN_INT_SIGNAL_NORM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_INT_SIGNAL_NORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_INT_SIGNAL_NORM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_INT_SIGNAL_NORM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_INT_SIGNAL_NORM` writer"]
pub struct W(crate::W<EN_INT_SIGNAL_NORM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_INT_SIGNAL_NORM_SPEC>;
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
impl From<crate::W<EN_INT_SIGNAL_NORM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_INT_SIGNAL_NORM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIXED_TO_0` reader - Fixed to 0"]
pub struct FIXED_TO_0_R(crate::FieldReader<bool, bool>);
impl FIXED_TO_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIXED_TO_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXED_TO_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `CARD_INT_EN` reader - Card Interrupt Signal Enable"]
pub struct CARD_INT_EN_R(crate::FieldReader<bool, CARD_INT_EN_A>);
impl CARD_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_INT_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == CARD_INT_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_INT_EN_A::VALUE2
    }
}
impl core::ops::Deref for CARD_INT_EN_R {
    type Target = crate::FieldReader<bool, CARD_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_INT_EN` writer - Card Interrupt Signal Enable"]
pub struct CARD_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
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
#[doc = "Field `CARD_REMOVAL_EN` reader - Card Removal Signal Enable"]
pub struct CARD_REMOVAL_EN_R(crate::FieldReader<bool, CARD_REMOVAL_EN_A>);
impl CARD_REMOVAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_REMOVAL_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == CARD_REMOVAL_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_REMOVAL_EN_A::VALUE2
    }
}
impl core::ops::Deref for CARD_REMOVAL_EN_R {
    type Target = crate::FieldReader<bool, CARD_REMOVAL_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_REMOVAL_EN` writer - Card Removal Signal Enable"]
pub struct CARD_REMOVAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_REMOVAL_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
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
#[doc = "Field `CARD_INS_EN` reader - Card Insertion Signal Enable"]
pub struct CARD_INS_EN_R(crate::FieldReader<bool, CARD_INS_EN_A>);
impl CARD_INS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_INS_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == CARD_INS_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_INS_EN_A::VALUE2
    }
}
impl core::ops::Deref for CARD_INS_EN_R {
    type Target = crate::FieldReader<bool, CARD_INS_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_INS_EN` writer - Card Insertion Signal Enable"]
pub struct CARD_INS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_INS_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
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
#[doc = "Field `BUFF_READ_READY_EN` reader - Buffer Read Ready Signal Enable"]
pub struct BUFF_READ_READY_EN_R(crate::FieldReader<bool, BUFF_READ_READY_EN_A>);
impl BUFF_READ_READY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFF_READ_READY_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == BUFF_READ_READY_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUFF_READ_READY_EN_A::VALUE2
    }
}
impl core::ops::Deref for BUFF_READ_READY_EN_R {
    type Target = crate::FieldReader<bool, BUFF_READ_READY_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_READ_READY_EN` writer - Buffer Read Ready Signal Enable"]
pub struct BUFF_READ_READY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_READ_READY_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_READ_READY_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
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
#[doc = "Field `BUFF_WRITE_READY_EN` reader - Buffer Write Ready Signal Enable"]
pub struct BUFF_WRITE_READY_EN_R(crate::FieldReader<bool, BUFF_WRITE_READY_EN_A>);
impl BUFF_WRITE_READY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFF_WRITE_READY_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == BUFF_WRITE_READY_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUFF_WRITE_READY_EN_A::VALUE2
    }
}
impl core::ops::Deref for BUFF_WRITE_READY_EN_R {
    type Target = crate::FieldReader<bool, BUFF_WRITE_READY_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_WRITE_READY_EN` writer - Buffer Write Ready Signal Enable"]
pub struct BUFF_WRITE_READY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_WRITE_READY_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFF_WRITE_READY_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
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
#[doc = "Field `BLOCK_GAP_EVENT_EN` reader - Block Gap Event Signal Enable"]
pub struct BLOCK_GAP_EVENT_EN_R(crate::FieldReader<bool, BLOCK_GAP_EVENT_EN_A>);
impl BLOCK_GAP_EVENT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLOCK_GAP_EVENT_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == BLOCK_GAP_EVENT_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BLOCK_GAP_EVENT_EN_A::VALUE2
    }
}
impl core::ops::Deref for BLOCK_GAP_EVENT_EN_R {
    type Target = crate::FieldReader<bool, BLOCK_GAP_EVENT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_GAP_EVENT_EN` writer - Block Gap Event Signal Enable"]
pub struct BLOCK_GAP_EVENT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_GAP_EVENT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCK_GAP_EVENT_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
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
#[doc = "Field `TX_COMPLETE_EN` reader - Transfer Complete Signal Enable"]
pub struct TX_COMPLETE_EN_R(crate::FieldReader<bool, TX_COMPLETE_EN_A>);
impl TX_COMPLETE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_COMPLETE_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == TX_COMPLETE_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TX_COMPLETE_EN_A::VALUE2
    }
}
impl core::ops::Deref for TX_COMPLETE_EN_R {
    type Target = crate::FieldReader<bool, TX_COMPLETE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_COMPLETE_EN` writer - Transfer Complete Signal Enable"]
pub struct TX_COMPLETE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_COMPLETE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_COMPLETE_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
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
#[doc = "Field `CMD_COMPLETE_EN` reader - Command Complete Signal Enable"]
pub struct CMD_COMPLETE_EN_R(crate::FieldReader<bool, CMD_COMPLETE_EN_A>);
impl CMD_COMPLETE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_COMPLETE_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == CMD_COMPLETE_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_COMPLETE_EN_A::VALUE2
    }
}
impl core::ops::Deref for CMD_COMPLETE_EN_R {
    type Target = crate::FieldReader<bool, CMD_COMPLETE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_COMPLETE_EN` writer - Command Complete Signal Enable"]
pub struct CMD_COMPLETE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_COMPLETE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_COMPLETE_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_int_signal_norm](index.html) module"]
pub struct EN_INT_SIGNAL_NORM_SPEC;
impl crate::RegisterSpec for EN_INT_SIGNAL_NORM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [en_int_signal_norm::R](R) reader structure"]
impl crate::Readable for EN_INT_SIGNAL_NORM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_int_signal_norm::W](W) writer structure"]
impl crate::Writable for EN_INT_SIGNAL_NORM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_INT_SIGNAL_NORM to value 0"]
impl crate::Resettable for EN_INT_SIGNAL_NORM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
