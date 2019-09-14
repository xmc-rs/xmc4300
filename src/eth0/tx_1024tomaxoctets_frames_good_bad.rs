#[doc = "Reader of register TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `TX1024_MAXOCTGB`"]
pub type TX1024_MAXOCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx1024_maxoctgb(&self) -> TX1024_MAXOCTGB_R {
        TX1024_MAXOCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
