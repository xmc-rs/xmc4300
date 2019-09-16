#[doc = "Reader of register TX_PAUSE_FRAMES"]
pub type R = crate::R<u32, super::TX_PAUSE_FRAMES>;
#[doc = "Reader of field `TXPAUSE`"]
pub type TXPAUSE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good PAUSE frames."]
    #[inline(always)]
    pub fn txpause(&self) -> TXPAUSE_R {
        TXPAUSE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
