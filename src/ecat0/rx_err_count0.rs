#[doc = "Register `RX_ERR_COUNT0` reader"]
pub struct R(crate::R<RX_ERR_COUNT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ERR_COUNT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ERR_COUNT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ERR_COUNT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INVALID_FRAME` reader - Invalid frame counter of Port y"]
pub type INVALID_FRAME_R = crate::FieldReader;
#[doc = "Field `RX_ERROR` reader - RX Error counter of Port y"]
pub type RX_ERROR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Invalid frame counter of Port y"]
    #[inline(always)]
    pub fn invalid_frame(&self) -> INVALID_FRAME_R {
        INVALID_FRAME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Error counter of Port y"]
    #[inline(always)]
    pub fn rx_error(&self) -> RX_ERROR_R {
        RX_ERROR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "RX Error Counter Port 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_err_count0](index.html) module"]
pub struct RX_ERR_COUNT0_SPEC;
impl crate::RegisterSpec for RX_ERR_COUNT0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rx_err_count0::R](R) reader structure"]
impl crate::Readable for RX_ERR_COUNT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_ERR_COUNT0 to value 0"]
impl crate::Resettable for RX_ERR_COUNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
