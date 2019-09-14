#[doc = "Reader of register TX_LATE_COLLISION_FRAMES"]
pub type R = crate::R<u32, super::TX_LATE_COLLISION_FRAMES>;
#[doc = "Reader of field `TXLATECOL`"]
pub type TXLATECOL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of late collision error."]
    #[inline(always)]
    pub fn txlatecol(&self) -> TXLATECOL_R {
        TXLATECOL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
