#[doc = "Register `ASSEL` reader"]
pub struct R(crate::R<ASSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASSEL` writer"]
pub struct W(crate::W<ASSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASSEL_SPEC>;
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
impl From<crate::W<ASSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL0` reader - Channel Selection"]
pub struct CHSEL0_R(crate::FieldReader<bool, CHSEL0_A>);
impl CHSEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL0_A {
        match self.bits {
            false => CHSEL0_A::VALUE1,
            true => CHSEL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL0_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL0_R {
    type Target = crate::FieldReader<bool, CHSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL0` writer - Channel Selection"]
pub struct CHSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL0_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL0_A::VALUE2)
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
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL1` reader - Channel Selection"]
pub struct CHSEL1_R(crate::FieldReader<bool, CHSEL1_A>);
impl CHSEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL1_A {
        match self.bits {
            false => CHSEL1_A::VALUE1,
            true => CHSEL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL1_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL1_R {
    type Target = crate::FieldReader<bool, CHSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL1` writer - Channel Selection"]
pub struct CHSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL1_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL1_A::VALUE2)
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
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL2` reader - Channel Selection"]
pub struct CHSEL2_R(crate::FieldReader<bool, CHSEL2_A>);
impl CHSEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL2_A {
        match self.bits {
            false => CHSEL2_A::VALUE1,
            true => CHSEL2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL2_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL2_R {
    type Target = crate::FieldReader<bool, CHSEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL2` writer - Channel Selection"]
pub struct CHSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL2_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL2_A::VALUE2)
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
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL3` reader - Channel Selection"]
pub struct CHSEL3_R(crate::FieldReader<bool, CHSEL3_A>);
impl CHSEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL3_A {
        match self.bits {
            false => CHSEL3_A::VALUE1,
            true => CHSEL3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL3_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL3_R {
    type Target = crate::FieldReader<bool, CHSEL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL3` writer - Channel Selection"]
pub struct CHSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL3_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL3_A::VALUE2)
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
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL4` reader - Channel Selection"]
pub struct CHSEL4_R(crate::FieldReader<bool, CHSEL4_A>);
impl CHSEL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL4_A {
        match self.bits {
            false => CHSEL4_A::VALUE1,
            true => CHSEL4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL4_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL4_R {
    type Target = crate::FieldReader<bool, CHSEL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL4` writer - Channel Selection"]
pub struct CHSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL4_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL4_A::VALUE2)
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
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL5` reader - Channel Selection"]
pub struct CHSEL5_R(crate::FieldReader<bool, CHSEL5_A>);
impl CHSEL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL5_A {
        match self.bits {
            false => CHSEL5_A::VALUE1,
            true => CHSEL5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL5_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL5_R {
    type Target = crate::FieldReader<bool, CHSEL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL5` writer - Channel Selection"]
pub struct CHSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL5_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL5_A::VALUE2)
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
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL6` reader - Channel Selection"]
pub struct CHSEL6_R(crate::FieldReader<bool, CHSEL6_A>);
impl CHSEL6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL6_A {
        match self.bits {
            false => CHSEL6_A::VALUE1,
            true => CHSEL6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL6_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL6_R {
    type Target = crate::FieldReader<bool, CHSEL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL6` writer - Channel Selection"]
pub struct CHSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL6_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL6_A::VALUE2)
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
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL7_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL7` reader - Channel Selection"]
pub struct CHSEL7_R(crate::FieldReader<bool, CHSEL7_A>);
impl CHSEL7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL7_A {
        match self.bits {
            false => CHSEL7_A::VALUE1,
            true => CHSEL7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHSEL7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHSEL7_A::VALUE2
    }
}
impl core::ops::Deref for CHSEL7_R {
    type Target = crate::FieldReader<bool, CHSEL7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL7` writer - Channel Selection"]
pub struct CHSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL7_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL7_A::VALUE2)
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
    #[doc = "Bit 0 - Channel Selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Selection"]
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W {
        CHSEL0_W { w: self }
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W {
        CHSEL1_W { w: self }
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W {
        CHSEL2_W { w: self }
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL3_W {
        CHSEL3_W { w: self }
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL4_W {
        CHSEL4_W { w: self }
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W {
        CHSEL5_W { w: self }
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W {
        CHSEL6_W { w: self }
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL7_W {
        CHSEL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autoscan Source Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assel](index.html) module"]
pub struct ASSEL_SPEC;
impl crate::RegisterSpec for ASSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [assel::R](R) reader structure"]
impl crate::Readable for ASSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [assel::W](W) writer structure"]
impl crate::Writable for ASSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASSEL to value 0"]
impl crate::Resettable for ASSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
