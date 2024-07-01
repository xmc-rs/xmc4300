#[doc = "Register `RX_OCTET_COUNT_GOOD` reader"]
pub type R = crate::R<RX_OCTET_COUNT_GOOD_SPEC>;
#[doc = "Field `RXOCTG` reader - This field indicates the number of bytes received, exclusive of preamble, only in good frames."]
pub type RXOCTG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received, exclusive of preamble, only in good frames."]
    #[inline(always)]
    pub fn rxoctg(&self) -> RXOCTG_R {
        RXOCTG_R::new(self.bits)
    }
}
#[doc = "Rx Octet Count Good Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_octet_count_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_OCTET_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for RX_OCTET_COUNT_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_octet_count_good::R`](R) reader structure"]
impl crate::Readable for RX_OCTET_COUNT_GOOD_SPEC {}
#[doc = "`reset()` method sets RX_OCTET_COUNT_GOOD to value 0"]
impl crate::Resettable for RX_OCTET_COUNT_GOOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
