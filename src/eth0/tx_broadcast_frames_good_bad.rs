#[doc = "Register `TX_BROADCAST_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TX_BROADCAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `TXBCASTGB` reader - This field indicates the number of transmitted good and bad broadcast frames."]
pub type TXBCASTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad broadcast frames."]
    #[inline(always)]
    pub fn txbcastgb(&self) -> TXBCASTGB_R {
        TXBCASTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_broadcast_frames_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_BROADCAST_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_BROADCAST_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_broadcast_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_BROADCAST_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_BROADCAST_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_BROADCAST_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
