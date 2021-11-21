#[doc = "Register `WADDR` reader"]
pub struct R(crate::R<WADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WADDR` reader - Write Error Address"]
pub struct WADDR_R(crate::FieldReader<u32, u32>);
impl WADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Write Error Address"]
    #[inline(always)]
    pub fn waddr(&self) -> WADDR_R {
        WADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "PBA Write Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waddr](index.html) module"]
pub struct WADDR_SPEC;
impl crate::RegisterSpec for WADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waddr::R](R) reader structure"]
impl crate::Readable for WADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WADDR to value 0"]
impl crate::Resettable for WADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
