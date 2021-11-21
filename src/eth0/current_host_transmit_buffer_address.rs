#[doc = "Register `CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS` reader"]
pub struct R(crate::R<CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer"]
pub struct CURTBUFAPTR_R(crate::FieldReader<u32, u32>);
impl CURTBUFAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURTBUFAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURTBUFAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Current Host Transmit Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [current_host_transmit_buffer_address](index.html) module"]
pub struct CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC;
impl crate::RegisterSpec for CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [current_host_transmit_buffer_address::R](R) reader structure"]
impl crate::Readable for CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS to value 0"]
impl crate::Resettable for CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
