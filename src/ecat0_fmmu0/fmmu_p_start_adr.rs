#[doc = "Register `FMMU_P_START_ADR` reader"]
pub type R = crate::R<FmmuPStartAdrSpec>;
#[doc = "Field `P_START_ADDR` reader - Physical Start Address"]
pub type PStartAddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Physical Start Address"]
    #[inline(always)]
    pub fn p_start_addr(&self) -> PStartAddrR {
        PStartAddrR::new(self.bits)
    }
}
#[doc = "Ph0sical Start address FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_p_start_adr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuPStartAdrSpec;
impl crate::RegisterSpec for FmmuPStartAdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fmmu_p_start_adr::R`](R) reader structure"]
impl crate::Readable for FmmuPStartAdrSpec {}
#[doc = "`reset()` method sets FMMU_P_START_ADR to value 0"]
impl crate::Resettable for FmmuPStartAdrSpec {
    const RESET_VALUE: u16 = 0;
}
