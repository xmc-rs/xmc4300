#[doc = "Register `RX_OUT_OF_RANGE_TYPE_FRAMES` reader"]
pub struct R(crate::R<RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXOUTOFRNG` reader - This field indicates the number of received frames with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
pub type RXOUTOFRNG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
    #[inline(always)]
    pub fn rxoutofrng(&self) -> RXOUTOFRNG_R {
        RXOUTOFRNG_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Out of Range Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_out_of_range_type_frames](index.html) module"]
pub struct RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC;
impl crate::RegisterSpec for RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_out_of_range_type_frames::R](R) reader structure"]
impl crate::Readable for RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_OUT_OF_RANGE_TYPE_FRAMES to value 0"]
impl crate::Resettable for RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
