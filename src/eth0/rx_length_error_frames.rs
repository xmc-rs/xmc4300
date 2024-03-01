#[doc = "Register `RX_LENGTH_ERROR_FRAMES` reader"]
pub type R = crate::R<RxLengthErrorFramesSpec>;
#[doc = "Field `RXLENERR` reader - This field indicates the number of frames received with length error (Length type field not equal to frame size) for all frames with valid length field."]
pub type RxlenerrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length error (Length type field not equal to frame size) for all frames with valid length field."]
    #[inline(always)]
    pub fn rxlenerr(&self) -> RxlenerrR {
        RxlenerrR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Length Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_length_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxLengthErrorFramesSpec;
impl crate::RegisterSpec for RxLengthErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_length_error_frames::R`](R) reader structure"]
impl crate::Readable for RxLengthErrorFramesSpec {}
#[doc = "`reset()` method sets RX_LENGTH_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxLengthErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
