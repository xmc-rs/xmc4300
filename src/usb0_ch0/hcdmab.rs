#[doc = "Register `HCDMAB` reader"]
pub type R = crate::R<HCDMAB_SPEC>;
#[doc = "Field `Buffer_Address` reader - Buffer Address"]
pub type BUFFER_ADDRESS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buffer_address(&self) -> BUFFER_ADDRESS_R {
        BUFFER_ADDRESS_R::new(self.bits)
    }
}
#[doc = "Host Channel DMA Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMAB_SPEC;
impl crate::RegisterSpec for HCDMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab::R`](R) reader structure"]
impl crate::Readable for HCDMAB_SPEC {}
#[doc = "`reset()` method sets HCDMAB to value 0"]
impl crate::Resettable for HCDMAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
