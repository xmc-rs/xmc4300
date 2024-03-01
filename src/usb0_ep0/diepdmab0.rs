#[doc = "Register `DIEPDMAB0` reader"]
pub type R = crate::R<Diepdmab0Spec>;
#[doc = "Field `DMABufferAddr` reader - DMA Buffer Address"]
pub type DmabufferAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DmabufferAddrR {
        DmabufferAddrR::new(self.bits)
    }
}
#[doc = "Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepdmab0Spec;
impl crate::RegisterSpec for Diepdmab0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab0::R`](R) reader structure"]
impl crate::Readable for Diepdmab0Spec {}
#[doc = "`reset()` method sets DIEPDMAB0 to value 0"]
impl crate::Resettable for Diepdmab0Spec {
    const RESET_VALUE: u32 = 0;
}
