#[doc = "Register `BRSSEL[%s]` reader"]
pub struct R(crate::R<BRSSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRSSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRSSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRSSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRSSEL[%s]` writer"]
pub struct W(crate::W<BRSSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRSSEL_SPEC>;
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
impl From<crate::W<BRSSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRSSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG0` reader - Channel Selection Group x"]
pub struct CHSELG0_R(crate::FieldReader<bool, CHSELG0_A>);
impl CHSELG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG0_A {
        match self.bits {
            false => CHSELG0_A::VALUE1,
            true => CHSELG0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG0_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG0_R {
    type Target = crate::FieldReader<bool, CHSELG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG0` writer - Channel Selection Group x"]
pub struct CHSELG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG0_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG1_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG1` reader - Channel Selection Group x"]
pub struct CHSELG1_R(crate::FieldReader<bool, CHSELG1_A>);
impl CHSELG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG1_A {
        match self.bits {
            false => CHSELG1_A::VALUE1,
            true => CHSELG1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG1_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG1_R {
    type Target = crate::FieldReader<bool, CHSELG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG1` writer - Channel Selection Group x"]
pub struct CHSELG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG1_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG2_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG2` reader - Channel Selection Group x"]
pub struct CHSELG2_R(crate::FieldReader<bool, CHSELG2_A>);
impl CHSELG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG2_A {
        match self.bits {
            false => CHSELG2_A::VALUE1,
            true => CHSELG2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG2_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG2_R {
    type Target = crate::FieldReader<bool, CHSELG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG2` writer - Channel Selection Group x"]
pub struct CHSELG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG2_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG3_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG3` reader - Channel Selection Group x"]
pub struct CHSELG3_R(crate::FieldReader<bool, CHSELG3_A>);
impl CHSELG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG3_A {
        match self.bits {
            false => CHSELG3_A::VALUE1,
            true => CHSELG3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG3_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG3_R {
    type Target = crate::FieldReader<bool, CHSELG3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG3` writer - Channel Selection Group x"]
pub struct CHSELG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG3_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG4_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG4` reader - Channel Selection Group x"]
pub struct CHSELG4_R(crate::FieldReader<bool, CHSELG4_A>);
impl CHSELG4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG4_A {
        match self.bits {
            false => CHSELG4_A::VALUE1,
            true => CHSELG4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG4_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG4_R {
    type Target = crate::FieldReader<bool, CHSELG4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG4` writer - Channel Selection Group x"]
pub struct CHSELG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG4_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG4_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG5_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG5` reader - Channel Selection Group x"]
pub struct CHSELG5_R(crate::FieldReader<bool, CHSELG5_A>);
impl CHSELG5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG5_A {
        match self.bits {
            false => CHSELG5_A::VALUE1,
            true => CHSELG5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG5_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG5_R {
    type Target = crate::FieldReader<bool, CHSELG5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG5` writer - Channel Selection Group x"]
pub struct CHSELG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG5_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG5_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG6_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG6` reader - Channel Selection Group x"]
pub struct CHSELG6_R(crate::FieldReader<bool, CHSELG6_A>);
impl CHSELG6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG6_A {
        match self.bits {
            false => CHSELG6_A::VALUE1,
            true => CHSELG6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG6_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG6_R {
    type Target = crate::FieldReader<bool, CHSELG6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG6` writer - Channel Selection Group x"]
pub struct CHSELG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG6_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG6_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELG7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSELG7_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELG7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG7` reader - Channel Selection Group x"]
pub struct CHSELG7_R(crate::FieldReader<bool, CHSELG7_A>);
impl CHSELG7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELG7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELG7_A {
        match self.bits {
            false => CHSELG7_A::VALUE1,
            true => CHSELG7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSELG7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSELG7_A::VALUE2
    }
}
impl core::ops::Deref for CHSELG7_R {
    type Target = crate::FieldReader<bool, CHSELG7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSELG7` writer - Channel Selection Group x"]
pub struct CHSELG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSELG7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSELG7_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSELG7_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg0(&self) -> CHSELG0_R {
        CHSELG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg1(&self) -> CHSELG1_R {
        CHSELG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg2(&self) -> CHSELG2_R {
        CHSELG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg3(&self) -> CHSELG3_R {
        CHSELG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg4(&self) -> CHSELG4_R {
        CHSELG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg5(&self) -> CHSELG5_R {
        CHSELG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg6(&self) -> CHSELG6_R {
        CHSELG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg7(&self) -> CHSELG7_R {
        CHSELG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg0(&mut self) -> CHSELG0_W {
        CHSELG0_W { w: self }
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg1(&mut self) -> CHSELG1_W {
        CHSELG1_W { w: self }
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg2(&mut self) -> CHSELG2_W {
        CHSELG2_W { w: self }
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg3(&mut self) -> CHSELG3_W {
        CHSELG3_W { w: self }
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg4(&mut self) -> CHSELG4_W {
        CHSELG4_W { w: self }
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg5(&mut self) -> CHSELG5_W {
        CHSELG5_W { w: self }
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg6(&mut self) -> CHSELG6_W {
        CHSELG6_W { w: self }
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg7(&mut self) -> CHSELG7_W {
        CHSELG7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Request Source Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brssel](index.html) module"]
pub struct BRSSEL_SPEC;
impl crate::RegisterSpec for BRSSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brssel::R](R) reader structure"]
impl crate::Readable for BRSSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brssel::W](W) writer structure"]
impl crate::Writable for BRSSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRSSEL[%s]
to value 0"]
impl crate::Resettable for BRSSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
