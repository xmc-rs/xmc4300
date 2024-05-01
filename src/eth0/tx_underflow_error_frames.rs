#[doc = "Register `TX_UNDERFLOW_ERROR_FRAMES` reader"]
pub type R = crate::R<TxUnderflowErrorFramesSpec>;
#[doc = "Field `TXUNDRFLW` reader - This field indicates the number of frames aborted because of frame underflow error."]
pub type TxundrflwR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of frame underflow error."]
    #[inline(always)]
    pub fn txundrflw(&self) -> TxundrflwR {
        TxundrflwR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Underflow Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_underflow_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxUnderflowErrorFramesSpec;
impl crate::RegisterSpec for TxUnderflowErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_underflow_error_frames::R`](R) reader structure"]
impl crate::Readable for TxUnderflowErrorFramesSpec {}
#[doc = "`reset()` method sets TX_UNDERFLOW_ERROR_FRAMES to value 0"]
impl crate::Resettable for TxUnderflowErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
