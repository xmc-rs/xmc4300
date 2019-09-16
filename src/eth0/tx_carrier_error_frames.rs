#[doc = "Reader of register TX_CARRIER_ERROR_FRAMES"]
pub type R = crate::R<u32, super::TX_CARRIER_ERROR_FRAMES>;
#[doc = "Reader of field `TXCARR`"]
pub type TXCARR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
    #[inline(always)]
    pub fn txcarr(&self) -> TXCARR_R {
        TXCARR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
