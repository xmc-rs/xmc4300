#[doc = "Register `RX_UNDERSIZE_FRAMES_GOOD` reader"]
pub type R = crate::R<RX_UNDERSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Field `RXUNDERSZG` reader - This field indicates the number of frames received with length less than 64 bytes and without errors."]
pub type RXUNDERSZG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length less than 64 bytes and without errors."]
    #[inline(always)]
    pub fn rxunderszg(&self) -> RXUNDERSZG_R {
        RXUNDERSZG_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Undersize Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_undersize_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_UNDERSIZE_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for RX_UNDERSIZE_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_undersize_frames_good::R`](R) reader structure"]
impl crate::Readable for RX_UNDERSIZE_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets RX_UNDERSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for RX_UNDERSIZE_FRAMES_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
