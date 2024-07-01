#[doc = "Register `RX_UNICAST_FRAMES_GOOD` reader"]
pub type R = crate::R<RX_UNICAST_FRAMES_GOOD_SPEC>;
#[doc = "Field `RXUCASTG` reader - This field indicates the number of received good unicast frames."]
pub type RXUCASTG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good unicast frames."]
    #[inline(always)]
    pub fn rxucastg(&self) -> RXUCASTG_R {
        RXUCASTG_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good Unicast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_unicast_frames_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_UNICAST_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for RX_UNICAST_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_unicast_frames_good::R`](R) reader structure"]
impl crate::Readable for RX_UNICAST_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets RX_UNICAST_FRAMES_GOOD to value 0"]
impl crate::Resettable for RX_UNICAST_FRAMES_GOOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
