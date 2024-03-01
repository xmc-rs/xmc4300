#[doc = "Register `PWRMON` reader"]
pub type R = crate::R<PwrmonSpec>;
#[doc = "Register `PWRMON` writer"]
pub type W = crate::W<PwrmonSpec>;
#[doc = "Field `THRS` reader - Threshold"]
pub type ThrsR = crate::FieldReader;
#[doc = "Field `THRS` writer - Threshold"]
pub type ThrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INTV` reader - Interval"]
pub type IntvR = crate::FieldReader;
#[doc = "Field `INTV` writer - Interval"]
pub type IntvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENB` reader - Enable"]
pub type EnbR = crate::BitReader;
#[doc = "Field `ENB` writer - Enable"]
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    pub fn thrs(&self) -> ThrsR {
        ThrsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    pub fn intv(&self) -> IntvR {
        IntvR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thrs(&mut self) -> ThrsW<PwrmonSpec> {
        ThrsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    #[must_use]
    pub fn intv(&mut self) -> IntvW<PwrmonSpec> {
        IntvW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> EnbW<PwrmonSpec> {
        EnbW::new(self, 16)
    }
}
#[doc = "Power Monitor Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrmon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrmon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrmonSpec;
impl crate::RegisterSpec for PwrmonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrmon::R`](R) reader structure"]
impl crate::Readable for PwrmonSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrmon::W`](W) writer structure"]
impl crate::Writable for PwrmonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRMON to value 0"]
impl crate::Resettable for PwrmonSpec {
    const RESET_VALUE: u32 = 0;
}
