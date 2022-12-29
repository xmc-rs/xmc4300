#[doc = "Register `TX_BROADCAST_FRAMES_GOOD` reader"]
pub struct R(crate::R<TX_BROADCAST_FRAMES_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_BROADCAST_FRAMES_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_BROADCAST_FRAMES_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_BROADCAST_FRAMES_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXBCASTG` reader - This field indicates the number of transmitted good broadcast frames."]
pub type TXBCASTG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good broadcast frames."]
    #[inline(always)]
    pub fn txbcastg(&self) -> TXBCASTG_R {
        TXBCASTG_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good Broadcast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_broadcast_frames_good](index.html) module"]
pub struct TX_BROADCAST_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for TX_BROADCAST_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_broadcast_frames_good::R](R) reader structure"]
impl crate::Readable for TX_BROADCAST_FRAMES_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_BROADCAST_FRAMES_GOOD to value 0"]
impl crate::Resettable for TX_BROADCAST_FRAMES_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
