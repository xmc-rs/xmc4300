#[doc = "Reader of register RXIPV6_GOOD_FRAMES"]
pub type R = crate::R<u32, super::RXIPV6_GOOD_FRAMES>;
#[doc = "Reader of field `RXIPV6GDFRM`"]
pub type RXIPV6GDFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads."]
    #[inline(always)]
    pub fn rxipv6gdfrm(&self) -> RXIPV6GDFRM_R {
        RXIPV6GDFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
