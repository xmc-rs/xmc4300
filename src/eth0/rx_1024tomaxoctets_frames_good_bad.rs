#[doc = "Register `RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `RX1024_MAXOCTGB` reader - This field indicates the number of received good and bad frames with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
pub type RX1024_MAXOCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn rx1024_maxoctgb(&self) -> RX1024_MAXOCTGB_R {
        RX1024_MAXOCTGB_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_1024tomaxoctets_frames_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_1024tomaxoctets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
