#[doc = "Reader of register DC_NEXT_SYNC1_PULSE[%s]"]
pub type R = crate::R<u32, super::DC_NEXT_SYNC1_PULSE>;
#[doc = "Reader of field `DC_NEXT_SYNC1_PULSE`"]
pub type DC_NEXT_SYNC1_PULSE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub fn dc_next_sync1_pulse(&self) -> DC_NEXT_SYNC1_PULSE_R {
        DC_NEXT_SYNC1_PULSE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
