#[doc = "Register `TX_FRAME_COUNT_GOOD_BAD` reader"]
pub type R = crate::R<TxFrameCountGoodBadSpec>;
#[doc = "Field `TXFRMGB` reader - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
pub type TxfrmgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline(always)]
    pub fn txfrmgb(&self) -> TxfrmgbR {
        TxfrmgbR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Goodand Bad Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_frame_count_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFrameCountGoodBadSpec;
impl crate::RegisterSpec for TxFrameCountGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_frame_count_good_bad::R`](R) reader structure"]
impl crate::Readable for TxFrameCountGoodBadSpec {}
#[doc = "`reset()` method sets TX_FRAME_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for TxFrameCountGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
