#[doc = "Register `CURRENT_HOST_RECEIVE_BUFFER_ADDRESS` reader"]
pub type R = crate::R<CurrentHostReceiveBufferAddressSpec>;
#[doc = "Field `CURRBUFAPTR` reader - Host Receive Buffer Address Pointer"]
pub type CurrbufaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CurrbufaptrR {
        CurrbufaptrR::new(self.bits)
    }
}
#[doc = "Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_receive_buffer_address::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentHostReceiveBufferAddressSpec;
impl crate::RegisterSpec for CurrentHostReceiveBufferAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_host_receive_buffer_address::R`](R) reader structure"]
impl crate::Readable for CurrentHostReceiveBufferAddressSpec {}
#[doc = "`reset()` method sets CURRENT_HOST_RECEIVE_BUFFER_ADDRESS to value 0"]
impl crate::Resettable for CurrentHostReceiveBufferAddressSpec {
    const RESET_VALUE: u32 = 0;
}
