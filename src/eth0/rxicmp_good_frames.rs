#[doc = "Reader of register RXICMP_GOOD_FRAMES"]
pub type R = crate::R<u32, super::RXICMP_GOOD_FRAMES>;
#[doc = "Reader of field `RXICMPGDFRM`"]
pub type RXICMPGDFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good ICMP payload."]
    #[inline(always)]
    pub fn rxicmpgdfrm(&self) -> RXICMPGDFRM_R {
        RXICMPGDFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
