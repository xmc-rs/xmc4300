#[doc = "Register `RX_256TO511OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Rx256to511octetsFramesGoodBadSpec>;
#[doc = "Field `RX256_511OCTGB` reader - This field indicates the number of received good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
pub type Rx256_511octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx256_511octgb(&self) -> Rx256_511octgbR {
        Rx256_511octgbR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_256to511octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx256to511octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Rx256to511octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_256to511octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Rx256to511octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets RX_256TO511OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Rx256to511octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
