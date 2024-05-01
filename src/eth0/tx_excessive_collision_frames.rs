#[doc = "Register `TX_EXCESSIVE_COLLISION_FRAMES` reader"]
pub type R = crate::R<TxExcessiveCollisionFramesSpec>;
#[doc = "Field `TXEXSCOL` reader - This field indicates the number of frames aborted because of excessive (16) collision error."]
pub type TxexscolR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of excessive (16) collision error."]
    #[inline(always)]
    pub fn txexscol(&self) -> TxexscolR {
        TxexscolR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Excessive Collision Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_excessive_collision_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxExcessiveCollisionFramesSpec;
impl crate::RegisterSpec for TxExcessiveCollisionFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_excessive_collision_frames::R`](R) reader structure"]
impl crate::Readable for TxExcessiveCollisionFramesSpec {}
#[doc = "`reset()` method sets TX_EXCESSIVE_COLLISION_FRAMES to value 0"]
impl crate::Resettable for TxExcessiveCollisionFramesSpec {
    const RESET_VALUE: u32 = 0;
}
