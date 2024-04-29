#[doc = "Register `RX_OUT_OF_RANGE_TYPE_FRAMES` reader"]
pub type R = crate::R<RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>;
#[doc = "Field `RXOUTOFRNG` reader - This field indicates the number of received frames with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
pub type RXOUTOFRNG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
    #[inline(always)]
    pub fn rxoutofrng(&self) -> RXOUTOFRNG_R {
        RXOUTOFRNG_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Out of Range Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_out_of_range_type_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC;
impl crate::RegisterSpec for RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_out_of_range_type_frames::R`](R) reader structure"]
impl crate::Readable for RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC {}
#[doc = "`reset()` method sets RX_OUT_OF_RANGE_TYPE_FRAMES to value 0"]
impl crate::Resettable for RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
