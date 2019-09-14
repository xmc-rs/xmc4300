#[doc = "Reader of register RXIPV6_NO_PAYLOAD_FRAMES"]
pub type R = crate::R<u32, super::RXIPV6_NO_PAYLOAD_FRAMES>;
#[doc = "Reader of field `RXIPV6NOPAYFRM`"]
pub type RXIPV6NOPAYFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers."]
    #[inline(always)]
    pub fn rxipv6nopayfrm(&self) -> RXIPV6NOPAYFRM_R {
        RXIPV6NOPAYFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
