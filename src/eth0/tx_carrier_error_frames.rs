#[doc = "Register `TX_CARRIER_ERROR_FRAMES` reader"]
pub struct R(crate::R<TX_CARRIER_ERROR_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CARRIER_ERROR_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CARRIER_ERROR_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CARRIER_ERROR_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCARR` reader - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
pub struct TXCARR_R(crate::FieldReader<u32, u32>);
impl TXCARR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXCARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCARR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
    #[inline(always)]
    pub fn txcarr(&self) -> TXCARR_R {
        TXCARR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Transmit Frame Count for Carrier Sense Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_carrier_error_frames](index.html) module"]
pub struct TX_CARRIER_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for TX_CARRIER_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_carrier_error_frames::R](R) reader structure"]
impl crate::Readable for TX_CARRIER_ERROR_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_CARRIER_ERROR_FRAMES to value 0"]
impl crate::Resettable for TX_CARRIER_ERROR_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
