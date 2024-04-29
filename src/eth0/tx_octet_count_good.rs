#[doc = "Register `TX_OCTET_COUNT_GOOD` reader"]
pub type R = crate::R<TX_OCTET_COUNT_GOOD_SPEC>;
#[doc = "Field `TXOCTG` reader - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames."]
pub type TXOCTG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames."]
    #[inline(always)]
    pub fn txoctg(&self) -> TXOCTG_R {
        TXOCTG_R::new(self.bits)
    }
}
#[doc = "Tx Octet Count Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_octet_count_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_OCTET_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for TX_OCTET_COUNT_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_octet_count_good::R`](R) reader structure"]
impl crate::Readable for TX_OCTET_COUNT_GOOD_SPEC {}
#[doc = "`reset()` method sets TX_OCTET_COUNT_GOOD to value 0"]
impl crate::Resettable for TX_OCTET_COUNT_GOOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
