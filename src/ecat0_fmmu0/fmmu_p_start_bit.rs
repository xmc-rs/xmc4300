#[doc = "Reader of register FMMU_P_START_BIT"]
pub type R = crate::R<u8, super::FMMU_P_START_BIT>;
#[doc = "Reader of field `P_START_BIT`"]
pub type P_START_BIT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Physical starting bit as target of logical start bit mapping"]
    #[inline(always)]
    pub fn p_start_bit(&self) -> P_START_BIT_R {
        P_START_BIT_R::new((self.bits & 0x07) as u8)
    }
}
