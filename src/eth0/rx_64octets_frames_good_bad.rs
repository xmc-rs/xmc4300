#[doc = "Reader of register RX_64OCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_64OCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `RX64OCTGB`"]
pub type RX64OCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length 64 bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx64octgb(&self) -> RX64OCTGB_R {
        RX64OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
