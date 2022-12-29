#[doc = "Register `TX_EXCESSIVE_COLLISION_FRAMES` reader"]
pub struct R(crate::R<TX_EXCESSIVE_COLLISION_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_EXCESSIVE_COLLISION_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_EXCESSIVE_COLLISION_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_EXCESSIVE_COLLISION_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXEXSCOL` reader - This field indicates the number of frames aborted because of excessive (16) collision error."]
pub type TXEXSCOL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of excessive (16) collision error."]
    #[inline(always)]
    pub fn txexscol(&self) -> TXEXSCOL_R {
        TXEXSCOL_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Excessive Collision Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_excessive_collision_frames](index.html) module"]
pub struct TX_EXCESSIVE_COLLISION_FRAMES_SPEC;
impl crate::RegisterSpec for TX_EXCESSIVE_COLLISION_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_excessive_collision_frames::R](R) reader structure"]
impl crate::Readable for TX_EXCESSIVE_COLLISION_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_EXCESSIVE_COLLISION_FRAMES to value 0"]
impl crate::Resettable for TX_EXCESSIVE_COLLISION_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
