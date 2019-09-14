#[doc = "Reader of register RXUDP_GOOD_FRAMES"]
pub type R = crate::R<u32, super::RXUDP_GOOD_FRAMES>;
#[doc = "Reader of field `RXUDPGDFRM`"]
pub type RXUDPGDFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good UDP payload. This counter is not updated when the counter is incremented."]
    #[inline(always)]
    pub fn rxudpgdfrm(&self) -> RXUDPGDFRM_R {
        RXUDPGDFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
