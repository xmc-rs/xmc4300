#[doc = "Reader of register RXIPV4_NO_PAYLOAD_FRAMES"]
pub type R = crate::R<u32, super::RXIPV4_NO_PAYLOAD_FRAMES>;
#[doc = "Reader of field `RXIPV4NOPAYFRM`"]
pub type RXIPV4NOPAYFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine."]
    #[inline(always)]
    pub fn rxipv4nopayfrm(&self) -> RXIPV4NOPAYFRM_R {
        RXIPV4NOPAYFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
