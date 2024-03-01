#[doc = "Register `CLRSR` writer"]
pub type W = crate::W<ClrsrSpec>;
#[doc = "Field `RPSE` writer - Periodic Seconds Interrupt Clear"]
pub type RpseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPMI` writer - Periodic Minutes Interrupt Clear"]
pub type RpmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPHO` writer - Periodic Hours Interrupt Clear"]
pub type RphoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPDA` writer - Periodic Days Interrupt Clear"]
pub type RpdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPMO` writer - Periodic Months Interrupt Clear"]
pub type RpmoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPYE` writer - Periodic Years Interrupt Clear"]
pub type RpyeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAI` writer - Alarm Interrupt Clear"]
pub type RaiW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpse(&mut self) -> RpseW<ClrsrSpec> {
        RpseW::new(self, 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpmi(&mut self) -> RpmiW<ClrsrSpec> {
        RpmiW::new(self, 1)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpho(&mut self) -> RphoW<ClrsrSpec> {
        RphoW::new(self, 2)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpda(&mut self) -> RpdaW<ClrsrSpec> {
        RpdaW::new(self, 3)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpmo(&mut self) -> RpmoW<ClrsrSpec> {
        RpmoW::new(self, 5)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpye(&mut self) -> RpyeW<ClrsrSpec> {
        RpyeW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rai(&mut self) -> RaiW<ClrsrSpec> {
        RaiW::new(self, 8)
    }
}
#[doc = "RTC Clear Service Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrsrSpec;
impl crate::RegisterSpec for ClrsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clrsr::W`](W) writer structure"]
impl crate::Writable for ClrsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRSR to value 0"]
impl crate::Resettable for ClrsrSpec {
    const RESET_VALUE: u32 = 0;
}
