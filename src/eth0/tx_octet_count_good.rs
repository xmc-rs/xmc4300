#[doc = "Register `TX_OCTET_COUNT_GOOD` reader"]
pub struct R(crate::R<TX_OCTET_COUNT_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_OCTET_COUNT_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_OCTET_COUNT_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_OCTET_COUNT_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXOCTG` reader - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames."]
pub type TXOCTG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames."]
    #[inline(always)]
    pub fn txoctg(&self) -> TXOCTG_R {
        TXOCTG_R::new(self.bits)
    }
}
#[doc = "Tx Octet Count Good Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_octet_count_good](index.html) module"]
pub struct TX_OCTET_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for TX_OCTET_COUNT_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_octet_count_good::R](R) reader structure"]
impl crate::Readable for TX_OCTET_COUNT_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_OCTET_COUNT_GOOD to value 0"]
impl crate::Resettable for TX_OCTET_COUNT_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
