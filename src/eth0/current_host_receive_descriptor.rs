#[doc = "Register `CURRENT_HOST_RECEIVE_DESCRIPTOR` reader"]
pub struct R(crate::R<CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Host Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[doc = "Current Host Receive Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [current_host_receive_descriptor](index.html) module"]
pub struct CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC;
impl crate::RegisterSpec for CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [current_host_receive_descriptor::R](R) reader structure"]
impl crate::Readable for CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURRENT_HOST_RECEIVE_DESCRIPTOR to value 0"]
impl crate::Resettable for CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
