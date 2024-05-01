#[doc = "Register `TX_65TO127OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `TX65_127OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
pub type TX65_127OCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx65_127octgb(&self) -> TX65_127OCTGB_R {
        TX65_127OCTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_65to127octets_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_65to127octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_65TO127OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
