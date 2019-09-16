#[doc = "Reader of register RX_256TO511OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_256TO511OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `RX256_511OCTGB`"]
pub type RX256_511OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx256_511octgb(&self) -> RX256_511OCTGB_R {
        RX256_511OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
