#[doc = "Register `RXIPV6_NO_PAYLOAD_FRAMES` reader"]
pub type R = crate::R<Rxipv6NoPayloadFramesSpec>;
#[doc = "Field `RXIPV6NOPAYFRM` reader - This field indicates the number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers."]
pub type Rxipv6nopayfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers."]
    #[inline(always)]
    pub fn rxipv6nopayfrm(&self) -> Rxipv6nopayfrmR {
        Rxipv6nopayfrmR::new(self.bits)
    }
}
#[doc = "Receive IPV6 No Payload Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_no_payload_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv6NoPayloadFramesSpec;
impl crate::RegisterSpec for Rxipv6NoPayloadFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_no_payload_frames::R`](R) reader structure"]
impl crate::Readable for Rxipv6NoPayloadFramesSpec {}
#[doc = "`reset()` method sets RXIPV6_NO_PAYLOAD_FRAMES to value 0"]
impl crate::Resettable for Rxipv6NoPayloadFramesSpec {
    const RESET_VALUE: u32 = 0;
}
