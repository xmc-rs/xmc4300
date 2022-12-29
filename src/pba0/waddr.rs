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
pub type WADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write Error Address"]
    #[inline(always)]
    pub fn waddr(&self) -> WADDR_R {
        WADDR_R::new(self.bits)
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
    const RESET_VALUE: Self::Ux = 0;
}
