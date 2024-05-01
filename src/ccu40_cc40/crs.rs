#[doc = "Register `CRS` reader"]
pub type R = crate::R<CRS_SPEC>;
#[doc = "Register `CRS` writer"]
pub type W = crate::W<CRS_SPEC>;
#[doc = "Field `CRS` reader - Compare Register"]
pub type CRS_R = crate::FieldReader<u16>;
#[doc = "Field `CRS` writer - Compare Register"]
pub type CRS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CRS_W<CRS_SPEC> {
        CRS_W::new(self, 0)
    }
}
#[doc = "Timer Shadow Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRS_SPEC;
impl crate::RegisterSpec for CRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crs::R`](R) reader structure"]
impl crate::Readable for CRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crs::W`](W) writer structure"]
impl crate::Writable for CRS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRS to value 0"]
impl crate::Resettable for CRS_SPEC {
    const RESET_VALUE: u32 = 0;
}
