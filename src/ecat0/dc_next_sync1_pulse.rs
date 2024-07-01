#[doc = "Register `DC_NEXT_SYNC1_PULSE[%s]` reader"]
pub type R = crate::R<DC_NEXT_SYNC1_PULSE_SPEC>;
#[doc = "Field `DC_NEXT_SYNC1_PULSE` reader - System time of next SYNC1 pulse in ns"]
pub type DC_NEXT_SYNC1_PULSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub fn dc_next_sync1_pulse(&self) -> DC_NEXT_SYNC1_PULSE_R {
        DC_NEXT_SYNC1_PULSE_R::new(self.bits)
    }
}
#[doc = "System time of next SYNC1 pulse in ns\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_next_sync1_pulse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_NEXT_SYNC1_PULSE_SPEC;
impl crate::RegisterSpec for DC_NEXT_SYNC1_PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_next_sync1_pulse::R`](R) reader structure"]
impl crate::Readable for DC_NEXT_SYNC1_PULSE_SPEC {}
#[doc = "`reset()` method sets DC_NEXT_SYNC1_PULSE[%s]
to value 0"]
impl crate::Resettable for DC_NEXT_SYNC1_PULSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
