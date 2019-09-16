#[doc = "Reader of register TX_512TO1023OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_512TO1023OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TX512_1023OCTGB`"]
pub type TX512_1023OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx512_1023octgb(&self) -> TX512_1023OCTGB_R {
        TX512_1023OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
