#[doc = "Register `MPU_RASR_A2` reader"]
pub type R = crate::R<MPU_RASR_A2_SPEC>;
#[doc = "Register `MPU_RASR_A2` writer"]
pub type W = crate::W<MPU_RASR_A2_SPEC>;
#[doc = "Field `ENABLE` reader - Region enable bit."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Region enable bit."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE` reader - MPU protection region size"]
pub type SIZE_R = crate::FieldReader;
#[doc = "Field `SIZE` writer - MPU protection region size"]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SRD` reader - Subregion disable bits"]
pub type SRD_R = crate::FieldReader<SRD_A>;
#[doc = "Subregion disable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRD_A {
    #[doc = "0: corresponding sub-region is enabled"]
    VALUE1 = 0,
    #[doc = "1: corresponding sub-region is disabled"]
    VALUE2 = 1,
}
impl From<SRD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRD_A {
    type Ux = u8;
}
impl SRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRD_A> {
        match self.bits {
            0 => Some(SRD_A::VALUE1),
            1 => Some(SRD_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "corresponding sub-region is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRD_A::VALUE1
    }
    #[doc = "corresponding sub-region is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRD_A::VALUE2
    }
}
#[doc = "Field `SRD` writer - Subregion disable bits"]
pub type SRD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SRD_A>;
impl<'a, REG> SRD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "corresponding sub-region is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRD_A::VALUE1)
    }
    #[doc = "corresponding sub-region is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRD_A::VALUE2)
    }
}
#[doc = "Field `B` reader - Memory access attribute"]
pub type B_R = crate::BitReader;
#[doc = "Field `B` writer - Memory access attribute"]
pub type B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C` reader - Memory access attribute"]
pub type C_R = crate::BitReader;
#[doc = "Field `C` writer - Memory access attribute"]
pub type C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Shareable bit"]
pub type S_R = crate::BitReader;
#[doc = "Field `S` writer - Shareable bit"]
pub type S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEX` reader - Memory access attribute"]
pub type TEX_R = crate::FieldReader;
#[doc = "Field `TEX` writer - Memory access attribute"]
pub type TEX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AP` reader - Access permission field"]
pub type AP_R = crate::FieldReader;
#[doc = "Field `AP` writer - Access permission field"]
pub type AP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XN` reader - Instruction access disable bit"]
pub type XN_R = crate::BitReader<XN_A>;
#[doc = "Instruction access disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XN_A {
    #[doc = "0: instruction fetches enabled"]
    VALUE1 = 0,
    #[doc = "1: instruction fetches disabled."]
    VALUE2 = 1,
}
impl From<XN_A> for bool {
    #[inline(always)]
    fn from(variant: XN_A) -> Self {
        variant as u8 != 0
    }
}
impl XN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XN_A {
        match self.bits {
            false => XN_A::VALUE1,
            true => XN_A::VALUE2,
        }
    }
    #[doc = "instruction fetches enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == XN_A::VALUE1
    }
    #[doc = "instruction fetches disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == XN_A::VALUE2
    }
}
#[doc = "Field `XN` writer - Instruction access disable bit"]
pub type XN_W<'a, REG> = crate::BitWriter<'a, REG, XN_A>;
impl<'a, REG> XN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "instruction fetches enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(XN_A::VALUE1)
    }
    #[doc = "instruction fetches disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(XN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    pub fn tex(&self) -> TEX_R {
        TEX_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    pub fn xn(&self) -> XN_R {
        XN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MPU_RASR_A2_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<MPU_RASR_A2_SPEC> {
        SIZE_W::new(self, 1)
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    #[must_use]
    pub fn srd(&mut self) -> SRD_W<MPU_RASR_A2_SPEC> {
        SRD_W::new(self, 8)
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<MPU_RASR_A2_SPEC> {
        B_W::new(self, 16)
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> C_W<MPU_RASR_A2_SPEC> {
        C_W::new(self, 17)
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<MPU_RASR_A2_SPEC> {
        S_W::new(self, 18)
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn tex(&mut self) -> TEX_W<MPU_RASR_A2_SPEC> {
        TEX_W::new(self, 19)
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> AP_W<MPU_RASR_A2_SPEC> {
        AP_W::new(self, 24)
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XN_W<MPU_RASR_A2_SPEC> {
        XN_W::new(self, 28)
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
#[doc = "MPU Region Attribute and Size Register A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RASR_A2_SPEC;
impl crate::RegisterSpec for MPU_RASR_A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rasr_a2::R`](R) reader structure"]
impl crate::Readable for MPU_RASR_A2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rasr_a2::W`](W) writer structure"]
impl crate::Writable for MPU_RASR_A2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RASR_A2 to value 0"]
impl crate::Resettable for MPU_RASR_A2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
