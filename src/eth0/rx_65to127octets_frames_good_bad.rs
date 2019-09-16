#[doc = "Reader of register RX_65TO127OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_65TO127OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `RX65_127OCTGB`"]
pub type RX65_127OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx65_127octgb(&self) -> RX65_127OCTGB_R {
        RX65_127OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
