#[doc = "Register `RX_WATCHDOG_ERROR_FRAMES` reader"]
pub type R = crate::R<RxWatchdogErrorFramesSpec>;
#[doc = "Field `RXWDGERR` reader - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
pub type RxwdgerrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
    #[inline(always)]
    pub fn rxwdgerr(&self) -> RxwdgerrR {
        RxwdgerrR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Watchdog Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_watchdog_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxWatchdogErrorFramesSpec;
impl crate::RegisterSpec for RxWatchdogErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_watchdog_error_frames::R`](R) reader structure"]
impl crate::Readable for RxWatchdogErrorFramesSpec {}
#[doc = "`reset()` method sets RX_WATCHDOG_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxWatchdogErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
