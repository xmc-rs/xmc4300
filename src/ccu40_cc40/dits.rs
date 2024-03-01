#[doc = "Register `DITS` reader"]
pub type R = crate::R<DitsSpec>;
#[doc = "Register `DITS` writer"]
pub type W = crate::W<DitsSpec>;
#[doc = "Field `DCVS` reader - Dither Shadow Compare Value"]
pub type DcvsR = crate::FieldReader;
#[doc = "Field `DCVS` writer - Dither Shadow Compare Value"]
pub type DcvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    pub fn dcvs(&self) -> DcvsR {
        DcvsR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither Shadow Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn dcvs(&mut self) -> DcvsW<DitsSpec> {
        DcvsW::new(self, 0)
    }
}
#[doc = "Dither Shadow Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DitsSpec;
impl crate::RegisterSpec for DitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dits::R`](R) reader structure"]
impl crate::Readable for DitsSpec {}
#[doc = "`write(|w| ..)` method takes [`dits::W`](W) writer structure"]
impl crate::Writable for DitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DITS to value 0"]
impl crate::Resettable for DitsSpec {
    const RESET_VALUE: u32 = 0;
}
