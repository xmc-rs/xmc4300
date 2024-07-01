#[doc = "Register `CMTR` reader"]
pub type R = crate::R<CMTR_SPEC>;
#[doc = "Register `CMTR` writer"]
pub type W = crate::W<CMTR_SPEC>;
#[doc = "Field `CTV` reader - Captured Timer Value"]
pub type CTV_R = crate::FieldReader<u16>;
#[doc = "Field `CTV` writer - Captured Timer Value"]
pub type CTV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Captured Timer Value"]
    #[inline(always)]
    pub fn ctv(&self) -> CTV_R {
        CTV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Captured Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ctv(&mut self) -> CTV_W<CMTR_SPEC> {
        CTV_W::new(self, 0)
    }
}
#[doc = "Capture Mode Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMTR_SPEC;
impl crate::RegisterSpec for CMTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmtr::R`](R) reader structure"]
impl crate::Readable for CMTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmtr::W`](W) writer structure"]
impl crate::Writable for CMTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMTR to value 0"]
impl crate::Resettable for CMTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
