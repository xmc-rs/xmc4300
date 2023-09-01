#[doc = "Register `TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `TX1024_MAXOCTGB` reader - This field indicates the number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
pub type TX1024_MAXOCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx1024_maxoctgb(&self) -> TX1024_MAXOCTGB_R {
        TX1024_MAXOCTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_1024tomaxoctets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_1024tomaxoctets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
