#[doc = "Register `DIEPDMAB` reader"]
pub type R = crate::R<DIEPDMAB_SPEC>;
#[doc = "Field `DMABufferAddr` reader - DMA Buffer Address"]
pub type DMABUFFER_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DMABUFFER_ADDR_R {
        DMABUFFER_ADDR_R::new(self.bits)
    }
}
#[doc = "Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMAB_SPEC;
impl crate::RegisterSpec for DIEPDMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab::R`](R) reader structure"]
impl crate::Readable for DIEPDMAB_SPEC {}
#[doc = "`reset()` method sets DIEPDMAB to value 0"]
impl crate::Resettable for DIEPDMAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
