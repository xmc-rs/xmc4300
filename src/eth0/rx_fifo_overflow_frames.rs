#[doc = "Register `RX_FIFO_OVERFLOW_FRAMES` reader"]
pub type R = crate::R<RxFifoOverflowFramesSpec>;
#[doc = "Field `RXFIFOOVFL` reader - This field indicates the number of received frames missed because of FIFO overflow."]
pub type RxfifoovflR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames missed because of FIFO overflow."]
    #[inline(always)]
    pub fn rxfifoovfl(&self) -> RxfifoovflR {
        RxfifoovflR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for FIFO Overflow Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_overflow_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoOverflowFramesSpec;
impl crate::RegisterSpec for RxFifoOverflowFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_overflow_frames::R`](R) reader structure"]
impl crate::Readable for RxFifoOverflowFramesSpec {}
#[doc = "`reset()` method sets RX_FIFO_OVERFLOW_FRAMES to value 0"]
impl crate::Resettable for RxFifoOverflowFramesSpec {
    const RESET_VALUE: u32 = 0;
}
