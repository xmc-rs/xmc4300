#[doc = "Register `RX_LENGTH_ERROR_FRAMES` reader"]
pub type R = crate::R<RX_LENGTH_ERROR_FRAMES_SPEC>;
#[doc = "Field `RXLENERR` reader - This field indicates the number of frames received with length error (Length type field not equal to frame size) for all frames with valid length field."]
pub type RXLENERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length error (Length type field not equal to frame size) for all frames with valid length field."]
    #[inline(always)]
    pub fn rxlenerr(&self) -> RXLENERR_R {
        RXLENERR_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Length Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_length_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_LENGTH_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RX_LENGTH_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_length_error_frames::R`](R) reader structure"]
impl crate::Readable for RX_LENGTH_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets RX_LENGTH_ERROR_FRAMES to value 0"]
impl crate::Resettable for RX_LENGTH_ERROR_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
