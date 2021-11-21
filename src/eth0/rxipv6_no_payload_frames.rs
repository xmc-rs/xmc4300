#[doc = "Register `RXIPV6_NO_PAYLOAD_FRAMES` reader"]
pub struct R(crate::R<RXIPV6_NO_PAYLOAD_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV6_NO_PAYLOAD_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV6_NO_PAYLOAD_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV6_NO_PAYLOAD_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV6NOPAYFRM` reader - This field indicates the number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers."]
pub struct RXIPV6NOPAYFRM_R(crate::FieldReader<u32, u32>);
impl RXIPV6NOPAYFRM_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXIPV6NOPAYFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6NOPAYFRM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers."]
    #[inline(always)]
    pub fn rxipv6nopayfrm(&self) -> RXIPV6NOPAYFRM_R {
        RXIPV6NOPAYFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive IPV6 No Payload Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv6_no_payload_frames](index.html) module"]
pub struct RXIPV6_NO_PAYLOAD_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV6_NO_PAYLOAD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv6_no_payload_frames::R](R) reader structure"]
impl crate::Readable for RXIPV6_NO_PAYLOAD_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXIPV6_NO_PAYLOAD_FRAMES to value 0"]
impl crate::Resettable for RXIPV6_NO_PAYLOAD_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
