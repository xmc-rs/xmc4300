#[doc = "Register `RX_JABBER_ERROR_FRAMES` reader"]
pub type R = crate::R<RX_JABBER_ERROR_FRAMES_SPEC>;
#[doc = "Field `RXJABERR` reader - This field indicates the number of giant frames received with length (including CRC) greater than 1,518 bytes (1,522 bytes for VLAN tagged) and with CRC error. If Jumbo Frame mode is enabled, then frames of length greater than 9,018 bytes (9,022 for VLAN tagged) are considered as giant frames."]
pub type RXJABERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of giant frames received with length (including CRC) greater than 1,518 bytes (1,522 bytes for VLAN tagged) and with CRC error. If Jumbo Frame mode is enabled, then frames of length greater than 9,018 bytes (9,022 for VLAN tagged) are considered as giant frames."]
    #[inline(always)]
    pub fn rxjaberr(&self) -> RXJABERR_R {
        RXJABERR_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Jabber Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_jabber_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_JABBER_ERROR_FRAMES_SPEC;
impl crate::RegisterSpec for RX_JABBER_ERROR_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_jabber_error_frames::R`](R) reader structure"]
impl crate::Readable for RX_JABBER_ERROR_FRAMES_SPEC {}
#[doc = "`reset()` method sets RX_JABBER_ERROR_FRAMES to value 0"]
impl crate::Resettable for RX_JABBER_ERROR_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
