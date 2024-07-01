#[doc = "Register `DAC0PATH` reader"]
pub type R = crate::R<DAC0PATH_SPEC>;
#[doc = "Register `DAC0PATH` writer"]
pub type W = crate::W<DAC0PATH_SPEC>;
#[doc = "Field `PAT6` reader - Pattern Number 6 for PATGEN of DAC0"]
pub type PAT6_R = crate::FieldReader;
#[doc = "Field `PAT6` writer - Pattern Number 6 for PATGEN of DAC0"]
pub type PAT6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT7` reader - Pattern Number 7 for PATGEN of DAC0"]
pub type PAT7_R = crate::FieldReader;
#[doc = "Field `PAT7` writer - Pattern Number 7 for PATGEN of DAC0"]
pub type PAT7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT8` reader - Pattern Number 8 for PATGEN of DAC0"]
pub type PAT8_R = crate::FieldReader;
#[doc = "Field `PAT8` writer - Pattern Number 8 for PATGEN of DAC0"]
pub type PAT8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat6(&self) -> PAT6_R {
        PAT6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat7(&self) -> PAT7_R {
        PAT7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat8(&self) -> PAT8_R {
        PAT8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat6(&mut self) -> PAT6_W<DAC0PATH_SPEC> {
        PAT6_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat7(&mut self) -> PAT7_W<DAC0PATH_SPEC> {
        PAT7_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat8(&mut self) -> PAT8_W<DAC0PATH_SPEC> {
        PAT8_W::new(self, 10)
    }
}
#[doc = "DAC0 Higher Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0path::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac0path::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC0PATH_SPEC;
impl crate::RegisterSpec for DAC0PATH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0path::R`](R) reader structure"]
impl crate::Readable for DAC0PATH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac0path::W`](W) writer structure"]
impl crate::Writable for DAC0PATH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC0PATH to value 0x7fdd"]
impl crate::Resettable for DAC0PATH_SPEC {
    const RESET_VALUE: u32 = 0x7fdd;
}
