#[doc = "Reader of register TX_64OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_64OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TX64OCTGB`"]
pub type TX64OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length of 64 bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx64octgb(&self) -> TX64OCTGB_R {
        TX64OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
