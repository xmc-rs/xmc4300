#[doc = "Register `SSTAT` reader"]
pub type R = crate::R<SstatSpec>;
#[doc = "Register `SSTAT` writer"]
pub type W = crate::W<SstatSpec>;
#[doc = "Field `SSTAT` reader - Source Status"]
pub type SstatR = crate::FieldReader<u32>;
#[doc = "Field `SSTAT` writer - Source Status"]
pub type SstatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&self) -> SstatR {
        SstatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    #[must_use]
    pub fn sstat(&mut self) -> SstatW<SstatSpec> {
        SstatW::new(self, 0)
    }
}
#[doc = "Source Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatSpec;
impl crate::RegisterSpec for SstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat::R`](R) reader structure"]
impl crate::Readable for SstatSpec {}
#[doc = "`write(|w| ..)` method takes [`sstat::W`](W) writer structure"]
impl crate::Writable for SstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTAT to value 0"]
impl crate::Resettable for SstatSpec {
    const RESET_VALUE: u32 = 0;
}
