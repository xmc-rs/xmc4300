#[doc = "Register `DOEPDMAB` reader"]
pub type R = crate::R<DoepdmabSpec>;
#[doc = "Field `DMABufferAddr` reader - DMA Buffer Address"]
pub type DmabufferAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DmabufferAddrR {
        DmabufferAddrR::new(self.bits)
    }
}
#[doc = "Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepdmabSpec;
impl crate::RegisterSpec for DoepdmabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab::R`](R) reader structure"]
impl crate::Readable for DoepdmabSpec {}
#[doc = "`reset()` method sets DOEPDMAB to value 0"]
impl crate::Resettable for DoepdmabSpec {
    const RESET_VALUE: u32 = 0;
}
