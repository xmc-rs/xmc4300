#[doc = "Register `TX_DEFERRED_FRAMES` reader"]
pub struct R(crate::R<TX_DEFERRED_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_DEFERRED_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_DEFERRED_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_DEFERRED_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXDEFRD` reader - This field indicates the number of successfully transmitted frames after a deferral in the half-duplex mode."]
pub type TXDEFRD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a deferral in the half-duplex mode."]
    #[inline(always)]
    pub fn txdefrd(&self) -> TXDEFRD_R {
        TXDEFRD_R::new(self.bits)
    }
}
#[doc = "Tx Deferred Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_deferred_frames](index.html) module"]
pub struct TX_DEFERRED_FRAMES_SPEC;
impl crate::RegisterSpec for TX_DEFERRED_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_deferred_frames::R](R) reader structure"]
impl crate::Readable for TX_DEFERRED_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_DEFERRED_FRAMES to value 0"]
impl crate::Resettable for TX_DEFERRED_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
