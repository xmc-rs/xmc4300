#[doc = "Register `TIM1` reader"]
pub type R = crate::R<TIM1_SPEC>;
#[doc = "Register `TIM1` writer"]
pub type W = crate::W<TIM1_SPEC>;
#[doc = "Field `DAWE` reader - Days of Week Time Value"]
pub type DAWE_R = crate::FieldReader;
#[doc = "Field `DAWE` writer - Days of Week Time Value"]
pub type DAWE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MO` reader - Month Time Value"]
pub type MO_R = crate::FieldReader;
#[doc = "Field `MO` writer - Month Time Value"]
pub type MO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YE` reader - Year Time Value"]
pub type YE_R = crate::FieldReader<u16>;
#[doc = "Field `YE` writer - Year Time Value"]
pub type YE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - Days of Week Time Value"]
    #[inline(always)]
    pub fn dawe(&self) -> DAWE_R {
        DAWE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Month Time Value"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Year Time Value"]
    #[inline(always)]
    pub fn ye(&self) -> YE_R {
        YE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Days of Week Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn dawe(&mut self) -> DAWE_W<TIM1_SPEC> {
        DAWE_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Month Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn mo(&mut self) -> MO_W<TIM1_SPEC> {
        MO_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Year Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn ye(&mut self) -> YE_W<TIM1_SPEC> {
        YE_W::new(self, 16)
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
#[doc = "RTC Time Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_SPEC;
impl crate::RegisterSpec for TIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1::R`](R) reader structure"]
impl crate::Readable for TIM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1::W`](W) writer structure"]
impl crate::Writable for TIM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM1 to value 0"]
impl crate::Resettable for TIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
