#[doc = "Register `MPU_RBAR_A1` reader"]
pub type R = crate::R<MPU_RBAR_A1_SPEC>;
#[doc = "Register `MPU_RBAR_A1` writer"]
pub type W = crate::W<MPU_RBAR_A1_SPEC>;
#[doc = "Field `REGION` reader - MPU region field"]
pub type REGION_R = crate::FieldReader;
#[doc = "Field `REGION` writer - MPU region field"]
pub type REGION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VALID` reader - MPU Region Number valid bit"]
pub type VALID_R = crate::BitReader<VALID_A>;
#[doc = "MPU Region Number valid bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALID_A {
    #[doc = "0: MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    VALUE1 = 0,
    #[doc = "1: the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    VALUE2 = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::VALUE1,
            true => VALID_A::VALUE2,
        }
    }
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VALID_A::VALUE1
    }
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VALID_A::VALUE2
    }
}
#[doc = "Field `VALID` writer - MPU Region Number valid bit"]
pub type VALID_W<'a, REG> = crate::BitWriter<'a, REG, VALID_A>;
impl<'a, REG> VALID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VALID_A::VALUE1)
    }
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VALID_A::VALUE2)
    }
}
#[doc = "Field `ADDR` reader - Region base address field"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Region base address field"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:3 - MPU region field"]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - MPU Region Number valid bit"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 9:31 - Region base address field"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - MPU region field"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<MPU_RBAR_A1_SPEC> {
        REGION_W::new(self, 0)
    }
    #[doc = "Bit 4 - MPU Region Number valid bit"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<MPU_RBAR_A1_SPEC> {
        VALID_W::new(self, 4)
    }
    #[doc = "Bits 9:31 - Region base address field"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<MPU_RBAR_A1_SPEC> {
        ADDR_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MPU Region Base Address Register A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RBAR_A1_SPEC;
impl crate::RegisterSpec for MPU_RBAR_A1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rbar_a1::R`](R) reader structure"]
impl crate::Readable for MPU_RBAR_A1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rbar_a1::W`](W) writer structure"]
impl crate::Writable for MPU_RBAR_A1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RBAR_A1 to value 0"]
impl crate::Resettable for MPU_RBAR_A1_SPEC {
    const RESET_VALUE: u32 = 0;
}
