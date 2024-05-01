#[doc = "Register `SGR` reader"]
pub type R = crate::R<SgrSpec>;
#[doc = "Register `SGR` writer"]
pub type W = crate::W<SgrSpec>;
#[doc = "Field `SGI` reader - Source gather interval"]
pub type SgiR = crate::FieldReader<u32>;
#[doc = "Field `SGI` writer - Source gather interval"]
pub type SgiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SGC` reader - Source gather count"]
pub type SgcR = crate::FieldReader<u16>;
#[doc = "Field `SGC` writer - Source gather count"]
pub type SgcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    pub fn sgi(&self) -> SgiR {
        SgiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    pub fn sgc(&self) -> SgcR {
        SgcR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    #[must_use]
    pub fn sgi(&mut self) -> SgiW<SgrSpec> {
        SgiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    #[must_use]
    pub fn sgc(&mut self) -> SgcW<SgrSpec> {
        SgcW::new(self, 20)
    }
}
#[doc = "Source Gather Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgrSpec;
impl crate::RegisterSpec for SgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr::R`](R) reader structure"]
impl crate::Readable for SgrSpec {}
#[doc = "`write(|w| ..)` method takes [`sgr::W`](W) writer structure"]
impl crate::Writable for SgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SGR to value 0"]
impl crate::Resettable for SgrSpec {
    const RESET_VALUE: u32 = 0;
}
