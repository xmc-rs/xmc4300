#[doc = "Register `FMMU_P_START_BIT` reader"]
pub type R = crate::R<FmmuPStartBitSpec>;
#[doc = "Field `P_START_BIT` reader - Physical starting bit as target of logical start bit mapping"]
pub type PStartBitR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Physical starting bit as target of logical start bit mapping"]
    #[inline(always)]
    pub fn p_start_bit(&self) -> PStartBitR {
        PStartBitR::new(self.bits & 7)
    }
}
#[doc = "Ph0sical Start bit FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_p_start_bit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuPStartBitSpec;
impl crate::RegisterSpec for FmmuPStartBitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_p_start_bit::R`](R) reader structure"]
impl crate::Readable for FmmuPStartBitSpec {}
#[doc = "`reset()` method sets FMMU_P_START_BIT to value 0"]
impl crate::Resettable for FmmuPStartBitSpec {
    const RESET_VALUE: u8 = 0;
}
