#[doc = "Reader of register DC_LATCH0_TIME_NEG[%s]"]
pub type R = crate::R<u32, super::DC_LATCH0_TIME_NEG>;
#[doc = "Reader of field `DC_LATCH0_TIME_NEG`"]
pub type DC_LATCH0_TIME_NEG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the negative edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_neg(&self) -> DC_LATCH0_TIME_NEG_R {
        DC_LATCH0_TIME_NEG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
