#[doc = "Register `DC_NEXT_SYNC1_PULSE[%s]` reader"]
pub type R = crate::R<DcNextSync1PulseSpec>;
#[doc = "Field `DC_NEXT_SYNC1_PULSE` reader - System time of next SYNC1 pulse in ns"]
pub type DcNextSync1PulseR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub fn dc_next_sync1_pulse(&self) -> DcNextSync1PulseR {
        DcNextSync1PulseR::new(self.bits)
    }
}
#[doc = "System time of next SYNC1 pulse in ns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_next_sync1_pulse::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcNextSync1PulseSpec;
impl crate::RegisterSpec for DcNextSync1PulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_next_sync1_pulse::R`](R) reader structure"]
impl crate::Readable for DcNextSync1PulseSpec {}
#[doc = "`reset()` method sets DC_NEXT_SYNC1_PULSE[%s]
to value 0"]
impl crate::Resettable for DcNextSync1PulseSpec {
    const RESET_VALUE: u32 = 0;
}
