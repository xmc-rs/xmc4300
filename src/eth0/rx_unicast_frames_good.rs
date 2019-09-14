#[doc = "Reader of register RX_UNICAST_FRAMES_GOOD"]
pub type R = crate::R<u32, super::RX_UNICAST_FRAMES_GOOD>;
#[doc = "Reader of field `RXUCASTG`"]
pub type RXUCASTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good unicast frames."]
    #[inline(always)]
    pub fn rxucastg(&self) -> RXUCASTG_R {
        RXUCASTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
