#[doc = "Reader of register TX_UNDERFLOW_ERROR_FRAMES"]
pub type R = crate::R<u32, super::TX_UNDERFLOW_ERROR_FRAMES>;
#[doc = "Reader of field `TXUNDRFLW`"]
pub type TXUNDRFLW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of frame underflow error."]
    #[inline(always)]
    pub fn txundrflw(&self) -> TXUNDRFLW_R {
        TXUNDRFLW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
