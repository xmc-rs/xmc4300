#[doc = "Register `TX_CARRIER_ERROR_FRAMES` reader"]
pub type R = crate::R<TX_CARRIER_ERROR_FRAMES_SPEC>;
#[doc = "Field `TXCARR` reader - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
pub type TXCARR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
    #[inline(always)]
    pub fn txcarr(&self) -> TXCARR_R {
        TXCARR_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Carrier Sense Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_carrier_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CARRIER_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for TX_CARRIER_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_carrier_error_frames::R`](R) reader structure"]
impl crate::Readable for TX_CARRIER_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets TX_CARRIER_ERROR_FRAMES to value 0"]
impl crate::Resettable for TX_CARRIER_ERROR_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
