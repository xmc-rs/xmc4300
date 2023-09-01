#[doc = "Register `TX_MULTICAST_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TX_MULTICAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `TXMCASTGB` reader - This field indicates the number of transmitted good and bad multicast frames."]
pub type TXMCASTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad multicast frames."]
    #[inline(always)]
    pub fn txmcastgb(&self) -> TXMCASTGB_R {
        TXMCASTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multicast_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_MULTICAST_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_MULTICAST_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_multicast_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_MULTICAST_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_MULTICAST_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_MULTICAST_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
