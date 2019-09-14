#[doc = "Reader of register RX_PAUSE_FRAMES"]
pub type R = crate::R<u32, super::RX_PAUSE_FRAMES>;
#[doc = "Reader of field `RXPAUSEFRM`"]
pub type RXPAUSEFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and valid PAUSE frames."]
    #[inline(always)]
    pub fn rxpausefrm(&self) -> RXPAUSEFRM_R {
        RXPAUSEFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
