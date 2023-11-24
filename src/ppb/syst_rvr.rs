#[doc = "Register `SYST_RVR` reader"]
pub type R = crate::R<SYST_RVR_SPEC>;
#[doc = "Register `SYST_RVR` writer"]
pub type W = crate::W<SYST_RVR_SPEC>;
#[doc = "Field `RELOAD` reader - Reload Value"]
pub type RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Reload Value"]
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Reload Value"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reload Value"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<SYST_RVR_SPEC> {
        RELOAD_W::new(self, 0)
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
#[doc = "SysTick Reload Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_rvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_rvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYST_RVR_SPEC;
impl crate::RegisterSpec for SYST_RVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_rvr::R`](R) reader structure"]
impl crate::Readable for SYST_RVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syst_rvr::W`](W) writer structure"]
impl crate::Writable for SYST_RVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_RVR to value 0"]
impl crate::Resettable for SYST_RVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
