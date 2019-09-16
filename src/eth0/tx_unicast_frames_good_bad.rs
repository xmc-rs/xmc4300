#[doc = "Reader of register TX_UNICAST_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_UNICAST_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TXUCASTGB`"]
pub type TXUCASTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad unicast frames."]
    #[inline(always)]
    pub fn txucastgb(&self) -> TXUCASTGB_R {
        TXUCASTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
