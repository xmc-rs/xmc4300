#[doc = "Reader of register RX_OCTET_COUNT_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_OCTET_COUNT_GOOD_BAD>;
#[doc = "Reader of field `RXOCTGB`"]
pub type RXOCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received, exclusive of preamble, in good and bad frames."]
    #[inline(always)]
    pub fn rxoctgb(&self) -> RXOCTGB_R {
        RXOCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
