#[doc = "Register `FWD_RX_ERR_COUNT0` reader"]
pub struct R(crate::R<FWD_RX_ERR_COUNT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWD_RX_ERR_COUNT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWD_RX_ERR_COUNT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWD_RX_ERR_COUNT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FORW_ERROR` reader - Forwarded error counter of Port y"]
pub struct FORW_ERROR_R(crate::FieldReader<u8, u8>);
impl FORW_ERROR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FORW_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORW_ERROR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Forwarded error counter of Port y"]
    #[inline(always)]
    pub fn forw_error(&self) -> FORW_ERROR_R {
        FORW_ERROR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Forwarded RX Error Counter Port 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwd_rx_err_count0](index.html) module"]
pub struct FWD_RX_ERR_COUNT0_SPEC;
impl crate::RegisterSpec for FWD_RX_ERR_COUNT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fwd_rx_err_count0::R](R) reader structure"]
impl crate::Readable for FWD_RX_ERR_COUNT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FWD_RX_ERR_COUNT0 to value 0"]
impl crate::Resettable for FWD_RX_ERR_COUNT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
