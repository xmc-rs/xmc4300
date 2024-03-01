#[doc = "Register `RX_65TO127OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<Rx65to127octetsFramesGoodBadSpec>;
#[doc = "Field `RX65_127OCTGB` reader - This field indicates the number of received good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble."]
pub type Rx65_127octgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx65_127octgb(&self) -> Rx65_127octgbR {
        Rx65_127octgbR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_65to127octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx65to127octetsFramesGoodBadSpec;
impl crate::RegisterSpec for Rx65to127octetsFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_65to127octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for Rx65to127octetsFramesGoodBadSpec {}
#[doc = "`reset()` method sets RX_65TO127OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for Rx65to127octetsFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
