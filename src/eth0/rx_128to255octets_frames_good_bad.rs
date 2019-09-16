#[doc = "Reader of register RX_128TO255OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_128TO255OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `RX128_255OCTGB`"]
pub type RX128_255OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx128_255octgb(&self) -> RX128_255OCTGB_R {
        RX128_255OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
