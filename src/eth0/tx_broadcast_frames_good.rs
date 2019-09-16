#[doc = "Reader of register TX_BROADCAST_FRAMES_GOOD"]
pub type R = crate::R<u32, super::TX_BROADCAST_FRAMES_GOOD>;
#[doc = "Reader of field `TXBCASTG`"]
pub type TXBCASTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good broadcast frames."]
    #[inline(always)]
    pub fn txbcastg(&self) -> TXBCASTG_R {
        TXBCASTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
