#[doc = "Register `SSTATAR` reader"]
pub type R = crate::R<SSTATAR_SPEC>;
#[doc = "Register `SSTATAR` writer"]
pub type W = crate::W<SSTATAR_SPEC>;
#[doc = "Field `SSTATAR` reader - Source Status Address"]
pub type SSTATAR_R = crate::FieldReader<u32>;
#[doc = "Field `SSTATAR` writer - Source Status Address"]
pub type SSTATAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    pub fn sstatar(&self) -> SSTATAR_R {
        SSTATAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    #[must_use]
    pub fn sstatar(&mut self) -> SSTATAR_W<SSTATAR_SPEC> {
        SSTATAR_W::new(self, 0)
    }
}
#[doc = "Source Status Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTATAR_SPEC;
impl crate::RegisterSpec for SSTATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar::R`](R) reader structure"]
impl crate::Readable for SSTATAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sstatar::W`](W) writer structure"]
impl crate::Writable for SSTATAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTATAR to value 0"]
impl crate::Resettable for SSTATAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
