#[doc = "Register `RXIPV6_HEADER_ERROR_OCTETS` reader"]
pub type R = crate::R<RXIPV6_HEADER_ERROR_OCTETS_SPEC>;
#[doc = "Field `RXIPV6HDRERROCT` reader - This field indicates the number of bytes received in IPv6 datagrams with header errors (length or version mismatch). The value in the IPv6 headers Length field is used to update this counter."]
pub type RXIPV6HDRERROCT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in IPv6 datagrams with header errors (length or version mismatch). The value in the IPv6 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv6hdrerroct(&self) -> RXIPV6HDRERROCT_R {
        RXIPV6HDRERROCT_R::new(self.bits)
    }
}
#[doc = "Receive IPV6 Header Error Octet Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_header_error_octets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV6_HEADER_ERROR_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV6_HEADER_ERROR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_header_error_octets::R`](R) reader structure"]
impl crate::Readable for RXIPV6_HEADER_ERROR_OCTETS_SPEC {}
#[doc = "`reset()` method sets RXIPV6_HEADER_ERROR_OCTETS to value 0"]
impl crate::Resettable for RXIPV6_HEADER_ERROR_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
