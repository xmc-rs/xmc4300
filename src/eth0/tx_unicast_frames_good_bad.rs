#[doc = "Register `TX_UNICAST_FRAMES_GOOD_BAD` reader"]
pub struct R(crate::R<TX_UNICAST_FRAMES_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_UNICAST_FRAMES_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_UNICAST_FRAMES_GOOD_BAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_UNICAST_FRAMES_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXUCASTGB` reader - This field indicates the number of transmitted good and bad unicast frames."]
pub struct TXUCASTGB_R(crate::FieldReader<u32, u32>);
impl TXUCASTGB_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXUCASTGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUCASTGB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad unicast frames."]
    #[inline(always)]
    pub fn txucastgb(&self) -> TXUCASTGB_R {
        TXUCASTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_unicast_frames_good_bad](index.html) module"]
pub struct TX_UNICAST_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_UNICAST_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_unicast_frames_good_bad::R](R) reader structure"]
impl crate::Readable for TX_UNICAST_FRAMES_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_UNICAST_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_UNICAST_FRAMES_GOOD_BAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
