#[doc = "Register `TX_SINGLE_COLLISION_GOOD_FRAMES` reader"]
pub type R = crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>;
#[doc = "Field `TXSNGLCOLG` reader - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
pub type TXSNGLCOLG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_single_collision_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_single_collision_good_frames::R`](R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_FRAMES to value 0"]
impl crate::Resettable for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
