#[doc = "Register `RX_CRC_ERROR_FRAMES` reader"]
pub type R = crate::R<RX_CRC_ERROR_FRAMES_SPEC>;
#[doc = "Field `RXCRCERR` reader - This field indicates the number of frames received with CRC error."]
pub type RXCRCERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with CRC error."]
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RXCRCERR_R {
        RXCRCERR_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for CRC Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CRC_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RX_CRC_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_error_frames::R`](R) reader structure"]
impl crate::Readable for RX_CRC_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets RX_CRC_ERROR_FRAMES to value 0"]
impl crate::Resettable for RX_CRC_ERROR_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
