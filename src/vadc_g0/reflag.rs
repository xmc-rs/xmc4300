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
#[doc = "Field `REV0` reader - Result Event for Result Register 0"]
pub type REV0_R = crate::BitReader<REV0_A>;
#[doc = "Result Event for Result Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV0_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV0_A::VALUE2
    }
}
#[doc = "Field `REV0` writer - Result Event for Result Register 0"]
pub type REV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV0_A, O>;
impl<'a, const O: u8> REV0_W<'a, O> {
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
}
#[doc = "Field `REV1` reader - Result Event for Result Register 1"]
pub type REV1_R = crate::BitReader<REV1_A>;
#[doc = "Result Event for Result Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV1_A::VALUE2
    }
}
#[doc = "Field `REV1` writer - Result Event for Result Register 1"]
pub type REV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV1_A, O>;
impl<'a, const O: u8> REV1_W<'a, O> {
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
}
#[doc = "Field `REV2` reader - Result Event for Result Register 2"]
pub type REV2_R = crate::BitReader<REV2_A>;
#[doc = "Result Event for Result Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV2_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV2_A::VALUE2
    }
}
#[doc = "Field `REV2` writer - Result Event for Result Register 2"]
pub type REV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV2_A, O>;
impl<'a, const O: u8> REV2_W<'a, O> {
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
}
#[doc = "Field `REV3` reader - Result Event for Result Register 3"]
pub type REV3_R = crate::BitReader<REV3_A>;
#[doc = "Result Event for Result Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV3_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV3_A::VALUE2
    }
}
#[doc = "Field `REV3` writer - Result Event for Result Register 3"]
pub type REV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV3_A, O>;
impl<'a, const O: u8> REV3_W<'a, O> {
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
}
#[doc = "Field `REV4` reader - Result Event for Result Register 4"]
pub type REV4_R = crate::BitReader<REV4_A>;
#[doc = "Result Event for Result Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV4_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV4_A::VALUE2
    }
}
#[doc = "Field `REV4` writer - Result Event for Result Register 4"]
pub type REV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV4_A, O>;
impl<'a, const O: u8> REV4_W<'a, O> {
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
}
#[doc = "Field `REV5` reader - Result Event for Result Register 5"]
pub type REV5_R = crate::BitReader<REV5_A>;
#[doc = "Result Event for Result Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV5_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV5_A::VALUE2
    }
}
#[doc = "Field `REV5` writer - Result Event for Result Register 5"]
pub type REV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV5_A, O>;
impl<'a, const O: u8> REV5_W<'a, O> {
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
}
#[doc = "Field `REV6` reader - Result Event for Result Register 6"]
pub type REV6_R = crate::BitReader<REV6_A>;
#[doc = "Result Event for Result Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV6_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV6_A::VALUE2
    }
}
#[doc = "Field `REV6` writer - Result Event for Result Register 6"]
pub type REV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV6_A, O>;
impl<'a, const O: u8> REV6_W<'a, O> {
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
}
#[doc = "Field `REV7` reader - Result Event for Result Register 7"]
pub type REV7_R = crate::BitReader<REV7_A>;
#[doc = "Result Event for Result Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV7_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV7_A::VALUE2
    }
}
#[doc = "Field `REV7` writer - Result Event for Result Register 7"]
pub type REV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV7_A, O>;
impl<'a, const O: u8> REV7_W<'a, O> {
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
}
#[doc = "Field `REV8` reader - Result Event for Result Register 8"]
pub type REV8_R = crate::BitReader<REV8_A>;
#[doc = "Result Event for Result Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV8_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV8_A::VALUE2
    }
}
#[doc = "Field `REV8` writer - Result Event for Result Register 8"]
pub type REV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV8_A, O>;
impl<'a, const O: u8> REV8_W<'a, O> {
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
}
#[doc = "Field `REV9` reader - Result Event for Result Register 9"]
pub type REV9_R = crate::BitReader<REV9_A>;
#[doc = "Result Event for Result Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV9_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV9_A::VALUE2
    }
}
#[doc = "Field `REV9` writer - Result Event for Result Register 9"]
pub type REV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV9_A, O>;
impl<'a, const O: u8> REV9_W<'a, O> {
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
}
#[doc = "Field `REV10` reader - Result Event for Result Register 10"]
pub type REV10_R = crate::BitReader<REV10_A>;
#[doc = "Result Event for Result Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV10_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV10_A::VALUE2
    }
}
#[doc = "Field `REV10` writer - Result Event for Result Register 10"]
pub type REV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV10_A, O>;
impl<'a, const O: u8> REV10_W<'a, O> {
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
}
#[doc = "Field `REV11` reader - Result Event for Result Register 11"]
pub type REV11_R = crate::BitReader<REV11_A>;
#[doc = "Result Event for Result Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV11_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV11_A::VALUE2
    }
}
#[doc = "Field `REV11` writer - Result Event for Result Register 11"]
pub type REV11_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV11_A, O>;
impl<'a, const O: u8> REV11_W<'a, O> {
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
}
#[doc = "Field `REV12` reader - Result Event for Result Register 12"]
pub type REV12_R = crate::BitReader<REV12_A>;
#[doc = "Result Event for Result Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV12_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV12_A::VALUE2
    }
}
#[doc = "Field `REV12` writer - Result Event for Result Register 12"]
pub type REV12_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV12_A, O>;
impl<'a, const O: u8> REV12_W<'a, O> {
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
}
#[doc = "Field `REV13` reader - Result Event for Result Register 13"]
pub type REV13_R = crate::BitReader<REV13_A>;
#[doc = "Result Event for Result Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV13_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV13_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV13_A::VALUE2
    }
}
#[doc = "Field `REV13` writer - Result Event for Result Register 13"]
pub type REV13_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV13_A, O>;
impl<'a, const O: u8> REV13_W<'a, O> {
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
}
#[doc = "Field `REV14` reader - Result Event for Result Register 14"]
pub type REV14_R = crate::BitReader<REV14_A>;
#[doc = "Result Event for Result Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV14_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV14_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV14_A::VALUE2
    }
}
#[doc = "Field `REV14` writer - Result Event for Result Register 14"]
pub type REV14_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV14_A, O>;
impl<'a, const O: u8> REV14_W<'a, O> {
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
}
#[doc = "Field `REV15` reader - Result Event for Result Register 15"]
pub type REV15_R = crate::BitReader<REV15_A>;
#[doc = "Result Event for Result Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl REV15_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REV15_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV15_A::VALUE2
    }
}
#[doc = "Field `REV15` writer - Result Event for Result Register 15"]
pub type REV15_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFLAG_SPEC, REV15_A, O>;
impl<'a, const O: u8> REV15_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline(always)]
    pub fn rev0(&self) -> REV0_R {
        REV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline(always)]
    pub fn rev1(&self) -> REV1_R {
        REV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline(always)]
    pub fn rev2(&self) -> REV2_R {
        REV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline(always)]
    pub fn rev3(&self) -> REV3_R {
        REV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline(always)]
    pub fn rev4(&self) -> REV4_R {
        REV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline(always)]
    pub fn rev5(&self) -> REV5_R {
        REV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline(always)]
    pub fn rev6(&self) -> REV6_R {
        REV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline(always)]
    pub fn rev7(&self) -> REV7_R {
        REV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline(always)]
    pub fn rev8(&self) -> REV8_R {
        REV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline(always)]
    pub fn rev9(&self) -> REV9_R {
        REV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline(always)]
    pub fn rev10(&self) -> REV10_R {
        REV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline(always)]
    pub fn rev11(&self) -> REV11_R {
        REV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline(always)]
    pub fn rev12(&self) -> REV12_R {
        REV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline(always)]
    pub fn rev13(&self) -> REV13_R {
        REV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline(always)]
    pub fn rev14(&self) -> REV14_R {
        REV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline(always)]
    pub fn rev15(&self) -> REV15_R {
        REV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn rev0(&mut self) -> REV0_W<0> {
        REV0_W::new(self)
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn rev1(&mut self) -> REV1_W<1> {
        REV1_W::new(self)
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn rev2(&mut self) -> REV2_W<2> {
        REV2_W::new(self)
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline(always)]
    #[must_use]
    pub fn rev3(&mut self) -> REV3_W<3> {
        REV3_W::new(self)
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline(always)]
    #[must_use]
    pub fn rev4(&mut self) -> REV4_W<4> {
        REV4_W::new(self)
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline(always)]
    #[must_use]
    pub fn rev5(&mut self) -> REV5_W<5> {
        REV5_W::new(self)
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline(always)]
    #[must_use]
    pub fn rev6(&mut self) -> REV6_W<6> {
        REV6_W::new(self)
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline(always)]
    #[must_use]
    pub fn rev7(&mut self) -> REV7_W<7> {
        REV7_W::new(self)
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline(always)]
    #[must_use]
    pub fn rev8(&mut self) -> REV8_W<8> {
        REV8_W::new(self)
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline(always)]
    #[must_use]
    pub fn rev9(&mut self) -> REV9_W<9> {
        REV9_W::new(self)
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline(always)]
    #[must_use]
    pub fn rev10(&mut self) -> REV10_W<10> {
        REV10_W::new(self)
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline(always)]
    #[must_use]
    pub fn rev11(&mut self) -> REV11_W<11> {
        REV11_W::new(self)
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline(always)]
    #[must_use]
    pub fn rev12(&mut self) -> REV12_W<12> {
        REV12_W::new(self)
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline(always)]
    #[must_use]
    pub fn rev13(&mut self) -> REV13_W<13> {
        REV13_W::new(self)
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline(always)]
    #[must_use]
    pub fn rev14(&mut self) -> REV14_W<14> {
        REV14_W::new(self)
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline(always)]
    #[must_use]
    pub fn rev15(&mut self) -> REV15_W<15> {
        REV15_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFLAG to value 0"]
impl crate::Resettable for REFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
