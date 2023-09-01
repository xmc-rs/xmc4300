#[doc = "Register `SYST_RVR` reader"]
pub type R = crate::R<SYST_RVR_SPEC>;
#[doc = "Register `SYST_RVR` writer"]
pub type W = crate::W<SYST_RVR_SPEC>;
#[doc = "Field `RELOAD` reader - Reload Value"]
pub type RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Reload Value"]
pub type RELOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
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
    pub fn reload(&mut self) -> RELOAD_W<SYST_RVR_SPEC, 0> {
        RELOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
