#[doc = "Register `TX_SINGLE_COLLISION_GOOD_FRAMES` reader"]
pub type R = crate::R<TxSingleCollisionGoodFramesSpec>;
#[doc = "Field `TXSNGLCOLG` reader - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
pub type TxsnglcolgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TxsnglcolgR {
        TxsnglcolgR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_single_collision_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxSingleCollisionGoodFramesSpec;
impl crate::RegisterSpec for TxSingleCollisionGoodFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_single_collision_good_frames::R`](R) reader structure"]
impl crate::Readable for TxSingleCollisionGoodFramesSpec {}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_FRAMES to value 0"]
impl crate::Resettable for TxSingleCollisionGoodFramesSpec {
    const RESET_VALUE: u32 = 0;
}
