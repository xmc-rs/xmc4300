#[doc = "Reader of register FMMU_P_START_ADR"]
pub type R = crate::R<u16, super::FMMU_P_START_ADR>;
#[doc = "Reader of field `P_START_ADDR`"]
pub type P_START_ADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Physical Start Address"]
    #[inline(always)]
    pub fn p_start_addr(&self) -> P_START_ADDR_R {
        P_START_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
