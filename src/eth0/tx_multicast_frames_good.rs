#[doc = "Reader of register TX_MULTICAST_FRAMES_GOOD"]
pub type R = crate::R<u32, super::TX_MULTICAST_FRAMES_GOOD>;
#[doc = "Reader of field `TXMCASTG`"]
pub type TXMCASTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good multicast frames."]
    #[inline(always)]
    pub fn txmcastg(&self) -> TXMCASTG_R {
        TXMCASTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
