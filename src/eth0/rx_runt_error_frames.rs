#[doc = "Register `RX_RUNT_ERROR_FRAMES` reader"]
pub type R = crate::R<RxRuntErrorFramesSpec>;
#[doc = "Field `RXRUNTERR` reader - This field indicates the number of frames received with runt error(&lt;64 bytes and CRC error)."]
pub type RxrunterrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with runt error(&lt;64 bytes and CRC error)."]
    #[inline(always)]
    pub fn rxrunterr(&self) -> RxrunterrR {
        RxrunterrR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Runt Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_runt_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxRuntErrorFramesSpec;
impl crate::RegisterSpec for RxRuntErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_runt_error_frames::R`](R) reader structure"]
impl crate::Readable for RxRuntErrorFramesSpec {}
#[doc = "`reset()` method sets RX_RUNT_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxRuntErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
