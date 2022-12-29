#[doc = "Register `RX_BROADCAST_FRAMES_GOOD` reader"]
pub struct R(crate::R<RX_BROADCAST_FRAMES_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_BROADCAST_FRAMES_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_BROADCAST_FRAMES_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_BROADCAST_FRAMES_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXBCASTG` reader - This field indicates the number of received good broadcast frames."]
pub type RXBCASTG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good broadcast frames."]
    #[inline(always)]
    pub fn rxbcastg(&self) -> RXBCASTG_R {
        RXBCASTG_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good Broadcast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_broadcast_frames_good](index.html) module"]
pub struct RX_BROADCAST_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for RX_BROADCAST_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_broadcast_frames_good::R](R) reader structure"]
impl crate::Readable for RX_BROADCAST_FRAMES_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_BROADCAST_FRAMES_GOOD to value 0"]
impl crate::Resettable for RX_BROADCAST_FRAMES_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
