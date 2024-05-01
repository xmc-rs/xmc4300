#[doc = "Register `FMMU_L_START_ADR` reader"]
pub type R = crate::R<FmmuLStartAdrSpec>;
#[doc = "Field `L_START_ADDR` reader - Logical start address within the EtherCAT Address Space"]
pub type LStartAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Logical start address within the EtherCAT Address Space"]
    #[inline(always)]
    pub fn l_start_addr(&self) -> LStartAddrR {
        LStartAddrR::new(self.bits)
    }
}
#[doc = "Logical Start address FMMU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_start_adr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuLStartAdrSpec;
impl crate::RegisterSpec for FmmuLStartAdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmmu_l_start_adr::R`](R) reader structure"]
impl crate::Readable for FmmuLStartAdrSpec {}
#[doc = "`reset()` method sets FMMU_L_START_ADR to value 0"]
impl crate::Resettable for FmmuLStartAdrSpec {
    const RESET_VALUE: u32 = 0;
}
