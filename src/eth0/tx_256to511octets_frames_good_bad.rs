#[doc = "Register `TX_256TO511OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Tx256to511octetsFramesGoodBadSpec>;
#[doc = "Field `TX256_511OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
pub type Tx256_511octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx256_511octgb(&self) -> Tx256_511octgbR {
        Tx256_511octgbR::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_256to511octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx256to511octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Tx256to511octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_256to511octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Tx256to511octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets TX_256TO511OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Tx256to511octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
