#[doc = "Register `TX_OCTET_COUNT_GOOD_BAD` reader"]
pub type R = crate::R<TxOctetCountGoodBadSpec>;
#[doc = "Field `TXOCTGB` reader - This field indicates the number of bytes transmitted in good and bad frames exclusive of preamble and retried bytes."]
pub type TxoctgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted in good and bad frames exclusive of preamble and retried bytes."]
    #[inline(always)]
    pub fn txoctgb(&self) -> TxoctgbR {
        TxoctgbR::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_octet_count_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxOctetCountGoodBadSpec;
impl crate::RegisterSpec for TxOctetCountGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_octet_count_good_bad::R`](R) reader structure"]
impl crate::Readable for TxOctetCountGoodBadSpec {}
#[doc = "`reset()` method sets TX_OCTET_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for TxOctetCountGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
