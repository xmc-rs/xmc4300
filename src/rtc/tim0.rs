#[doc = "Register `TIM0` reader"]
pub type R = crate::R<TIM0_SPEC>;
#[doc = "Register `TIM0` writer"]
pub type W = crate::W<TIM0_SPEC>;
#[doc = "Field `SE` reader - Seconds Time Value"]
pub type SE_R = crate::FieldReader;
#[doc = "Field `SE` writer - Seconds Time Value"]
pub type SE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MI` reader - Minutes Time Value"]
pub type MI_R = crate::FieldReader;
#[doc = "Field `MI` writer - Minutes Time Value"]
pub type MI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HO` reader - Hours Time Value"]
pub type HO_R = crate::FieldReader;
#[doc = "Field `HO` writer - Hours Time Value"]
pub type HO_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DA` reader - Days Time Value"]
pub type DA_R = crate::FieldReader;
#[doc = "Field `DA` writer - Days Time Value"]
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    pub fn ho(&self) -> HO_R {
        HO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<TIM0_SPEC> {
        SE_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn mi(&mut self) -> MI_W<TIM0_SPEC> {
        MI_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn ho(&mut self) -> HO_W<TIM0_SPEC> {
        HO_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<TIM0_SPEC> {
        DA_W::new(self, 24)
    }
}
#[doc = "RTC Time Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM0_SPEC;
impl crate::RegisterSpec for TIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim0::R`](R) reader structure"]
impl crate::Readable for TIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim0::W`](W) writer structure"]
impl crate::Writable for TIM0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM0 to value 0"]
impl crate::Resettable for TIM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
