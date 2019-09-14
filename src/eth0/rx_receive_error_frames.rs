#[doc = "Reader of register RX_RECEIVE_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RX_RECEIVE_ERROR_FRAMES>;
#[doc = "Reader of field `RXRCVERR`"]
pub type RXRCVERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
    #[inline(always)]
    pub fn rxrcverr(&self) -> RXRCVERR_R {
        RXRCVERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
