#[doc = "Reader of register TX_MULTICAST_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_MULTICAST_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TXMCASTGB`"]
pub type TXMCASTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad multicast frames."]
    #[inline(always)]
    pub fn txmcastgb(&self) -> TXMCASTGB_R {
        TXMCASTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
