#[doc = "Register `TX_DEFERRED_FRAMES` reader"]
pub type R = crate::R<TX_DEFERRED_FRAMES_SPEC>;
#[doc = "Field `TXDEFRD` reader - This field indicates the number of successfully transmitted frames after a deferral in the half-duplex mode."]
pub type TXDEFRD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a deferral in the half-duplex mode."]
    #[inline(always)]
    pub fn txdefrd(&self) -> TXDEFRD_R {
        TXDEFRD_R::new(self.bits)
    }
}
#[doc = "Tx Deferred Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_deferred_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DEFERRED_FRAMES_SPEC;
impl crate::RegisterSpec for TX_DEFERRED_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_deferred_frames::R`](R) reader structure"]
impl crate::Readable for TX_DEFERRED_FRAMES_SPEC {}
#[doc = "`reset()` method sets TX_DEFERRED_FRAMES to value 0"]
impl crate::Resettable for TX_DEFERRED_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
