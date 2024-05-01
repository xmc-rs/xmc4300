#[doc = "Register `HCDMAB` reader"]
pub type R = crate::R<HcdmabSpec>;
#[doc = "Field `Buffer_Address` reader - Buffer Address"]
pub type BufferAddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buffer_address(&self) -> BufferAddressR {
        BufferAddressR::new(self.bits)
    }
}
#[doc = "Host Channel DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcdmabSpec;
impl crate::RegisterSpec for HcdmabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab::R`](R) reader structure"]
impl crate::Readable for HcdmabSpec {}
#[doc = "`reset()` method sets HCDMAB to value 0"]
impl crate::Resettable for HcdmabSpec {
    const RESET_VALUE: u32 = 0;
}
