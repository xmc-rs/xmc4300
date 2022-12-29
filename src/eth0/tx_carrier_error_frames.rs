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
pub type TXCARR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
    #[inline(always)]
    pub fn txcarr(&self) -> TXCARR_R {
        TXCARR_R::new(self.bits)
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
    const RESET_VALUE: Self::Ux = 0;
}
