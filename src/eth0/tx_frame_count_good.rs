#[doc = "Reader of register TX_FRAME_COUNT_GOOD"]
pub type R = crate::R<u32, super::TX_FRAME_COUNT_GOOD>;
#[doc = "Reader of field `TXFRMG`"]
pub type TXFRMG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good frames, exclusive of preamble."]
    #[inline(always)]
    pub fn txfrmg(&self) -> TXFRMG_R {
        TXFRMG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
