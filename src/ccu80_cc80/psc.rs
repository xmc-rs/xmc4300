#[doc = "Register `PSC` reader"]
pub type R = crate::R<PscSpec>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PscSpec>;
#[doc = "Field `PSIV` reader - Prescaler Initial Value"]
pub type PsivR = crate::FieldReader;
#[doc = "Field `PSIV` writer - Prescaler Initial Value"]
pub type PsivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    pub fn psiv(&self) -> PsivR {
        PsivR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn psiv(&mut self) -> PsivW<PscSpec> {
        PsivW::new(self, 0)
    }
}
#[doc = "Prescaler Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscSpec;
impl crate::RegisterSpec for PscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PscSpec {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PscSpec {
    const RESET_VALUE: u32 = 0;
}
