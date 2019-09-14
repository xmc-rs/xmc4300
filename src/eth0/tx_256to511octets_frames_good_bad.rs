#[doc = "Reader of register TX_256TO511OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_256TO511OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TX256_511OCTGB`"]
pub type TX256_511OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx256_511octgb(&self) -> TX256_511OCTGB_R {
        TX256_511OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
