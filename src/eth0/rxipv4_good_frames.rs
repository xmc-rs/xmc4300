#[doc = "Reader of register RXIPV4_GOOD_FRAMES"]
pub type R = crate::R<u32, super::RXIPV4_GOOD_FRAMES>;
#[doc = "Reader of field `RXIPV4GDFRM`"]
pub type RXIPV4GDFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
    #[inline(always)]
    pub fn rxipv4gdfrm(&self) -> RXIPV4GDFRM_R {
        RXIPV4GDFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
