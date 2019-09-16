#[doc = "Reader of register TX_DEFERRED_FRAMES"]
pub type R = crate::R<u32, super::TX_DEFERRED_FRAMES>;
#[doc = "Reader of field `TXDEFRD`"]
pub type TXDEFRD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a deferral in the half-duplex mode."]
    #[inline(always)]
    pub fn txdefrd(&self) -> TXDEFRD_R {
        TXDEFRD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
