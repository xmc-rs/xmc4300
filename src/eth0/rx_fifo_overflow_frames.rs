#[doc = "Register `RX_FIFO_OVERFLOW_FRAMES` reader"]
pub type R = crate::R<RX_FIFO_OVERFLOW_FRAMES_SPEC>;
#[doc = "Field `RXFIFOOVFL` reader - This field indicates the number of received frames missed because of FIFO overflow."]
pub type RXFIFOOVFL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames missed because of FIFO overflow."]
    #[inline(always)]
    pub fn rxfifoovfl(&self) -> RXFIFOOVFL_R {
        RXFIFOOVFL_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for FIFO Overflow Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_overflow_frames::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_FIFO_OVERFLOW_FRAMES_SPEC;
impl crate::RegisterSpec for RX_FIFO_OVERFLOW_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_overflow_frames::R`](R) reader structure"]
impl crate::Readable for RX_FIFO_OVERFLOW_FRAMES_SPEC {}
#[doc = "`reset()` method sets RX_FIFO_OVERFLOW_FRAMES to value 0"]
impl crate::Resettable for RX_FIFO_OVERFLOW_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
