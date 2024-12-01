#[doc = "Register `SSTAT` reader"]
pub type R = crate::R<SSTAT_SPEC>;
#[doc = "Register `SSTAT` writer"]
pub type W = crate::W<SSTAT_SPEC>;
#[doc = "Field `SSTAT` reader - Source Status"]
pub type SSTAT_R = crate::FieldReader<u32>;
#[doc = "Field `SSTAT` writer - Source Status"]
pub type SSTAT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&self) -> SSTAT_R {
        SSTAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&mut self) -> SSTAT_W<SSTAT_SPEC> {
        SSTAT_W::new(self, 0)
    }
}
#[doc = "Source Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTAT_SPEC;
impl crate::RegisterSpec for SSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat::R`](R) reader structure"]
impl crate::Readable for SSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sstat::W`](W) writer structure"]
impl crate::Writable for SSTAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTAT to value 0"]
impl crate::Resettable for SSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
