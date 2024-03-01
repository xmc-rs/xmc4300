#[doc = "Register `TX_MULTIPLE_COLLISION_GOOD_FRAMES` reader"]
pub type R = crate::R<TxMultipleCollisionGoodFramesSpec>;
#[doc = "Field `TXMULTCOLG` reader - This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode."]
pub type TxmultcolgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode."]
    #[inline(always)]
    pub fn txmultcolg(&self) -> TxmultcolgR {
        TxmultcolgR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multiple_collision_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxMultipleCollisionGoodFramesSpec;
impl crate::RegisterSpec for TxMultipleCollisionGoodFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_multiple_collision_good_frames::R`](R) reader structure"]
impl crate::Readable for TxMultipleCollisionGoodFramesSpec {}
#[doc = "`reset()` method sets TX_MULTIPLE_COLLISION_GOOD_FRAMES to value 0"]
impl crate::Resettable for TxMultipleCollisionGoodFramesSpec {
    const RESET_VALUE: u32 = 0;
}
