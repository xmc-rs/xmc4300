#[doc = "Register `RX_RUNT_ERROR_FRAMES` reader"]
pub struct R(crate::R<RX_RUNT_ERROR_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_RUNT_ERROR_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_RUNT_ERROR_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_RUNT_ERROR_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRUNTERR` reader - This field indicates the number of frames received with runt error(<64 bytes and CRC error)."]
pub struct RXRUNTERR_R(crate::FieldReader<u32, u32>);
impl RXRUNTERR_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXRUNTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRUNTERR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with runt error(<64 bytes and CRC error)."]
    #[inline(always)]
    pub fn rxrunterr(&self) -> RXRUNTERR_R {
        RXRUNTERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Frame Count for Runt Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_runt_error_frames](index.html) module"]
pub struct RX_RUNT_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RX_RUNT_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_runt_error_frames::R](R) reader structure"]
impl crate::Readable for RX_RUNT_ERROR_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_RUNT_ERROR_FRAMES to value 0"]
impl crate::Resettable for RX_RUNT_ERROR_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
