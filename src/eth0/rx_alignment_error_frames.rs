#[doc = "Register `RX_ALIGNMENT_ERROR_FRAMES` reader"]
pub type R = crate::R<RxAlignmentErrorFramesSpec>;
#[doc = "Field `RXALGNERR` reader - This field indicates the number of frames received with alignment (dribble) error."]
pub type RxalgnerrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with alignment (dribble) error."]
    #[inline(always)]
    pub fn rxalgnerr(&self) -> RxalgnerrR {
        RxalgnerrR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Alignment Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_alignment_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxAlignmentErrorFramesSpec;
impl crate::RegisterSpec for RxAlignmentErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_alignment_error_frames::R`](R) reader structure"]
impl crate::Readable for RxAlignmentErrorFramesSpec {}
#[doc = "`reset()` method sets RX_ALIGNMENT_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxAlignmentErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
