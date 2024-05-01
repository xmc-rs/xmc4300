#[doc = "Register `MPU_TYPE` reader"]
pub type R = crate::R<MpuTypeSpec>;
#[doc = "Field `SEPARATE` reader - Support for unified or separate instruction and date memory maps"]
pub type SeparateR = crate::BitReader;
#[doc = "Field `DREGION` reader - Number of supported MPU data regions"]
pub type DregionR = crate::FieldReader;
#[doc = "Field `IREGION` reader - Number of supported MPU instruction regions"]
pub type IregionR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Support for unified or separate instruction and date memory maps"]
    #[inline(always)]
    pub fn separate(&self) -> SeparateR {
        SeparateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Number of supported MPU data regions"]
    #[inline(always)]
    pub fn dregion(&self) -> DregionR {
        DregionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of supported MPU instruction regions"]
    #[inline(always)]
    pub fn iregion(&self) -> IregionR {
        IregionR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "MPU Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_type::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuTypeSpec;
impl crate::RegisterSpec for MpuTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_type::R`](R) reader structure"]
impl crate::Readable for MpuTypeSpec {}
#[doc = "`reset()` method sets MPU_TYPE to value 0x0800"]
impl crate::Resettable for MpuTypeSpec {
    const RESET_VALUE: u32 = 0x0800;
}
