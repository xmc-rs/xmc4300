#[doc = "Register `RXIPV4_HEADER_ERROR_OCTETS` reader"]
pub struct R(crate::R<RXIPV4_HEADER_ERROR_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV4_HEADER_ERROR_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV4_HEADER_ERROR_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV4_HEADER_ERROR_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV4HDRERROCT` reader - This field indicates the number of bytes received in the IPv4 datagrams with header errors (checksum, length, or version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
pub struct RXIPV4HDRERROCT_R(crate::FieldReader<u32, u32>);
impl RXIPV4HDRERROCT_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXIPV4HDRERROCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4HDRERROCT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in the IPv4 datagrams with header errors (checksum, length, or version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4hdrerroct(&self) -> RXIPV4HDRERROCT_R {
        RXIPV4HDRERROCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive IPV4 Header Error Octet Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv4_header_error_octets](index.html) module"]
pub struct RXIPV4_HEADER_ERROR_OCTETS_SPEC;
impl crate::RegisterSpec for RXIPV4_HEADER_ERROR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv4_header_error_octets::R](R) reader structure"]
impl crate::Readable for RXIPV4_HEADER_ERROR_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXIPV4_HEADER_ERROR_OCTETS to value 0"]
impl crate::Resettable for RXIPV4_HEADER_ERROR_OCTETS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
