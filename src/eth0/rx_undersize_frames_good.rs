#[doc = "Register `RX_UNDERSIZE_FRAMES_GOOD` reader"]
pub type R = crate::R<RxUndersizeFramesGoodSpec>;
#[doc = "Field `RXUNDERSZG` reader - This field indicates the number of frames received with length less than 64 bytes and without errors."]
pub type RxunderszgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length less than 64 bytes and without errors."]
    #[inline(always)]
    pub fn rxunderszg(&self) -> RxunderszgR {
        RxunderszgR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Undersize Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_undersize_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxUndersizeFramesGoodSpec;
impl crate::RegisterSpec for RxUndersizeFramesGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_undersize_frames_good::R`](R) reader structure"]
impl crate::Readable for RxUndersizeFramesGoodSpec {}
#[doc = "`reset()` method sets RX_UNDERSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for RxUndersizeFramesGoodSpec {
    const RESET_VALUE: u32 = 0;
}
