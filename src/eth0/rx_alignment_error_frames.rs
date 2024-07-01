#[doc = "Register `RX_ALIGNMENT_ERROR_FRAMES` reader"]
pub type R = crate::R<RX_ALIGNMENT_ERROR_FRAMES_SPEC>;
#[doc = "Field `RXALGNERR` reader - This field indicates the number of frames received with alignment (dribble) error."]
pub type RXALGNERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with alignment (dribble) error."]
    #[inline(always)]
    pub fn rxalgnerr(&self) -> RXALGNERR_R {
        RXALGNERR_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Alignment Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ALIGNMENT_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RX_ALIGNMENT_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_alignment_error_frames::R`](R) reader structure"]
impl crate::Readable for RX_ALIGNMENT_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets RX_ALIGNMENT_ERROR_FRAMES to value 0"]
impl crate::Resettable for RX_ALIGNMENT_ERROR_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
