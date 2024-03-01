#[doc = "Register `CRS` reader"]
pub type R = crate::R<CrsSpec>;
#[doc = "Register `CRS` writer"]
pub type W = crate::W<CrsSpec>;
#[doc = "Field `CRS` reader - Compare Register"]
pub type CrsR = crate::FieldReader<u16>;
#[doc = "Field `CRS` writer - Compare Register"]
pub type CrsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CrsW<CrsSpec> {
        CrsW::new(self, 0)
    }
}
#[doc = "Timer Shadow Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsSpec;
impl crate::RegisterSpec for CrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crs::R`](R) reader structure"]
impl crate::Readable for CrsSpec {}
#[doc = "`write(|w| ..)` method takes [`crs::W`](W) writer structure"]
impl crate::Writable for CrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRS to value 0"]
impl crate::Resettable for CrsSpec {
    const RESET_VALUE: u32 = 0;
}
