#[doc = "Register `RXIPV6_HEADER_ERROR_FRAMES` reader"]
pub type R = crate::R<RXIPV6_HEADER_ERROR_FRAMES_SPEC>;
#[doc = "Field `RXIPV6HDRERRFRM` reader - This field indicates the number of IPv6 datagrams received with header errors (length or version mismatch)."]
pub type RXIPV6HDRERRFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv6 datagrams received with header errors (length or version mismatch)."]
    #[inline(always)]
    pub fn rxipv6hdrerrfrm(&self) -> RXIPV6HDRERRFRM_R {
        RXIPV6HDRERRFRM_R::new(self.bits)
    }
}
#[doc = "Receive IPV6 Header Error Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_header_error_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV6_HEADER_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV6_HEADER_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_header_error_frames::R`](R) reader structure"]
impl crate::Readable for RXIPV6_HEADER_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXIPV6_HEADER_ERROR_FRAMES to value 0"]
impl crate::Resettable for RXIPV6_HEADER_ERROR_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
