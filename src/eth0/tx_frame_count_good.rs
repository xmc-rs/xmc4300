#[doc = "Register `TX_FRAME_COUNT_GOOD` reader"]
pub type R = crate::R<TxFrameCountGoodSpec>;
#[doc = "Field `TXFRMG` reader - This field indicates the number of transmitted good frames, exclusive of preamble."]
pub type TxfrmgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good frames, exclusive of preamble."]
    #[inline(always)]
    pub fn txfrmg(&self) -> TxfrmgR {
        TxfrmgR::new(self.bits)
    }
}
#[doc = "Tx Frame Count Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_frame_count_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFrameCountGoodSpec;
impl crate::RegisterSpec for TxFrameCountGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_frame_count_good::R`](R) reader structure"]
impl crate::Readable for TxFrameCountGoodSpec {}
#[doc = "`reset()` method sets TX_FRAME_COUNT_GOOD to value 0"]
impl crate::Resettable for TxFrameCountGoodSpec {
    const RESET_VALUE: u32 = 0;
}
