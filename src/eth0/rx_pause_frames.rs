#[doc = "Register `RX_PAUSE_FRAMES` reader"]
pub type R = crate::R<RxPauseFramesSpec>;
#[doc = "Field `RXPAUSEFRM` reader - This field indicates the number of received good and valid PAUSE frames."]
pub type RxpausefrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and valid PAUSE frames."]
    #[inline(always)]
    pub fn rxpausefrm(&self) -> RxpausefrmR {
        RxpausefrmR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for PAUSE Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_pause_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxPauseFramesSpec;
impl crate::RegisterSpec for RxPauseFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_pause_frames::R`](R) reader structure"]
impl crate::Readable for RxPauseFramesSpec {}
#[doc = "`reset()` method sets RX_PAUSE_FRAMES to value 0"]
impl crate::Resettable for RxPauseFramesSpec {
    const RESET_VALUE: u32 = 0;
}
