#[doc = "Register `FMMU_NUM` reader"]
pub type R = crate::R<FmmuNumSpec>;
#[doc = "Field `NUM_FMMU` reader - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
pub type NumFmmuR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
    #[inline(always)]
    pub fn num_fmmu(&self) -> NumFmmuR {
        NumFmmuR::new(self.bits)
    }
}
#[doc = "FMMUs Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuNumSpec;
impl crate::RegisterSpec for FmmuNumSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_num::R`](R) reader structure"]
impl crate::Readable for FmmuNumSpec {}
#[doc = "`reset()` method sets FMMU_NUM to value 0x08"]
impl crate::Resettable for FmmuNumSpec {
    const RESET_VALUE: u8 = 0x08;
}
