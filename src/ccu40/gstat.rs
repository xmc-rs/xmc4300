#[doc = "Register `GSTAT` reader"]
pub struct R(crate::R<GSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CC40 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S0I_A> for bool {
    #[inline(always)]
    fn from(variant: S0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0I` reader - CC40 IDLE status"]
pub struct S0I_R(crate::FieldReader<bool, S0I_A>);
impl S0I_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0I_A {
        match self.bits {
            false => S0I_A::VALUE1,
            true => S0I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S0I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S0I_A::VALUE2
    }
}
impl core::ops::Deref for S0I_R {
    type Target = crate::FieldReader<bool, S0I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CC41 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S1I_A> for bool {
    #[inline(always)]
    fn from(variant: S1I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1I` reader - CC41 IDLE status"]
pub struct S1I_R(crate::FieldReader<bool, S1I_A>);
impl S1I_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1I_A {
        match self.bits {
            false => S1I_A::VALUE1,
            true => S1I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S1I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S1I_A::VALUE2
    }
}
impl core::ops::Deref for S1I_R {
    type Target = crate::FieldReader<bool, S1I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CC42 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S2I_A> for bool {
    #[inline(always)]
    fn from(variant: S2I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2I` reader - CC42 IDLE status"]
pub struct S2I_R(crate::FieldReader<bool, S2I_A>);
impl S2I_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2I_A {
        match self.bits {
            false => S2I_A::VALUE1,
            true => S2I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S2I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S2I_A::VALUE2
    }
}
impl core::ops::Deref for S2I_R {
    type Target = crate::FieldReader<bool, S2I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CC43 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S3I_A> for bool {
    #[inline(always)]
    fn from(variant: S3I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3I` reader - CC43 IDLE status"]
pub struct S3I_R(crate::FieldReader<bool, S3I_A>);
impl S3I_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3I_A {
        match self.bits {
            false => S3I_A::VALUE1,
            true => S3I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S3I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S3I_A::VALUE2
    }
}
impl core::ops::Deref for S3I_R {
    type Target = crate::FieldReader<bool, S3I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Prescaler Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRB_A {
    #[doc = "0: Prescaler is stopped"]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running"]
    VALUE2 = 1,
}
impl From<PRB_A> for bool {
    #[inline(always)]
    fn from(variant: PRB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRB` reader - Prescaler Run Bit"]
pub struct PRB_R(crate::FieldReader<bool, PRB_A>);
impl PRB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRB_A {
        match self.bits {
            false => PRB_A::VALUE1,
            true => PRB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRB_A::VALUE2
    }
}
impl core::ops::Deref for PRB_R {
    type Target = crate::FieldReader<bool, PRB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CC40 IDLE status"]
    #[inline(always)]
    pub fn s0i(&self) -> S0I_R {
        S0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CC41 IDLE status"]
    #[inline(always)]
    pub fn s1i(&self) -> S1I_R {
        S1I_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CC42 IDLE status"]
    #[inline(always)]
    pub fn s2i(&self) -> S2I_R {
        S2I_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CC43 IDLE status"]
    #[inline(always)]
    pub fn s3i(&self) -> S3I_R {
        S3I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Prescaler Run Bit"]
    #[inline(always)]
    pub fn prb(&self) -> PRB_R {
        PRB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Global Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gstat](index.html) module"]
pub struct GSTAT_SPEC;
impl crate::RegisterSpec for GSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gstat::R](R) reader structure"]
impl crate::Readable for GSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GSTAT to value 0x0f"]
impl crate::Resettable for GSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
