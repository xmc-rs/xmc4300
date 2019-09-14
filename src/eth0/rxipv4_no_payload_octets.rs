#[doc = "Reader of register RXIPV4_NO_PAYLOAD_OCTETS"]
pub type R = crate::R<u32, super::RXIPV4_NO_PAYLOAD_OCTETS>;
#[doc = "Reader of field `RXIPV4NOPAYOCT`"]
pub type RXIPV4NOPAYOCT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4nopayoct(&self) -> RXIPV4NOPAYOCT_R {
        RXIPV4NOPAYOCT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
