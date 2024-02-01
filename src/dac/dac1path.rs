#[doc = "Register `DAC1PATH` reader"]
pub type R = crate::R<DAC1PATH_SPEC>;
#[doc = "Register `DAC1PATH` writer"]
pub type W = crate::W<DAC1PATH_SPEC>;
#[doc = "Field `PAT6` reader - Pattern Number 6 for PATGEN of DAC1"]
pub type PAT6_R = crate::FieldReader;
#[doc = "Field `PAT6` writer - Pattern Number 6 for PATGEN of DAC1"]
pub type PAT6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT7` reader - Pattern Number 7 for PATGEN of DAC1"]
pub type PAT7_R = crate::FieldReader;
#[doc = "Field `PAT7` writer - Pattern Number 7 for PATGEN of DAC1"]
pub type PAT7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT8` reader - Pattern Number 8 for PATGEN of DAC1"]
pub type PAT8_R = crate::FieldReader;
#[doc = "Field `PAT8` writer - Pattern Number 8 for PATGEN of DAC1"]
pub type PAT8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat6(&self) -> PAT6_R {
        PAT6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat7(&self) -> PAT7_R {
        PAT7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat8(&self) -> PAT8_R {
        PAT8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat6(&mut self) -> PAT6_W<DAC1PATH_SPEC> {
        PAT6_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat7(&mut self) -> PAT7_W<DAC1PATH_SPEC> {
        PAT7_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat8(&mut self) -> PAT8_W<DAC1PATH_SPEC> {
        PAT8_W::new(self, 10)
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
#[doc = "DAC1 Higher Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1path::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1path::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC1PATH_SPEC;
impl crate::RegisterSpec for DAC1PATH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1path::R`](R) reader structure"]
impl crate::Readable for DAC1PATH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac1path::W`](W) writer structure"]
impl crate::Writable for DAC1PATH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1PATH to value 0x7fdd"]
impl crate::Resettable for DAC1PATH_SPEC {
    const RESET_VALUE: u32 = 0x7fdd;
}
