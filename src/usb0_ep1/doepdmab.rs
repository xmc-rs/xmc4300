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
pub type DMABUFFER_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DMABUFFER_ADDR_R {
        DMABUFFER_ADDR_R::new(self.bits)
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
    const RESET_VALUE: Self::Ux = 0;
}
