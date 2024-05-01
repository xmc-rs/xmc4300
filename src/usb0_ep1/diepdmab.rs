#[doc = "Register `DIEPDMAB` reader"]
pub type R = crate::R<DiepdmabSpec>;
#[doc = "Field `DMABufferAddr` reader - DMA Buffer Address"]
pub type DmabufferAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DmabufferAddrR {
        DmabufferAddrR::new(self.bits)
    }
}
#[doc = "Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepdmabSpec;
impl crate::RegisterSpec for DiepdmabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab::R`](R) reader structure"]
impl crate::Readable for DiepdmabSpec {}
#[doc = "`reset()` method sets DIEPDMAB to value 0"]
impl crate::Resettable for DiepdmabSpec {
    const RESET_VALUE: u32 = 0;
}
