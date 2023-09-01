#[doc = "Register `RX_OCTET_COUNT_GOOD_BAD` reader"]
pub type R = crate::R<RX_OCTET_COUNT_GOOD_BAD_SPEC>;
#[doc = "Field `RXOCTGB` reader - This field indicates the number of bytes received, exclusive of preamble, in good and bad frames."]
pub type RXOCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received, exclusive of preamble, in good and bad frames."]
    #[inline(always)]
    pub fn rxoctgb(&self) -> RXOCTGB_R {
        RXOCTGB_R::new(self.bits)
    }
}
#[doc = "Receive Octet Count for Good and Bad Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_octet_count_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_OCTET_COUNT_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_OCTET_COUNT_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_octet_count_good_bad::R`](R) reader structure"]
impl crate::Readable for RX_OCTET_COUNT_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets RX_OCTET_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for RX_OCTET_COUNT_GOOD_BAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
