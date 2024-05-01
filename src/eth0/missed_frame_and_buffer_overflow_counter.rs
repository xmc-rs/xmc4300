#[doc = "Register `MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER` reader"]
pub type R = crate::R<MissedFrameAndBufferOverflowCounterSpec>;
#[doc = "Field `MISFRMCNT` reader - This field indicates the number of frames missed by the controller because of the RAM Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
pub type MisfrmcntR = crate::FieldReader<u16>;
#[doc = "Field `MISCNTOVF` reader - Overflow bit for Missed Frame Counter"]
pub type MiscntovfR = crate::BitReader;
#[doc = "Field `OVFFRMCNT` reader - This field indicates the number of frames missed by the application. The counter is cleared when this register is read."]
pub type OvffrmcntR = crate::FieldReader<u16>;
#[doc = "Field `OVFCNTOVF` reader - Overflow bit for FIFO Overflow Counter"]
pub type OvfcntovfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the RAM Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MisfrmcntR {
        MisfrmcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MiscntovfR {
        MiscntovfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OvffrmcntR {
        OvffrmcntR::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OvfcntovfR {
        OvfcntovfR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`missed_frame_and_buffer_overflow_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MissedFrameAndBufferOverflowCounterSpec;
impl crate::RegisterSpec for MissedFrameAndBufferOverflowCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`missed_frame_and_buffer_overflow_counter::R`](R) reader structure"]
impl crate::Readable for MissedFrameAndBufferOverflowCounterSpec {}
#[doc = "`reset()` method sets MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER to value 0"]
impl crate::Resettable for MissedFrameAndBufferOverflowCounterSpec {
    const RESET_VALUE: u32 = 0;
}
