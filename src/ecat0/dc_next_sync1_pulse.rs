#[doc = "Register `DC_NEXT_SYNC1_PULSE[%s]` reader"]
pub struct R(crate::R<DC_NEXT_SYNC1_PULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_NEXT_SYNC1_PULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_NEXT_SYNC1_PULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_NEXT_SYNC1_PULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC_NEXT_SYNC1_PULSE` reader - System time of next SYNC1 pulse in ns"]
pub type DC_NEXT_SYNC1_PULSE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub fn dc_next_sync1_pulse(&self) -> DC_NEXT_SYNC1_PULSE_R {
        DC_NEXT_SYNC1_PULSE_R::new(self.bits)
    }
}
#[doc = "System time of next SYNC1 pulse in ns\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_next_sync1_pulse](index.html) module"]
pub struct DC_NEXT_SYNC1_PULSE_SPEC;
impl crate::RegisterSpec for DC_NEXT_SYNC1_PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_next_sync1_pulse::R](R) reader structure"]
impl crate::Readable for DC_NEXT_SYNC1_PULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_NEXT_SYNC1_PULSE[%s]
to value 0"]
impl crate::Resettable for DC_NEXT_SYNC1_PULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
