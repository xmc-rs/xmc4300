#[doc = "Register `GCST` reader"]
pub struct R(crate::R<GCST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Slice 0 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0SS_A> for bool {
    #[inline(always)]
    fn from(variant: S0SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0SS` reader - Slice 0 shadow transfer status"]
pub struct S0SS_R(crate::FieldReader<bool, S0SS_A>);
impl S0SS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0SS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0SS_A {
        match self.bits {
            false => S0SS_A::VALUE1,
            true => S0SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S0SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S0SS_A::VALUE2
    }
}
impl core::ops::Deref for S0SS_R {
    type Target = crate::FieldReader<bool, S0SS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 0 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S0DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0DSS` reader - Slice 0 Dither shadow transfer status"]
pub struct S0DSS_R(crate::FieldReader<bool, S0DSS_A>);
impl S0DSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0DSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0DSS_A {
        match self.bits {
            false => S0DSS_A::VALUE1,
            true => S0DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S0DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S0DSS_A::VALUE2
    }
}
impl core::ops::Deref for S0DSS_R {
    type Target = crate::FieldReader<bool, S0DSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 0 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S0PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0PSS` reader - Slice 0 Prescaler shadow transfer status"]
pub struct S0PSS_R(crate::FieldReader<bool, S0PSS_A>);
impl S0PSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0PSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0PSS_A {
        match self.bits {
            false => S0PSS_A::VALUE1,
            true => S0PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S0PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S0PSS_A::VALUE2
    }
}
impl core::ops::Deref for S0PSS_R {
    type Target = crate::FieldReader<bool, S0PSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 1 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1SS_A> for bool {
    #[inline(always)]
    fn from(variant: S1SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1SS` reader - Slice 1 shadow transfer status"]
pub struct S1SS_R(crate::FieldReader<bool, S1SS_A>);
impl S1SS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1SS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1SS_A {
        match self.bits {
            false => S1SS_A::VALUE1,
            true => S1SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S1SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S1SS_A::VALUE2
    }
}
impl core::ops::Deref for S1SS_R {
    type Target = crate::FieldReader<bool, S1SS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 1 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S1DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1DSS` reader - Slice 1 Dither shadow transfer status"]
pub struct S1DSS_R(crate::FieldReader<bool, S1DSS_A>);
impl S1DSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1DSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1DSS_A {
        match self.bits {
            false => S1DSS_A::VALUE1,
            true => S1DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S1DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S1DSS_A::VALUE2
    }
}
impl core::ops::Deref for S1DSS_R {
    type Target = crate::FieldReader<bool, S1DSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 1 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S1PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1PSS` reader - Slice 1 Prescaler shadow transfer status"]
pub struct S1PSS_R(crate::FieldReader<bool, S1PSS_A>);
impl S1PSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1PSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1PSS_A {
        match self.bits {
            false => S1PSS_A::VALUE1,
            true => S1PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S1PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S1PSS_A::VALUE2
    }
}
impl core::ops::Deref for S1PSS_R {
    type Target = crate::FieldReader<bool, S1PSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 2 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2SS_A> for bool {
    #[inline(always)]
    fn from(variant: S2SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2SS` reader - Slice 2 shadow transfer status"]
pub struct S2SS_R(crate::FieldReader<bool, S2SS_A>);
impl S2SS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2SS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2SS_A {
        match self.bits {
            false => S2SS_A::VALUE1,
            true => S2SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S2SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S2SS_A::VALUE2
    }
}
impl core::ops::Deref for S2SS_R {
    type Target = crate::FieldReader<bool, S2SS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 2 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S2DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2DSS` reader - Slice 2 Dither shadow transfer status"]
pub struct S2DSS_R(crate::FieldReader<bool, S2DSS_A>);
impl S2DSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2DSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2DSS_A {
        match self.bits {
            false => S2DSS_A::VALUE1,
            true => S2DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S2DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S2DSS_A::VALUE2
    }
}
impl core::ops::Deref for S2DSS_R {
    type Target = crate::FieldReader<bool, S2DSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 2 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S2PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2PSS` reader - Slice 2 Prescaler shadow transfer status"]
pub struct S2PSS_R(crate::FieldReader<bool, S2PSS_A>);
impl S2PSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2PSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2PSS_A {
        match self.bits {
            false => S2PSS_A::VALUE1,
            true => S2PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S2PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S2PSS_A::VALUE2
    }
}
impl core::ops::Deref for S2PSS_R {
    type Target = crate::FieldReader<bool, S2PSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 3 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3SS_A> for bool {
    #[inline(always)]
    fn from(variant: S3SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3SS` reader - Slice 3 shadow transfer status"]
pub struct S3SS_R(crate::FieldReader<bool, S3SS_A>);
impl S3SS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3SS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3SS_A {
        match self.bits {
            false => S3SS_A::VALUE1,
            true => S3SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S3SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S3SS_A::VALUE2
    }
}
impl core::ops::Deref for S3SS_R {
    type Target = crate::FieldReader<bool, S3SS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 3 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S3DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3DSS` reader - Slice 3 Dither shadow transfer status"]
pub struct S3DSS_R(crate::FieldReader<bool, S3DSS_A>);
impl S3DSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3DSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3DSS_A {
        match self.bits {
            false => S3DSS_A::VALUE1,
            true => S3DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S3DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S3DSS_A::VALUE2
    }
}
impl core::ops::Deref for S3DSS_R {
    type Target = crate::FieldReader<bool, S3DSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice 3 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S3PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3PSS` reader - Slice 3 Prescaler shadow transfer status"]
pub struct S3PSS_R(crate::FieldReader<bool, S3PSS_A>);
impl S3PSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3PSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3PSS_A {
        match self.bits {
            false => S3PSS_A::VALUE1,
            true => S3PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S3PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S3PSS_A::VALUE2
    }
}
impl core::ops::Deref for S3PSS_R {
    type Target = crate::FieldReader<bool, S3PSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC40ST` reader - Slice 0 status bit"]
pub struct CC40ST_R(crate::FieldReader<bool, bool>);
impl CC40ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC40ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC40ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC41ST` reader - Slice 1 status bit"]
pub struct CC41ST_R(crate::FieldReader<bool, bool>);
impl CC41ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC41ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC41ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC42ST` reader - Slice 2 status bit"]
pub struct CC42ST_R(crate::FieldReader<bool, bool>);
impl CC42ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC42ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC42ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC43ST` reader - Slice 3 status bit"]
pub struct CC43ST_R(crate::FieldReader<bool, bool>);
impl CC43ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC43ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC43ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Slice 0 shadow transfer status"]
    #[inline(always)]
    pub fn s0ss(&self) -> S0SS_R {
        S0SS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s0dss(&self) -> S0DSS_R {
        S0DSS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s0pss(&self) -> S0PSS_R {
        S0PSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer status"]
    #[inline(always)]
    pub fn s1ss(&self) -> S1SS_R {
        S1SS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s1dss(&self) -> S1DSS_R {
        S1DSS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s1pss(&self) -> S1PSS_R {
        S1PSS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer status"]
    #[inline(always)]
    pub fn s2ss(&self) -> S2SS_R {
        S2SS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s2dss(&self) -> S2DSS_R {
        S2DSS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s2pss(&self) -> S2PSS_R {
        S2PSS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer status"]
    #[inline(always)]
    pub fn s3ss(&self) -> S3SS_R {
        S3SS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s3dss(&self) -> S3DSS_R {
        S3DSS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s3pss(&self) -> S3PSS_R {
        S3PSS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slice 0 status bit"]
    #[inline(always)]
    pub fn cc40st(&self) -> CC40ST_R {
        CC40ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slice 1 status bit"]
    #[inline(always)]
    pub fn cc41st(&self) -> CC41ST_R {
        CC41ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slice 2 status bit"]
    #[inline(always)]
    pub fn cc42st(&self) -> CC42ST_R {
        CC42ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Slice 3 status bit"]
    #[inline(always)]
    pub fn cc43st(&self) -> CC43ST_R {
        CC43ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "Global Channel Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcst](index.html) module"]
pub struct GCST_SPEC;
impl crate::RegisterSpec for GCST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcst::R](R) reader structure"]
impl crate::Readable for GCST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GCST to value 0"]
impl crate::Resettable for GCST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
