#[doc = "Register `TX_SINGLE_COLLISION_GOOD_FRAMES` reader"]
pub struct R(crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXSNGLCOLG` reader - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
pub struct TXSNGLCOLG_R(crate::FieldReader<u32, u32>);
impl TXSNGLCOLG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXSNGLCOLG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSNGLCOLG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_single_collision_good_frames](index.html) module"]
pub struct TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_single_collision_good_frames::R](R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_FRAMES to value 0"]
impl crate::Resettable for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
