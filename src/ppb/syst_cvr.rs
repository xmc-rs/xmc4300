#[doc = "Register `SYST_CVR` reader"]
pub type R = crate::R<SYST_CVR_SPEC>;
#[doc = "Register `SYST_CVR` writer"]
pub type W = crate::W<SYST_CVR_SPEC>;
#[doc = "Field `CURRENT` reader - Current Value"]
pub type CURRENT_R = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - Current Value"]
pub type CURRENT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Current Value"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Current Value"]
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W<SYST_CVR_SPEC> {
        CURRENT_W::new(self, 0)
    }
}
#[doc = "SysTick Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_cvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_cvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYST_CVR_SPEC;
impl crate::RegisterSpec for SYST_CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_cvr::R`](R) reader structure"]
impl crate::Readable for SYST_CVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syst_cvr::W`](W) writer structure"]
impl crate::Writable for SYST_CVR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_CVR to value 0"]
impl crate::Resettable for SYST_CVR_SPEC {
    const RESET_VALUE: u32 = 0;
}
