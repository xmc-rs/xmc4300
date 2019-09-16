#[doc = "Reader of register RXICMP_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RXICMP_ERROR_FRAMES>;
#[doc = "Reader of field `RXICMPERRFRM`"]
pub type RXICMPERRFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose ICMP payload has a checksum error."]
    #[inline(always)]
    pub fn rxicmperrfrm(&self) -> RXICMPERRFRM_R {
        RXICMPERRFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
