#[doc = "Register `TX_DEFERRED_FRAMES` reader"]
pub type R = crate::R<TxDeferredFramesSpec>;
#[doc = "Field `TXDEFRD` reader - This field indicates the number of successfully transmitted frames after a deferral in the half-duplex mode."]
pub type TxdefrdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a deferral in the half-duplex mode."]
    #[inline(always)]
    pub fn txdefrd(&self) -> TxdefrdR {
        TxdefrdR::new(self.bits)
    }
}
#[doc = "Tx Deferred Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_deferred_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDeferredFramesSpec;
impl crate::RegisterSpec for TxDeferredFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_deferred_frames::R`](R) reader structure"]
impl crate::Readable for TxDeferredFramesSpec {}
#[doc = "`reset()` method sets TX_DEFERRED_FRAMES to value 0"]
impl crate::Resettable for TxDeferredFramesSpec {
    const RESET_VALUE: u32 = 0;
}
