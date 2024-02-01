#[doc = "Register `TX_OCTET_COUNT_GOOD_BAD` reader"]
pub type R = crate::R<TX_OCTET_COUNT_GOOD_BAD_SPEC>;
#[doc = "Field `TXOCTGB` reader - This field indicates the number of bytes transmitted in good and bad frames exclusive of preamble and retried bytes."]
pub type TXOCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted in good and bad frames exclusive of preamble and retried bytes."]
    #[inline(always)]
    pub fn txoctgb(&self) -> TXOCTGB_R {
        TXOCTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_octet_count_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_OCTET_COUNT_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_OCTET_COUNT_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_octet_count_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_OCTET_COUNT_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_OCTET_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for TX_OCTET_COUNT_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
