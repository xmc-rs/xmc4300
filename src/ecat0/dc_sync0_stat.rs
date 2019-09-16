#[doc = "Reader of register DC_SYNC0_STAT"]
pub type R = crate::R<u8, super::DC_SYNC0_STAT>;
#[doc = "Reader of field `S0_STATE`"]
pub type S0_STATE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SYNC0 state for Acknowledge mode"]
    #[inline(always)]
    pub fn s0_state(&self) -> S0_STATE_R {
        S0_STATE_R::new((self.bits & 0x01) != 0)
    }
}
