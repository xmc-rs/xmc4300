#[doc = "Reader of register RX_WATCHDOG_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RX_WATCHDOG_ERROR_FRAMES>;
#[doc = "Reader of field `RXWDGERR`"]
pub type RXWDGERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
    #[inline(always)]
    pub fn rxwdgerr(&self) -> RXWDGERR_R {
        RXWDGERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
