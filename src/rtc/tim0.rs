#[doc = "Register `TIM0` reader"]
pub type R = crate::R<Tim0Spec>;
#[doc = "Register `TIM0` writer"]
pub type W = crate::W<Tim0Spec>;
#[doc = "Field `SE` reader - Seconds Time Value"]
pub type SeR = crate::FieldReader;
#[doc = "Field `SE` writer - Seconds Time Value"]
pub type SeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MI` reader - Minutes Time Value"]
pub type MiR = crate::FieldReader;
#[doc = "Field `MI` writer - Minutes Time Value"]
pub type MiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HO` reader - Hours Time Value"]
pub type HoR = crate::FieldReader;
#[doc = "Field `HO` writer - Hours Time Value"]
pub type HoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DA` reader - Days Time Value"]
pub type DaR = crate::FieldReader;
#[doc = "Field `DA` writer - Days Time Value"]
pub type DaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    pub fn mi(&self) -> MiR {
        MiR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    pub fn ho(&self) -> HoR {
        HoR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SeW<Tim0Spec> {
        SeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn mi(&mut self) -> MiW<Tim0Spec> {
        MiW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn ho(&mut self) -> HoW<Tim0Spec> {
        HoW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DaW<Tim0Spec> {
        DaW::new(self, 24)
    }
}
#[doc = "RTC Time Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim0Spec;
impl crate::RegisterSpec for Tim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim0::R`](R) reader structure"]
impl crate::Readable for Tim0Spec {}
#[doc = "`write(|w| ..)` method takes [`tim0::W`](W) writer structure"]
impl crate::Writable for Tim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM0 to value 0"]
impl crate::Resettable for Tim0Spec {
    const RESET_VALUE: u32 = 0;
}
