#[doc = "Register `TX_MULTICAST_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TxMulticastFramesGoodBadSpec>;
#[doc = "Field `TXMCASTGB` reader - This field indicates the number of transmitted good and bad multicast frames."]
pub type TxmcastgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad multicast frames."]
    #[inline(always)]
    pub fn txmcastgb(&self) -> TxmcastgbR {
        TxmcastgbR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multicast_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxMulticastFramesGoodBadSpec;
impl crate::RegisterSpec for TxMulticastFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_multicast_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TxMulticastFramesGoodBadSpec {}
#[doc = "`reset()` method sets TX_MULTICAST_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TxMulticastFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
