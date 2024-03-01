#[doc = "Register `FMMU_L_STOP_BIT` reader"]
pub type R = crate::R<FmmuLStopBitSpec>;
#[doc = "Field `L_STOP_BIT` reader - Last logical bit that shall be mapped"]
pub type LStopBitR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last logical bit that shall be mapped"]
    #[inline(always)]
    pub fn l_stop_bit(&self) -> LStopBitR {
        LStopBitR::new(self.bits & 7)
    }
}
#[doc = "Stop bit FMMU 0 in logical address space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_l_stop_bit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuLStopBitSpec;
impl crate::RegisterSpec for FmmuLStopBitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_l_stop_bit::R`](R) reader structure"]
impl crate::Readable for FmmuLStopBitSpec {}
#[doc = "`reset()` method sets FMMU_L_STOP_BIT to value 0"]
impl crate::Resettable for FmmuLStopBitSpec {
    const RESET_VALUE: u8 = 0;
}
