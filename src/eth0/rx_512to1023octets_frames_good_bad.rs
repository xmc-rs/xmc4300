#[doc = "Reader of register RX_512TO1023OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_512TO1023OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `RX512_1023OCTGB`"]
pub type RX512_1023OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx512_1023octgb(&self) -> RX512_1023OCTGB_R {
        RX512_1023OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
