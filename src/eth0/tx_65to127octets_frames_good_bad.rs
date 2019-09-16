#[doc = "Reader of register TX_65TO127OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_65TO127OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TX65_127OCTGB`"]
pub type TX65_127OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx65_127octgb(&self) -> TX65_127OCTGB_R {
        TX65_127OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
