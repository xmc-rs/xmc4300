#[doc = "Reader of register TX_EXCESSIVE_COLLISION_FRAMES"]
pub type R = crate::R<u32, super::TX_EXCESSIVE_COLLISION_FRAMES>;
#[doc = "Reader of field `TXEXSCOL`"]
pub type TXEXSCOL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of excessive (16) collision error."]
    #[inline(always)]
    pub fn txexscol(&self) -> TXEXSCOL_R {
        TXEXSCOL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
