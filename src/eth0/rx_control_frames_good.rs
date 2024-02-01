#[doc = "Register `RX_CONTROL_FRAMES_GOOD` reader"]
pub type R = crate::R<RX_CONTROL_FRAMES_GOOD_SPEC>;
#[doc = "Field `RXCTRLG` reader - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
pub type RXCTRLG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with error because of the watchdog timeout error (frames with more than 2,048 bytes data load)."]
    #[inline(always)]
    pub fn rxctrlg(&self) -> RXCTRLG_R {
        RXCTRLG_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good Control Frames Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_control_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CONTROL_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for RX_CONTROL_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_control_frames_good::R`](R) reader structure"]
impl crate::Readable for RX_CONTROL_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets RX_CONTROL_FRAMES_GOOD to value 0"]
impl crate::Resettable for RX_CONTROL_FRAMES_GOOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
