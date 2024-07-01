#[doc = "Register `RX_PAUSE_FRAMES` reader"]
pub type R = crate::R<RX_PAUSE_FRAMES_SPEC>;
#[doc = "Field `RXPAUSEFRM` reader - This field indicates the number of received good and valid PAUSE frames."]
pub type RXPAUSEFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and valid PAUSE frames."]
    #[inline(always)]
    pub fn rxpausefrm(&self) -> RXPAUSEFRM_R {
        RXPAUSEFRM_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for PAUSE Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pause_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_PAUSE_FRAMES_SPEC;
impl crate::RegisterSpec for RX_PAUSE_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_pause_frames::R`](R) reader structure"]
impl crate::Readable for RX_PAUSE_FRAMES_SPEC {}
#[doc = "`reset()` method sets RX_PAUSE_FRAMES to value 0"]
impl crate::Resettable for RX_PAUSE_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
