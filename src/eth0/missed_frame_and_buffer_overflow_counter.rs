#[doc = "Register `MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER` reader"]
pub type R = crate::R<MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>;
#[doc = "Field `MISFRMCNT` reader - This field indicates the number of frames missed by the controller because of the RAM Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
pub type MISFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `MISCNTOVF` reader - Overflow bit for Missed Frame Counter"]
pub type MISCNTOVF_R = crate::BitReader;
#[doc = "Field `OVFFRMCNT` reader - This field indicates the number of frames missed by the application. The counter is cleared when this register is read."]
pub type OVFFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `OVFCNTOVF` reader - Overflow bit for FIFO Overflow Counter"]
pub type OVFCNTOVF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the RAM Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MISFRMCNT_R {
        MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OVFFRMCNT_R {
        OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`missed_frame_and_buffer_overflow_counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC;
impl crate::RegisterSpec for MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`missed_frame_and_buffer_overflow_counter::R`](R) reader structure"]
impl crate::Readable for MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC {}
#[doc = "`reset()` method sets MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER to value 0"]
impl crate::Resettable for MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
