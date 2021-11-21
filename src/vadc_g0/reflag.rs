#[doc = "Register `REFLAG` reader"]
pub struct R(crate::R<REFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFLAG` writer"]
pub struct W(crate::W<REFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFLAG_SPEC>;
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
impl From<crate::W<REFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Result Event for Result Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV0_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV0_A> for bool {
    #[inline(always)]
    fn from(variant: REV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV0` reader - Result Event for Result Register 0"]
pub struct REV0_R(crate::FieldReader<bool, REV0_A>);
impl REV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV0_A {
        match self.bits {
            false => REV0_A::VALUE1,
            true => REV0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV0_A::VALUE2
    }
}
impl core::ops::Deref for REV0_R {
    type Target = crate::FieldReader<bool, REV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV0` writer - Result Event for Result Register 0"]
pub struct REV0_W<'a> {
    w: &'a mut W,
}
impl<'a> REV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0_A::VALUE2)
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
#[doc = "Result Event for Result Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV1_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV1_A> for bool {
    #[inline(always)]
    fn from(variant: REV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV1` reader - Result Event for Result Register 1"]
pub struct REV1_R(crate::FieldReader<bool, REV1_A>);
impl REV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV1_A {
        match self.bits {
            false => REV1_A::VALUE1,
            true => REV1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV1_A::VALUE2
    }
}
impl core::ops::Deref for REV1_R {
    type Target = crate::FieldReader<bool, REV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV1` writer - Result Event for Result Register 1"]
pub struct REV1_W<'a> {
    w: &'a mut W,
}
impl<'a> REV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV1_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV1_A::VALUE2)
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
#[doc = "Result Event for Result Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV2_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV2_A> for bool {
    #[inline(always)]
    fn from(variant: REV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV2` reader - Result Event for Result Register 2"]
pub struct REV2_R(crate::FieldReader<bool, REV2_A>);
impl REV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV2_A {
        match self.bits {
            false => REV2_A::VALUE1,
            true => REV2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV2_A::VALUE2
    }
}
impl core::ops::Deref for REV2_R {
    type Target = crate::FieldReader<bool, REV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV2` writer - Result Event for Result Register 2"]
pub struct REV2_W<'a> {
    w: &'a mut W,
}
impl<'a> REV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV2_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV2_A::VALUE2)
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
#[doc = "Result Event for Result Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV3_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV3_A> for bool {
    #[inline(always)]
    fn from(variant: REV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV3` reader - Result Event for Result Register 3"]
pub struct REV3_R(crate::FieldReader<bool, REV3_A>);
impl REV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV3_A {
        match self.bits {
            false => REV3_A::VALUE1,
            true => REV3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV3_A::VALUE2
    }
}
impl core::ops::Deref for REV3_R {
    type Target = crate::FieldReader<bool, REV3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV3` writer - Result Event for Result Register 3"]
pub struct REV3_W<'a> {
    w: &'a mut W,
}
impl<'a> REV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV3_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV3_A::VALUE2)
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
#[doc = "Result Event for Result Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV4_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV4_A> for bool {
    #[inline(always)]
    fn from(variant: REV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV4` reader - Result Event for Result Register 4"]
pub struct REV4_R(crate::FieldReader<bool, REV4_A>);
impl REV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV4_A {
        match self.bits {
            false => REV4_A::VALUE1,
            true => REV4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV4_A::VALUE2
    }
}
impl core::ops::Deref for REV4_R {
    type Target = crate::FieldReader<bool, REV4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV4` writer - Result Event for Result Register 4"]
pub struct REV4_W<'a> {
    w: &'a mut W,
}
impl<'a> REV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV4_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV4_A::VALUE2)
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
#[doc = "Result Event for Result Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV5_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV5_A> for bool {
    #[inline(always)]
    fn from(variant: REV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV5` reader - Result Event for Result Register 5"]
pub struct REV5_R(crate::FieldReader<bool, REV5_A>);
impl REV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV5_A {
        match self.bits {
            false => REV5_A::VALUE1,
            true => REV5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV5_A::VALUE2
    }
}
impl core::ops::Deref for REV5_R {
    type Target = crate::FieldReader<bool, REV5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV5` writer - Result Event for Result Register 5"]
pub struct REV5_W<'a> {
    w: &'a mut W,
}
impl<'a> REV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV5_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV5_A::VALUE2)
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
#[doc = "Result Event for Result Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV6_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV6_A> for bool {
    #[inline(always)]
    fn from(variant: REV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV6` reader - Result Event for Result Register 6"]
pub struct REV6_R(crate::FieldReader<bool, REV6_A>);
impl REV6_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV6_A {
        match self.bits {
            false => REV6_A::VALUE1,
            true => REV6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV6_A::VALUE2
    }
}
impl core::ops::Deref for REV6_R {
    type Target = crate::FieldReader<bool, REV6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV6` writer - Result Event for Result Register 6"]
pub struct REV6_W<'a> {
    w: &'a mut W,
}
impl<'a> REV6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV6_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV6_A::VALUE2)
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
#[doc = "Result Event for Result Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV7_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV7_A> for bool {
    #[inline(always)]
    fn from(variant: REV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV7` reader - Result Event for Result Register 7"]
pub struct REV7_R(crate::FieldReader<bool, REV7_A>);
impl REV7_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV7_A {
        match self.bits {
            false => REV7_A::VALUE1,
            true => REV7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV7_A::VALUE2
    }
}
impl core::ops::Deref for REV7_R {
    type Target = crate::FieldReader<bool, REV7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV7` writer - Result Event for Result Register 7"]
pub struct REV7_W<'a> {
    w: &'a mut W,
}
impl<'a> REV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV7_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV7_A::VALUE2)
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
#[doc = "Result Event for Result Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV8_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV8_A> for bool {
    #[inline(always)]
    fn from(variant: REV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV8` reader - Result Event for Result Register 8"]
pub struct REV8_R(crate::FieldReader<bool, REV8_A>);
impl REV8_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV8_A {
        match self.bits {
            false => REV8_A::VALUE1,
            true => REV8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV8_A::VALUE2
    }
}
impl core::ops::Deref for REV8_R {
    type Target = crate::FieldReader<bool, REV8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV8` writer - Result Event for Result Register 8"]
pub struct REV8_W<'a> {
    w: &'a mut W,
}
impl<'a> REV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV8_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV8_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Result Event for Result Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV9_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV9_A> for bool {
    #[inline(always)]
    fn from(variant: REV9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV9` reader - Result Event for Result Register 9"]
pub struct REV9_R(crate::FieldReader<bool, REV9_A>);
impl REV9_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV9_A {
        match self.bits {
            false => REV9_A::VALUE1,
            true => REV9_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV9_A::VALUE2
    }
}
impl core::ops::Deref for REV9_R {
    type Target = crate::FieldReader<bool, REV9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV9` writer - Result Event for Result Register 9"]
pub struct REV9_W<'a> {
    w: &'a mut W,
}
impl<'a> REV9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV9_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV9_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Result Event for Result Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV10_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV10_A> for bool {
    #[inline(always)]
    fn from(variant: REV10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV10` reader - Result Event for Result Register 10"]
pub struct REV10_R(crate::FieldReader<bool, REV10_A>);
impl REV10_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV10_A {
        match self.bits {
            false => REV10_A::VALUE1,
            true => REV10_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV10_A::VALUE2
    }
}
impl core::ops::Deref for REV10_R {
    type Target = crate::FieldReader<bool, REV10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV10` writer - Result Event for Result Register 10"]
pub struct REV10_W<'a> {
    w: &'a mut W,
}
impl<'a> REV10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV10_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV10_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Result Event for Result Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV11_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV11_A> for bool {
    #[inline(always)]
    fn from(variant: REV11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV11` reader - Result Event for Result Register 11"]
pub struct REV11_R(crate::FieldReader<bool, REV11_A>);
impl REV11_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV11_A {
        match self.bits {
            false => REV11_A::VALUE1,
            true => REV11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV11_A::VALUE2
    }
}
impl core::ops::Deref for REV11_R {
    type Target = crate::FieldReader<bool, REV11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV11` writer - Result Event for Result Register 11"]
pub struct REV11_W<'a> {
    w: &'a mut W,
}
impl<'a> REV11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV11_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV11_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Result Event for Result Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV12_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV12_A> for bool {
    #[inline(always)]
    fn from(variant: REV12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV12` reader - Result Event for Result Register 12"]
pub struct REV12_R(crate::FieldReader<bool, REV12_A>);
impl REV12_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV12_A {
        match self.bits {
            false => REV12_A::VALUE1,
            true => REV12_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV12_A::VALUE2
    }
}
impl core::ops::Deref for REV12_R {
    type Target = crate::FieldReader<bool, REV12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV12` writer - Result Event for Result Register 12"]
pub struct REV12_W<'a> {
    w: &'a mut W,
}
impl<'a> REV12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV12_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV12_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Result Event for Result Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV13_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV13_A> for bool {
    #[inline(always)]
    fn from(variant: REV13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV13` reader - Result Event for Result Register 13"]
pub struct REV13_R(crate::FieldReader<bool, REV13_A>);
impl REV13_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV13_A {
        match self.bits {
            false => REV13_A::VALUE1,
            true => REV13_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV13_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV13_A::VALUE2
    }
}
impl core::ops::Deref for REV13_R {
    type Target = crate::FieldReader<bool, REV13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV13` writer - Result Event for Result Register 13"]
pub struct REV13_W<'a> {
    w: &'a mut W,
}
impl<'a> REV13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV13_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV13_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Result Event for Result Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV14_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV14_A> for bool {
    #[inline(always)]
    fn from(variant: REV14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV14` reader - Result Event for Result Register 14"]
pub struct REV14_R(crate::FieldReader<bool, REV14_A>);
impl REV14_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV14_A {
        match self.bits {
            false => REV14_A::VALUE1,
            true => REV14_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV14_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV14_A::VALUE2
    }
}
impl core::ops::Deref for REV14_R {
    type Target = crate::FieldReader<bool, REV14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV14` writer - Result Event for Result Register 14"]
pub struct REV14_W<'a> {
    w: &'a mut W,
}
impl<'a> REV14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV14_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV14_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Result Event for Result Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV15_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    VALUE2 = 1,
}
impl From<REV15_A> for bool {
    #[inline(always)]
    fn from(variant: REV15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV15` reader - Result Event for Result Register 15"]
pub struct REV15_R(crate::FieldReader<bool, REV15_A>);
impl REV15_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV15_A {
        match self.bits {
            false => REV15_A::VALUE1,
            true => REV15_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REV15_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REV15_A::VALUE2
    }
}
impl core::ops::Deref for REV15_R {
    type Target = crate::FieldReader<bool, REV15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV15` writer - Result Event for Result Register 15"]
pub struct REV15_W<'a> {
    w: &'a mut W,
}
impl<'a> REV15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV15_A::VALUE1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV15_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline(always)]
    pub fn rev0(&self) -> REV0_R {
        REV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline(always)]
    pub fn rev1(&self) -> REV1_R {
        REV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline(always)]
    pub fn rev2(&self) -> REV2_R {
        REV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline(always)]
    pub fn rev3(&self) -> REV3_R {
        REV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline(always)]
    pub fn rev4(&self) -> REV4_R {
        REV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline(always)]
    pub fn rev5(&self) -> REV5_R {
        REV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline(always)]
    pub fn rev6(&self) -> REV6_R {
        REV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline(always)]
    pub fn rev7(&self) -> REV7_R {
        REV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline(always)]
    pub fn rev8(&self) -> REV8_R {
        REV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline(always)]
    pub fn rev9(&self) -> REV9_R {
        REV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline(always)]
    pub fn rev10(&self) -> REV10_R {
        REV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline(always)]
    pub fn rev11(&self) -> REV11_R {
        REV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline(always)]
    pub fn rev12(&self) -> REV12_R {
        REV12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline(always)]
    pub fn rev13(&self) -> REV13_R {
        REV13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline(always)]
    pub fn rev14(&self) -> REV14_R {
        REV14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline(always)]
    pub fn rev15(&self) -> REV15_R {
        REV15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline(always)]
    pub fn rev0(&mut self) -> REV0_W {
        REV0_W { w: self }
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline(always)]
    pub fn rev1(&mut self) -> REV1_W {
        REV1_W { w: self }
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline(always)]
    pub fn rev2(&mut self) -> REV2_W {
        REV2_W { w: self }
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline(always)]
    pub fn rev3(&mut self) -> REV3_W {
        REV3_W { w: self }
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline(always)]
    pub fn rev4(&mut self) -> REV4_W {
        REV4_W { w: self }
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline(always)]
    pub fn rev5(&mut self) -> REV5_W {
        REV5_W { w: self }
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline(always)]
    pub fn rev6(&mut self) -> REV6_W {
        REV6_W { w: self }
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline(always)]
    pub fn rev7(&mut self) -> REV7_W {
        REV7_W { w: self }
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline(always)]
    pub fn rev8(&mut self) -> REV8_W {
        REV8_W { w: self }
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline(always)]
    pub fn rev9(&mut self) -> REV9_W {
        REV9_W { w: self }
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline(always)]
    pub fn rev10(&mut self) -> REV10_W {
        REV10_W { w: self }
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline(always)]
    pub fn rev11(&mut self) -> REV11_W {
        REV11_W { w: self }
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline(always)]
    pub fn rev12(&mut self) -> REV12_W {
        REV12_W { w: self }
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline(always)]
    pub fn rev13(&mut self) -> REV13_W {
        REV13_W { w: self }
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline(always)]
    pub fn rev14(&mut self) -> REV14_W {
        REV14_W { w: self }
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline(always)]
    pub fn rev15(&mut self) -> REV15_W {
        REV15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reflag](index.html) module"]
pub struct REFLAG_SPEC;
impl crate::RegisterSpec for REFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reflag::R](R) reader structure"]
impl crate::Readable for REFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reflag::W](W) writer structure"]
impl crate::Writable for REFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFLAG to value 0"]
impl crate::Resettable for REFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
