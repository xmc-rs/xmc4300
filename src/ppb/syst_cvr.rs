#[doc = "Register `SYST_CVR` reader"]
pub type R = crate::R<SystCvrSpec>;
#[doc = "Register `SYST_CVR` writer"]
pub type W = crate::W<SystCvrSpec>;
#[doc = "Field `CURRENT` reader - Current Value"]
pub type CurrentR = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - Current Value"]
pub type CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Current Value"]
    #[inline(always)]
    pub fn current(&self) -> CurrentR {
        CurrentR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Current Value"]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CurrentW<SystCvrSpec> {
        CurrentW::new(self, 0)
    }
}
#[doc = "SysTick Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_cvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_cvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystCvrSpec;
impl crate::RegisterSpec for SystCvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_cvr::R`](R) reader structure"]
impl crate::Readable for SystCvrSpec {}
#[doc = "`write(|w| ..)` method takes [`syst_cvr::W`](W) writer structure"]
impl crate::Writable for SystCvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_CVR to value 0"]
impl crate::Resettable for SystCvrSpec {
    const RESET_VALUE: u32 = 0;
}
