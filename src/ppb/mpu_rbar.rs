#[doc = "Register `MPU_RBAR` reader"]
pub type R = crate::R<MpuRbarSpec>;
#[doc = "Register `MPU_RBAR` writer"]
pub type W = crate::W<MpuRbarSpec>;
#[doc = "Field `REGION` reader - MPU region field"]
pub type RegionR = crate::FieldReader;
#[doc = "Field `REGION` writer - MPU region field"]
pub type RegionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "MPU Region Number valid bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valid {
    #[doc = "0: MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    Value1 = 0,
    #[doc = "1: the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    Value2 = 1,
}
impl From<Valid> for bool {
    #[inline(always)]
    fn from(variant: Valid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - MPU Region Number valid bit"]
pub type ValidR = crate::BitReader<Valid>;
impl ValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Valid {
        match self.bits {
            false => Valid::Value1,
            true => Valid::Value2,
        }
    }
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Valid::Value1
    }
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Valid::Value2
    }
}
#[doc = "Field `VALID` writer - MPU Region Number valid bit"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG, Valid>;
impl<'a, REG> ValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::Value1)
    }
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::Value2)
    }
}
#[doc = "Field `ADDR` reader - Region base address field"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Region base address field"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:3 - MPU region field"]
    #[inline(always)]
    pub fn region(&self) -> RegionR {
        RegionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - MPU Region Number valid bit"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 9:31 - Region base address field"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - MPU region field"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> RegionW<MpuRbarSpec> {
        RegionW::new(self, 0)
    }
    #[doc = "Bit 4 - MPU Region Number valid bit"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<MpuRbarSpec> {
        ValidW::new(self, 4)
    }
    #[doc = "Bits 9:31 - Region base address field"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<MpuRbarSpec> {
        AddrW::new(self, 9)
    }
}
#[doc = "MPU Region Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRbarSpec;
impl crate::RegisterSpec for MpuRbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rbar::R`](R) reader structure"]
impl crate::Readable for MpuRbarSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rbar::W`](W) writer structure"]
impl crate::Writable for MpuRbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RBAR to value 0"]
impl crate::Resettable for MpuRbarSpec {
    const RESET_VALUE: u32 = 0;
}
