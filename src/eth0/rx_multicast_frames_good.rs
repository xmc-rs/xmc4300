#[doc = "Reader of register RX_MULTICAST_FRAMES_GOOD"]
pub type R = crate::R<u32, super::RX_MULTICAST_FRAMES_GOOD>;
#[doc = "Reader of field `RXMCASTG`"]
pub type RXMCASTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good multicast frames."]
    #[inline(always)]
    pub fn rxmcastg(&self) -> RXMCASTG_R {
        RXMCASTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
