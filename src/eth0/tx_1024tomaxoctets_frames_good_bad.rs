#[doc = "Register `TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Tx1024tomaxoctetsFramesGoodBadSpec>;
#[doc = "Field `TX1024_MAXOCTGB` reader - This field indicates the number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
pub type Tx1024MaxoctgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx1024_maxoctgb(&self) -> Tx1024MaxoctgbR {
        Tx1024MaxoctgbR::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_1024tomaxoctets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx1024tomaxoctetsFramesGoodBadSpec;
impl crate::RegisterSpec for Tx1024tomaxoctetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_1024tomaxoctets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Tx1024tomaxoctetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Tx1024tomaxoctetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
