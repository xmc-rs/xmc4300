#[doc = "Register `DAC1PATL` reader"]
pub type R = crate::R<Dac1patlSpec>;
#[doc = "Register `DAC1PATL` writer"]
pub type W = crate::W<Dac1patlSpec>;
#[doc = "Field `PAT0` reader - Pattern Number 0 for PATGEN of DAC1"]
pub type Pat0R = crate::FieldReader;
#[doc = "Field `PAT0` writer - Pattern Number 0 for PATGEN of DAC1"]
pub type Pat0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT1` reader - Pattern Number 1 for PATGEN of DAC1"]
pub type Pat1R = crate::FieldReader;
#[doc = "Field `PAT1` writer - Pattern Number 1 for PATGEN of DAC1"]
pub type Pat1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT2` reader - Pattern Number 2 for PATGEN of DAC1"]
pub type Pat2R = crate::FieldReader;
#[doc = "Field `PAT2` writer - Pattern Number 2 for PATGEN of DAC1"]
pub type Pat2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT3` reader - Pattern Number 3 for PATGEN of DAC1"]
pub type Pat3R = crate::FieldReader;
#[doc = "Field `PAT3` writer - Pattern Number 3 for PATGEN of DAC1"]
pub type Pat3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT4` reader - Pattern Number 4 for PATGEN of DAC1"]
pub type Pat4R = crate::FieldReader;
#[doc = "Field `PAT4` writer - Pattern Number 4 for PATGEN of DAC1"]
pub type Pat4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT5` reader - Pattern Number 5 for PATGEN of DAC1"]
pub type Pat5R = crate::FieldReader;
#[doc = "Field `PAT5` writer - Pattern Number 5 for PATGEN of DAC1"]
pub type Pat5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat0(&self) -> Pat0R {
        Pat0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat1(&self) -> Pat1R {
        Pat1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat2(&self) -> Pat2R {
        Pat2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat3(&self) -> Pat3R {
        Pat3R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat4(&self) -> Pat4R {
        Pat4R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat5(&self) -> Pat5R {
        Pat5R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat0(&mut self) -> Pat0W<Dac1patlSpec> {
        Pat0W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat1(&mut self) -> Pat1W<Dac1patlSpec> {
        Pat1W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat2(&mut self) -> Pat2W<Dac1patlSpec> {
        Pat2W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat3(&mut self) -> Pat3W<Dac1patlSpec> {
        Pat3W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat4(&mut self) -> Pat4W<Dac1patlSpec> {
        Pat4W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat5(&mut self) -> Pat5W<Dac1patlSpec> {
        Pat5W::new(self, 25)
    }
}
#[doc = "DAC1 Lower Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1patl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1patl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1patlSpec;
impl crate::RegisterSpec for Dac1patlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1patl::R`](R) reader structure"]
impl crate::Readable for Dac1patlSpec {}
#[doc = "`write(|w| ..)` method takes [`dac1patl::W`](W) writer structure"]
impl crate::Writable for Dac1patlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1PATL to value 0x3568_b0c0"]
impl crate::Resettable for Dac1patlSpec {
    const RESET_VALUE: u32 = 0x3568_b0c0;
}
