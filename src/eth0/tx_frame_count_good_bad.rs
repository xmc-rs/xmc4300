#[doc = "Register `TX_FRAME_COUNT_GOOD_BAD` reader"]
pub type R = crate::R<TX_FRAME_COUNT_GOOD_BAD_SPEC>;
#[doc = "Field `TXFRMGB` reader - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
pub type TXFRMGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline(always)]
    pub fn txfrmgb(&self) -> TXFRMGB_R {
        TXFRMGB_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Goodand Bad Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_frame_count_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_FRAME_COUNT_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_FRAME_COUNT_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_frame_count_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_FRAME_COUNT_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_FRAME_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for TX_FRAME_COUNT_GOOD_BAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
