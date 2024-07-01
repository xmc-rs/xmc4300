#[doc = "Register `CLRSR` writer"]
pub type W = crate::W<CLRSR_SPEC>;
#[doc = "Field `RPSE` writer - Periodic Seconds Interrupt Clear"]
pub type RPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPMI` writer - Periodic Minutes Interrupt Clear"]
pub type RPMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPHO` writer - Periodic Hours Interrupt Clear"]
pub type RPHO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPDA` writer - Periodic Days Interrupt Clear"]
pub type RPDA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPMO` writer - Periodic Months Interrupt Clear"]
pub type RPMO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPYE` writer - Periodic Years Interrupt Clear"]
pub type RPYE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAI` writer - Alarm Interrupt Clear"]
pub type RAI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpse(&mut self) -> RPSE_W<CLRSR_SPEC> {
        RPSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpmi(&mut self) -> RPMI_W<CLRSR_SPEC> {
        RPMI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpho(&mut self) -> RPHO_W<CLRSR_SPEC> {
        RPHO_W::new(self, 2)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpda(&mut self) -> RPDA_W<CLRSR_SPEC> {
        RPDA_W::new(self, 3)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpmo(&mut self) -> RPMO_W<CLRSR_SPEC> {
        RPMO_W::new(self, 5)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpye(&mut self) -> RPYE_W<CLRSR_SPEC> {
        RPYE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rai(&mut self) -> RAI_W<CLRSR_SPEC> {
        RAI_W::new(self, 8)
    }
}
#[doc = "RTC Clear Service Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRSR_SPEC;
impl crate::RegisterSpec for CLRSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clrsr::W`](W) writer structure"]
impl crate::Writable for CLRSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRSR to value 0"]
impl crate::Resettable for CLRSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
