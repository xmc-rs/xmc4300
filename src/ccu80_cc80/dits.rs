#[doc = "Register `DITS` reader"]
pub type R = crate::R<DITS_SPEC>;
#[doc = "Register `DITS` writer"]
pub type W = crate::W<DITS_SPEC>;
#[doc = "Field `DCVS` reader - Dither Shadow Compare Value"]
pub type DCVS_R = crate::FieldReader;
#[doc = "Field `DCVS` writer - Dither Shadow Compare Value"]
pub type DCVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    pub fn dcvs(&self) -> DCVS_R {
        DCVS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    pub fn dcvs(&mut self) -> DCVS_W<DITS_SPEC> {
        DCVS_W::new(self, 0)
    }
}
#[doc = "Dither Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DITS_SPEC;
impl crate::RegisterSpec for DITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dits::R`](R) reader structure"]
impl crate::Readable for DITS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dits::W`](W) writer structure"]
impl crate::Writable for DITS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DITS to value 0"]
impl crate::Resettable for DITS_SPEC {
    const RESET_VALUE: u32 = 0;
}
