#[doc = "Register `CURRENT_HOST_RECEIVE_DESCRIPTOR` reader"]
pub type R = crate::R<CurrentHostReceiveDescriptorSpec>;
#[doc = "Field `CURRDESAPTR` reader - Host Receive Descriptor Address Pointer"]
pub type CurrdesaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CurrdesaptrR {
        CurrdesaptrR::new(self.bits)
    }
}
#[doc = "Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_receive_descriptor::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentHostReceiveDescriptorSpec;
impl crate::RegisterSpec for CurrentHostReceiveDescriptorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_host_receive_descriptor::R`](R) reader structure"]
impl crate::Readable for CurrentHostReceiveDescriptorSpec {}
#[doc = "`reset()` method sets CURRENT_HOST_RECEIVE_DESCRIPTOR to value 0"]
impl crate::Resettable for CurrentHostReceiveDescriptorSpec {
    const RESET_VALUE: u32 = 0;
}
