#[doc = "Register `FMMU_L_START_BIT` reader"]
pub type R = crate::R<FmmuLStartBitSpec>;
#[doc = "Field `L_START_BIT` reader - Logical starting bit that shall be mapped"]
pub type LStartBitR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Logical starting bit that shall be mapped"]
    #[inline(always)]
    pub fn l_start_bit(&self) -> LStartBitR {
        LStartBitR::new(self.bits & 7)
    }
}
#[doc = "Start bit FMMU 0 in logical address space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_start_bit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuLStartBitSpec;
impl crate::RegisterSpec for FmmuLStartBitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_l_start_bit::R`](R) reader structure"]
impl crate::Readable for FmmuLStartBitSpec {}
#[doc = "`reset()` method sets FMMU_L_START_BIT to value 0"]
impl crate::Resettable for FmmuLStartBitSpec {
    const RESET_VALUE: u8 = 0;
}
