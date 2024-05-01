#[doc = "Register `RX_RECEIVE_ERROR_FRAMES` reader"]
pub type R = crate::R<RxReceiveErrorFramesSpec>;
#[doc = "Field `RXRCVERR` reader - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
pub type RxrcverrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
    #[inline(always)]
    pub fn rxrcverr(&self) -> RxrcverrR {
        RxrcverrR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Receive Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_receive_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxReceiveErrorFramesSpec;
impl crate::RegisterSpec for RxReceiveErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_receive_error_frames::R`](R) reader structure"]
impl crate::Readable for RxReceiveErrorFramesSpec {}
#[doc = "`reset()` method sets RX_RECEIVE_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxReceiveErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
