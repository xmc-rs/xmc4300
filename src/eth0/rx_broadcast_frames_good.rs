#[doc = "Reader of register RX_BROADCAST_FRAMES_GOOD"]
pub type R = crate::R<u32, super::RX_BROADCAST_FRAMES_GOOD>;
#[doc = "Reader of field `RXBCASTG`"]
pub type RXBCASTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good broadcast frames."]
    #[inline(always)]
    pub fn rxbcastg(&self) -> RXBCASTG_R {
        RXBCASTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
