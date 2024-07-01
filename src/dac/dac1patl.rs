#[doc = "Register `DAC1PATL` reader"]
pub type R = crate::R<DAC1PATL_SPEC>;
#[doc = "Register `DAC1PATL` writer"]
pub type W = crate::W<DAC1PATL_SPEC>;
#[doc = "Field `PAT0` reader - Pattern Number 0 for PATGEN of DAC1"]
pub type PAT0_R = crate::FieldReader;
#[doc = "Field `PAT0` writer - Pattern Number 0 for PATGEN of DAC1"]
pub type PAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT1` reader - Pattern Number 1 for PATGEN of DAC1"]
pub type PAT1_R = crate::FieldReader;
#[doc = "Field `PAT1` writer - Pattern Number 1 for PATGEN of DAC1"]
pub type PAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT2` reader - Pattern Number 2 for PATGEN of DAC1"]
pub type PAT2_R = crate::FieldReader;
#[doc = "Field `PAT2` writer - Pattern Number 2 for PATGEN of DAC1"]
pub type PAT2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT3` reader - Pattern Number 3 for PATGEN of DAC1"]
pub type PAT3_R = crate::FieldReader;
#[doc = "Field `PAT3` writer - Pattern Number 3 for PATGEN of DAC1"]
pub type PAT3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT4` reader - Pattern Number 4 for PATGEN of DAC1"]
pub type PAT4_R = crate::FieldReader;
#[doc = "Field `PAT4` writer - Pattern Number 4 for PATGEN of DAC1"]
pub type PAT4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT5` reader - Pattern Number 5 for PATGEN of DAC1"]
pub type PAT5_R = crate::FieldReader;
#[doc = "Field `PAT5` writer - Pattern Number 5 for PATGEN of DAC1"]
pub type PAT5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat0(&self) -> PAT0_R {
        PAT0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat1(&self) -> PAT1_R {
        PAT1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat2(&self) -> PAT2_R {
        PAT2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat3(&self) -> PAT3_R {
        PAT3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat4(&self) -> PAT4_R {
        PAT4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat5(&self) -> PAT5_R {
        PAT5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat0(&mut self) -> PAT0_W<DAC1PATL_SPEC> {
        PAT0_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat1(&mut self) -> PAT1_W<DAC1PATL_SPEC> {
        PAT1_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat2(&mut self) -> PAT2_W<DAC1PATL_SPEC> {
        PAT2_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat3(&mut self) -> PAT3_W<DAC1PATL_SPEC> {
        PAT3_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat4(&mut self) -> PAT4_W<DAC1PATL_SPEC> {
        PAT4_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat5(&mut self) -> PAT5_W<DAC1PATL_SPEC> {
        PAT5_W::new(self, 25)
    }
}
#[doc = "DAC1 Lower Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1patl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac1patl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC1PATL_SPEC;
impl crate::RegisterSpec for DAC1PATL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1patl::R`](R) reader structure"]
impl crate::Readable for DAC1PATL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac1patl::W`](W) writer structure"]
impl crate::Writable for DAC1PATL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1PATL to value 0x3568_b0c0"]
impl crate::Resettable for DAC1PATL_SPEC {
    const RESET_VALUE: u32 = 0x3568_b0c0;
}
