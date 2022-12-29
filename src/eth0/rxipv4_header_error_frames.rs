#[doc = "Register `RXIPV4_HEADER_ERROR_FRAMES` reader"]
pub struct R(crate::R<RXIPV4_HEADER_ERROR_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV4_HEADER_ERROR_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV4_HEADER_ERROR_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV4_HEADER_ERROR_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV4HDRERRFRM` reader - This field indicates the number of IPv4 datagrams received with header errors (checksum, length, or version mismatch)."]
pub type RXIPV4HDRERRFRM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv4 datagrams received with header errors (checksum, length, or version mismatch)."]
    #[inline(always)]
    pub fn rxipv4hdrerrfrm(&self) -> RXIPV4HDRERRFRM_R {
        RXIPV4HDRERRFRM_R::new(self.bits)
    }
}
#[doc = "Receive IPV4 Header Error Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv4_header_error_frames](index.html) module"]
pub struct RXIPV4_HEADER_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV4_HEADER_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv4_header_error_frames::R](R) reader structure"]
impl crate::Readable for RXIPV4_HEADER_ERROR_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXIPV4_HEADER_ERROR_FRAMES to value 0"]
impl crate::Resettable for RXIPV4_HEADER_ERROR_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
