#[doc = "Register `CURRENT_HOST_TRANSMIT_DESCRIPTOR` reader"]
pub struct R(crate::R<CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURTDESAPTR` reader - Host Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
#[doc = "Current Host Transmit Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [current_host_transmit_descriptor](index.html) module"]
pub struct CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC;
impl crate::RegisterSpec for CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [current_host_transmit_descriptor::R](R) reader structure"]
impl crate::Readable for CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURRENT_HOST_TRANSMIT_DESCRIPTOR to value 0"]
impl crate::Resettable for CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
