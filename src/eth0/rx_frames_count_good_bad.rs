#[doc = "Register `RX_FRAMES_COUNT_GOOD_BAD` reader"]
pub type R = crate::R<RX_FRAMES_COUNT_GOOD_BAD_SPEC>;
#[doc = "Field `RXFRMGB` reader - This field indicates the number of received good and bad frames."]
pub type RXFRMGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames."]
    #[inline(always)]
    pub fn rxfrmgb(&self) -> RXFRMGB_R {
        RXFRMGB_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_frames_count_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_FRAMES_COUNT_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_FRAMES_COUNT_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_frames_count_good_bad::R`](R) reader structure"]
impl crate::Readable for RX_FRAMES_COUNT_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets RX_FRAMES_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for RX_FRAMES_COUNT_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
