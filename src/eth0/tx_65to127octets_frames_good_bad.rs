#[doc = "Register `TX_65TO127OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Tx65to127octetsFramesGoodBadSpec>;
#[doc = "Field `TX65_127OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
pub type Tx65_127octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx65_127octgb(&self) -> Tx65_127octgbR {
        Tx65_127octgbR::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_65to127octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx65to127octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Tx65to127octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_65to127octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Tx65to127octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets TX_65TO127OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Tx65to127octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
