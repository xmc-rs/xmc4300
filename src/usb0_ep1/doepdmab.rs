#[doc = "Register `DOEPDMAB` reader"]
pub type R = crate::R<DOEPDMAB_SPEC>;
#[doc = "Field `DMABufferAddr` reader - DMA Buffer Address"]
pub type DMABUFFER_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DMABUFFER_ADDR_R {
        DMABUFFER_ADDR_R::new(self.bits)
    }
}
#[doc = "Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB_SPEC;
impl crate::RegisterSpec for DOEPDMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB_SPEC {}
#[doc = "`reset()` method sets DOEPDMAB to value 0"]
impl crate::Resettable for DOEPDMAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
