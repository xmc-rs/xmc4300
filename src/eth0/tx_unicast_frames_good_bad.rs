#[doc = "Register `TX_UNICAST_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TX_UNICAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `TXUCASTGB` reader - This field indicates the number of transmitted good and bad unicast frames."]
pub type TXUCASTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad unicast frames."]
    #[inline(always)]
    pub fn txucastgb(&self) -> TXUCASTGB_R {
        TXUCASTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_unicast_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_UNICAST_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_UNICAST_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_unicast_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_UNICAST_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_UNICAST_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_UNICAST_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
