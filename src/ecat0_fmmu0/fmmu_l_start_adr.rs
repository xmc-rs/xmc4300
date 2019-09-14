#[doc = "Reader of register FMMU_L_START_ADR"]
pub type R = crate::R<u32, super::FMMU_L_START_ADR>;
#[doc = "Reader of field `L_START_ADDR`"]
pub type L_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Logical start address within the EtherCAT Address Space"]
    #[inline(always)]
    pub fn l_start_addr(&self) -> L_START_ADDR_R {
        L_START_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
