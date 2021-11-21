#[doc = "Register `DOEPDMAB` reader"]
pub struct R(crate::R<DOEPDMAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMABufferAddr` reader - DMA Buffer Address"]
pub struct DMABUFFERADDR_R(crate::FieldReader<u32, u32>);
impl DMABUFFERADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMABUFFERADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMABUFFERADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DMABUFFERADDR_R {
        DMABUFFERADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Device Endpoint DMA Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdmab](index.html) module"]
pub struct DOEPDMAB_SPEC;
impl crate::RegisterSpec for DOEPDMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdmab::R](R) reader structure"]
impl crate::Readable for DOEPDMAB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DOEPDMAB to value 0"]
impl crate::Resettable for DOEPDMAB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
