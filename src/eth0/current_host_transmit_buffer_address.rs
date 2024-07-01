#[doc = "Register `CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS` reader"]
pub type R = crate::R<CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>;
#[doc = "Field `CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`current_host_transmit_buffer_address::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC;
impl crate::RegisterSpec for CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_host_transmit_buffer_address::R`](R) reader structure"]
impl crate::Readable for CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC {}
#[doc = "`reset()` method sets CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS to value 0"]
impl crate::Resettable for CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC {
    const RESET_VALUE: u32 = 0;
}
