#[doc = "Register `TX_PAUSE_FRAMES` reader"]
pub type R = crate::R<TX_PAUSE_FRAMES_SPEC>;
#[doc = "Field `TXPAUSE` reader - This field indicates the number of transmitted good PAUSE frames."]
pub type TXPAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good PAUSE frames."]
    #[inline(always)]
    pub fn txpause(&self) -> TXPAUSE_R {
        TXPAUSE_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good PAUSE Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pause_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_PAUSE_FRAMES_SPEC;
impl crate::RegisterSpec for TX_PAUSE_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_pause_frames::R`](R) reader structure"]
impl crate::Readable for TX_PAUSE_FRAMES_SPEC {}
#[doc = "`reset()` method sets TX_PAUSE_FRAMES to value 0"]
impl crate::Resettable for TX_PAUSE_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
