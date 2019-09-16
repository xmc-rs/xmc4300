#[doc = "Reader of register DC_SYNC1_STAT"]
pub type R = crate::R<u8, super::DC_SYNC1_STAT>;
#[doc = "Reader of field `S1_STATE`"]
pub type S1_STATE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SYNC1 state for Acknowledge mode"]
    #[inline(always)]
    pub fn s1_state(&self) -> S1_STATE_R {
        S1_STATE_R::new((self.bits & 0x01) != 0)
    }
}
