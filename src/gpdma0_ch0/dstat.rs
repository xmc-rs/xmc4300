#[doc = "Register `DSTAT` reader"]
pub type R = crate::R<DstatSpec>;
#[doc = "Register `DSTAT` writer"]
pub type W = crate::W<DstatSpec>;
#[doc = "Field `DSTAT` reader - Destination Status"]
pub type DstatR = crate::FieldReader<u32>;
#[doc = "Field `DSTAT` writer - Destination Status"]
pub type DstatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    pub fn dstat(&self) -> DstatR {
        DstatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    #[must_use]
    pub fn dstat(&mut self) -> DstatW<DstatSpec> {
        DstatW::new(self, 0)
    }
}
#[doc = "Destination Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstatSpec;
impl crate::RegisterSpec for DstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstat::R`](R) reader structure"]
impl crate::Readable for DstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dstat::W`](W) writer structure"]
impl crate::Writable for DstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSTAT to value 0"]
impl crate::Resettable for DstatSpec {
    const RESET_VALUE: u32 = 0;
}
