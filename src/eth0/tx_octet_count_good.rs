#[doc = "Register `TX_OCTET_COUNT_GOOD` reader"]
pub type R = crate::R<TxOctetCountGoodSpec>;
#[doc = "Field `TXOCTG` reader - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames."]
pub type TxoctgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames."]
    #[inline(always)]
    pub fn txoctg(&self) -> TxoctgR {
        TxoctgR::new(self.bits)
    }
}
#[doc = "Tx Octet Count Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_octet_count_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxOctetCountGoodSpec;
impl crate::RegisterSpec for TxOctetCountGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_octet_count_good::R`](R) reader structure"]
impl crate::Readable for TxOctetCountGoodSpec {}
#[doc = "`reset()` method sets TX_OCTET_COUNT_GOOD to value 0"]
impl crate::Resettable for TxOctetCountGoodSpec {
    const RESET_VALUE: u32 = 0;
}
