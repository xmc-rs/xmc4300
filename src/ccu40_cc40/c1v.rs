#[doc = "Register `C1V` reader"]
pub struct R(crate::R<C1V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTV` reader - Capture Value"]
pub struct CAPTV_R(crate::FieldReader<u16, u16>);
impl CAPTV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CAPTV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPCV` reader - Prescaler Value"]
pub struct FPCV_R(crate::FieldReader<u8, u8>);
impl FPCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FPCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFL_A {
    #[doc = "0: No new value was captured into the specific capture register"]
    VALUE1 = 0,
    #[doc = "1: A new value was captured into the specific register"]
    VALUE2 = 1,
}
impl From<FFL_A> for bool {
    #[inline(always)]
    fn from(variant: FFL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFL` reader - Full Flag"]
pub struct FFL_R(crate::FieldReader<bool, FFL_A>);
impl FFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFL_A {
        match self.bits {
            false => FFL_A::VALUE1,
            true => FFL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FFL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FFL_A::VALUE2
    }
}
impl core::ops::Deref for FFL_R {
    type Target = crate::FieldReader<bool, FFL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture Value"]
    #[inline(always)]
    pub fn captv(&self) -> CAPTV_R {
        CAPTV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Prescaler Value"]
    #[inline(always)]
    pub fn fpcv(&self) -> FPCV_R {
        FPCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Full Flag"]
    #[inline(always)]
    pub fn ffl(&self) -> FFL_R {
        FFL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
#[doc = "Capture Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1v](index.html) module"]
pub struct C1V_SPEC;
impl crate::RegisterSpec for C1V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1v::R](R) reader structure"]
impl crate::Readable for C1V_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C1V to value 0"]
impl crate::Resettable for C1V_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
