#[doc = "Register `TX_LATE_COLLISION_FRAMES` reader"]
pub type R = crate::R<TX_LATE_COLLISION_FRAMES_SPEC>;
#[doc = "Field `TXLATECOL` reader - This field indicates the number of frames aborted because of late collision error."]
pub type TXLATECOL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of late collision error."]
    #[inline(always)]
    pub fn txlatecol(&self) -> TXLATECOL_R {
        TXLATECOL_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Late Collision Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_late_collision_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_LATE_COLLISION_FRAMES_SPEC;
impl crate::RegisterSpec for TX_LATE_COLLISION_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_late_collision_frames::R`](R) reader structure"]
impl crate::Readable for TX_LATE_COLLISION_FRAMES_SPEC {}
#[doc = "`reset()` method sets TX_LATE_COLLISION_FRAMES to value 0"]
impl crate::Resettable for TX_LATE_COLLISION_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
