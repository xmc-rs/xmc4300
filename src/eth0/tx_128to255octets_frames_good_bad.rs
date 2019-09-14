#[doc = "Reader of register TX_128TO255OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_128TO255OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TX128_255OCTGB`"]
pub type TX128_255OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx128_255octgb(&self) -> TX128_255OCTGB_R {
        TX128_255OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
