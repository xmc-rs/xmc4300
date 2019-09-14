#[doc = "Reader of register RX_CONTROL_FRAMES_GOOD"]
pub type R = crate::R<u32, super::RX_CONTROL_FRAMES_GOOD>;
#[doc = "Reader of field `RXCTRLG`"]
pub type RXCTRLG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
    #[inline(always)]
    pub fn rxctrlg(&self) -> RXCTRLG_R {
        RXCTRLG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
