#[doc = "Register `RX_ERR_COUNT1` reader"]
pub struct R(crate::R<RX_ERR_COUNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ERR_COUNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ERR_COUNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ERR_COUNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INVALID_FRAME` reader - Invalid frame counter of Port y"]
pub struct INVALID_FRAME_R(crate::FieldReader<u8, u8>);
impl INVALID_FRAME_R {
    pub(crate) fn new(bits: u8) -> Self {
        INVALID_FRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVALID_FRAME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ERROR` reader - RX Error counter of Port y"]
pub struct RX_ERROR_R(crate::FieldReader<u8, u8>);
impl RX_ERROR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ERROR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "RX Error Counter Port 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_err_count1](index.html) module"]
pub struct RX_ERR_COUNT1_SPEC;
impl crate::RegisterSpec for RX_ERR_COUNT1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rx_err_count1::R](R) reader structure"]
impl crate::Readable for RX_ERR_COUNT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_ERR_COUNT1 to value 0"]
impl crate::Resettable for RX_ERR_COUNT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
