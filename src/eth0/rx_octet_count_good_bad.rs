#[doc = "Register `RX_OCTET_COUNT_GOOD_BAD` reader"]
pub struct R(crate::R<RX_OCTET_COUNT_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_OCTET_COUNT_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_OCTET_COUNT_GOOD_BAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_OCTET_COUNT_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXOCTGB` reader - This field indicates the number of bytes received, exclusive of preamble, in good and bad frames."]
pub type RXOCTGB_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received, exclusive of preamble, in good and bad frames."]
    #[inline(always)]
    pub fn rxoctgb(&self) -> RXOCTGB_R {
        RXOCTGB_R::new(self.bits)
    }
}
#[doc = "Receive Octet Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_octet_count_good_bad](index.html) module"]
pub struct RX_OCTET_COUNT_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_OCTET_COUNT_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_octet_count_good_bad::R](R) reader structure"]
impl crate::Readable for RX_OCTET_COUNT_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_OCTET_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for RX_OCTET_COUNT_GOOD_BAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
