#[doc = "Register `TX_MULTICAST_FRAMES_GOOD` reader"]
pub type R = crate::R<TX_MULTICAST_FRAMES_GOOD_SPEC>;
#[doc = "Field `TXMCASTG` reader - This field indicates the number of transmitted good multicast frames."]
pub type TXMCASTG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good multicast frames."]
    #[inline(always)]
    pub fn txmcastg(&self) -> TXMCASTG_R {
        TXMCASTG_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good Multicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multicast_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_MULTICAST_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for TX_MULTICAST_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_multicast_frames_good::R`](R) reader structure"]
impl crate::Readable for TX_MULTICAST_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets TX_MULTICAST_FRAMES_GOOD to value 0"]
impl crate::Resettable for TX_MULTICAST_FRAMES_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
