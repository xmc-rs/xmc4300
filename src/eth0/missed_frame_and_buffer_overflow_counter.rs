#[doc = "Register `MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER` reader"]
pub struct R(crate::R<MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MISFRMCNT` reader - This field indicates the number of frames missed by the controller because of the RAM Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
pub struct MISFRMCNT_R(crate::FieldReader<u16, u16>);
impl MISFRMCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        MISFRMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISFRMCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISCNTOVF` reader - Overflow bit for Missed Frame Counter"]
pub struct MISCNTOVF_R(crate::FieldReader<bool, bool>);
impl MISCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFFRMCNT` reader - This field indicates the number of frames missed by the application. The counter is cleared when this register is read."]
pub struct OVFFRMCNT_R(crate::FieldReader<u16, u16>);
impl OVFFRMCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        OVFFRMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFFRMCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFCNTOVF` reader - Overflow bit for FIFO Overflow Counter"]
pub struct OVFCNTOVF_R(crate::FieldReader<bool, bool>);
impl OVFCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVFCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the RAM Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MISFRMCNT_R {
        MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OVFFRMCNT_R {
        OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
#[doc = "Missed Frame and Buffer Overflow Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [missed_frame_and_buffer_overflow_counter](index.html) module"]
pub struct MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC;
impl crate::RegisterSpec for MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [missed_frame_and_buffer_overflow_counter::R](R) reader structure"]
impl crate::Readable for MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER to value 0"]
impl crate::Resettable for MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
