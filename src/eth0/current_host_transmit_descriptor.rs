#[doc = "Register `CURRENT_HOST_TRANSMIT_DESCRIPTOR` reader"]
pub type R = crate::R<CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>;
#[doc = "Field `CURTDESAPTR` reader - Host Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
#[doc = "Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`current_host_transmit_descriptor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC;
impl crate::RegisterSpec for CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_host_transmit_descriptor::R`](R) reader structure"]
impl crate::Readable for CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC {}
#[doc = "`reset()` method sets CURRENT_HOST_TRANSMIT_DESCRIPTOR to value 0"]
impl crate::Resettable for CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
