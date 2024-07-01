#[doc = "Register `TX_UNDERFLOW_ERROR_FRAMES` reader"]
pub type R = crate::R<TX_UNDERFLOW_ERROR_FRAMES_SPEC>;
#[doc = "Field `TXUNDRFLW` reader - This field indicates the number of frames aborted because of frame underflow error."]
pub type TXUNDRFLW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of frame underflow error."]
    #[inline(always)]
    pub fn txundrflw(&self) -> TXUNDRFLW_R {
        TXUNDRFLW_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Underflow Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_underflow_error_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_UNDERFLOW_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for TX_UNDERFLOW_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_underflow_error_frames::R`](R) reader structure"]
impl crate::Readable for TX_UNDERFLOW_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets TX_UNDERFLOW_ERROR_FRAMES to value 0"]
impl crate::Resettable for TX_UNDERFLOW_ERROR_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
