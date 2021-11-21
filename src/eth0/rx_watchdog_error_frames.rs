#[doc = "Register `RX_WATCHDOG_ERROR_FRAMES` reader"]
pub struct R(crate::R<RX_WATCHDOG_ERROR_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_WATCHDOG_ERROR_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_WATCHDOG_ERROR_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_WATCHDOG_ERROR_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXWDGERR` reader - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
pub struct RXWDGERR_R(crate::FieldReader<u32, u32>);
impl RXWDGERR_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXWDGERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWDGERR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
    #[inline(always)]
    pub fn rxwdgerr(&self) -> RXWDGERR_R {
        RXWDGERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Frame Count for Watchdog Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_watchdog_error_frames](index.html) module"]
pub struct RX_WATCHDOG_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RX_WATCHDOG_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_watchdog_error_frames::R](R) reader structure"]
impl crate::Readable for RX_WATCHDOG_ERROR_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_WATCHDOG_ERROR_FRAMES to value 0"]
impl crate::Resettable for RX_WATCHDOG_ERROR_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
