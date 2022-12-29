#[doc = "Register `RES` reader"]
pub struct R(crate::R<RES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RES` reader - Result Register"]
pub type RES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Result Register"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
#[doc = "CRC Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res](index.html) module"]
pub struct RES_SPEC;
impl crate::RegisterSpec for RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res::R](R) reader structure"]
impl crate::Readable for RES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RES to value 0xffff_ffff"]
impl crate::Resettable for RES_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
