#[doc = "Register `CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS` reader"]
pub type R = crate::R<CurrentHostTransmitBufferAddressSpec>;
#[doc = "Field `CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer"]
pub type CurtbufaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CurtbufaptrR {
        CurtbufaptrR::new(self.bits)
    }
}
#[doc = "Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_transmit_buffer_address::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentHostTransmitBufferAddressSpec;
impl crate::RegisterSpec for CurrentHostTransmitBufferAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_host_transmit_buffer_address::R`](R) reader structure"]
impl crate::Readable for CurrentHostTransmitBufferAddressSpec {}
#[doc = "`reset()` method sets CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS to value 0"]
impl crate::Resettable for CurrentHostTransmitBufferAddressSpec {
    const RESET_VALUE: u32 = 0;
}
