#[doc = "Reader of register RXIPV6_NO_PAYLOAD_OCTETS"]
pub type R = crate::R<u32, super::RXIPV6_NO_PAYLOAD_OCTETS>;
#[doc = "Reader of field `RXIPV6NOPAYOCT`"]
pub type RXIPV6NOPAYOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv6nopayoct(&self) -> RXIPV6NOPAYOCT_R {
        RXIPV6NOPAYOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
