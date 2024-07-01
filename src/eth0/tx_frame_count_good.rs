#[doc = "Register `TX_FRAME_COUNT_GOOD` reader"]
pub type R = crate::R<TX_FRAME_COUNT_GOOD_SPEC>;
#[doc = "Field `TXFRMG` reader - This field indicates the number of transmitted good frames, exclusive of preamble."]
pub type TXFRMG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good frames, exclusive of preamble."]
    #[inline(always)]
    pub fn txfrmg(&self) -> TXFRMG_R {
        TXFRMG_R::new(self.bits)
    }
}
#[doc = "Tx Frame Count Good Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_frame_count_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_FRAME_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for TX_FRAME_COUNT_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_frame_count_good::R`](R) reader structure"]
impl crate::Readable for TX_FRAME_COUNT_GOOD_SPEC {}
#[doc = "`reset()` method sets TX_FRAME_COUNT_GOOD to value 0"]
impl crate::Resettable for TX_FRAME_COUNT_GOOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
