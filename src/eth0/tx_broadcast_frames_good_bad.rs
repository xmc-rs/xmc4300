#[doc = "Reader of register TX_BROADCAST_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_BROADCAST_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TXBCASTGB`"]
pub type TXBCASTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad broadcast frames."]
    #[inline(always)]
    pub fn txbcastgb(&self) -> TXBCASTGB_R {
        TXBCASTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
