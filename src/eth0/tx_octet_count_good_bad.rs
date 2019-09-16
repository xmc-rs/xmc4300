#[doc = "Reader of register TX_OCTET_COUNT_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_OCTET_COUNT_GOOD_BAD>;
#[doc = "Reader of field `TXOCTGB`"]
pub type TXOCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted in good and bad frames exclusive of preamble and retried bytes."]
    #[inline(always)]
    pub fn txoctgb(&self) -> TXOCTGB_R {
        TXOCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
