#[doc = "Register `TX_BROADCAST_FRAMES_GOOD` reader"]
pub type R = crate::R<TxBroadcastFramesGoodSpec>;
#[doc = "Field `TXBCASTG` reader - This field indicates the number of transmitted good broadcast frames."]
pub type TxbcastgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good broadcast frames."]
    #[inline(always)]
    pub fn txbcastg(&self) -> TxbcastgR {
        TxbcastgR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good Broadcast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_broadcast_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxBroadcastFramesGoodSpec;
impl crate::RegisterSpec for TxBroadcastFramesGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_broadcast_frames_good::R`](R) reader structure"]
impl crate::Readable for TxBroadcastFramesGoodSpec {}
#[doc = "`reset()` method sets TX_BROADCAST_FRAMES_GOOD to value 0"]
impl crate::Resettable for TxBroadcastFramesGoodSpec {
    const RESET_VALUE: u32 = 0;
}
