#[doc = "Register `RX_OUT_OF_RANGE_TYPE_FRAMES` reader"]
pub type R = crate::R<RxOutOfRangeTypeFramesSpec>;
#[doc = "Field `RXOUTOFRNG` reader - This field indicates the number of received frames with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
pub type RxoutofrngR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
    #[inline(always)]
    pub fn rxoutofrng(&self) -> RxoutofrngR {
        RxoutofrngR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Out of Range Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_out_of_range_type_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOutOfRangeTypeFramesSpec;
impl crate::RegisterSpec for RxOutOfRangeTypeFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_out_of_range_type_frames::R`](R) reader structure"]
impl crate::Readable for RxOutOfRangeTypeFramesSpec {}
#[doc = "`reset()` method sets RX_OUT_OF_RANGE_TYPE_FRAMES to value 0"]
impl crate::Resettable for RxOutOfRangeTypeFramesSpec {
    const RESET_VALUE: u32 = 0;
}
