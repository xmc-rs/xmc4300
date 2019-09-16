#[doc = "Reader of register RXIPV4_FRAGMENTED_FRAMES"]
pub type R = crate::R<u32, super::RXIPV4_FRAGMENTED_FRAMES>;
#[doc = "Reader of field `RXIPV4FRAGFRM`"]
pub type RXIPV4FRAGFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IPv4 datagrams received with fragmentation."]
    #[inline(always)]
    pub fn rxipv4fragfrm(&self) -> RXIPV4FRAGFRM_R {
        RXIPV4FRAGFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
