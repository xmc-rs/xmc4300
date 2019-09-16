#[doc = "Reader of register RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `RX1024_MAXOCTGB`"]
pub type RX1024_MAXOCTGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn rx1024_maxoctgb(&self) -> RX1024_MAXOCTGB_R {
        RX1024_MAXOCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
