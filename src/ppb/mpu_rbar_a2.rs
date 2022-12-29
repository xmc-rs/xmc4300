#[doc = "Register `MPU_RBAR_A2` reader"]
pub struct R(crate::R<MPU_RBAR_A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RBAR_A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RBAR_A2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RBAR_A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RBAR_A2` writer"]
pub struct W(crate::W<MPU_RBAR_A2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RBAR_A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MPU_RBAR_A2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RBAR_A2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION` reader - MPU region field"]
pub type REGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION` writer - MPU region field"]
pub type REGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RBAR_A2_SPEC, u8, u8, 4, O>;
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
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::VALUE1,
            true => VALID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VALID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VALID_A::VALUE2
    }
}
#[doc = "Field `VALID` writer - MPU Region Number valid bit"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RBAR_A2_SPEC, VALID_A, O>;
impl<'a, const O: u8> VALID_W<'a, O> {
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VALID_A::VALUE1)
    }
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VALID_A::VALUE2)
    }
}
#[doc = "Field `ADDR` reader - Region base address field"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Region base address field"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RBAR_A2_SPEC, u32, u32, 23, O>;
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
    pub fn region(&mut self) -> REGION_W<0> {
        REGION_W::new(self)
    }
    #[doc = "Bit 4 - MPU Region Number valid bit"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<4> {
        VALID_W::new(self)
    }
    #[doc = "Bits 9:31 - Region base address field"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<9> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Region Base Address Register A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar_a2](index.html) module"]
pub struct MPU_RBAR_A2_SPEC;
impl crate::RegisterSpec for MPU_RBAR_A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rbar_a2::R](R) reader structure"]
impl crate::Readable for MPU_RBAR_A2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a2::W](W) writer structure"]
impl crate::Writable for MPU_RBAR_A2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RBAR_A2 to value 0"]
impl crate::Resettable for MPU_RBAR_A2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
