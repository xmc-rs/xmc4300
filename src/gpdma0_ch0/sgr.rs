#[doc = "Register `SGR` reader"]
pub type R = crate::R<SGR_SPEC>;
#[doc = "Register `SGR` writer"]
pub type W = crate::W<SGR_SPEC>;
#[doc = "Field `SGI` reader - Source gather interval"]
pub type SGI_R = crate::FieldReader<u32>;
#[doc = "Field `SGI` writer - Source gather interval"]
pub type SGI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SGC` reader - Source gather count"]
pub type SGC_R = crate::FieldReader<u16>;
#[doc = "Field `SGC` writer - Source gather count"]
pub type SGC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    pub fn sgi(&self) -> SGI_R {
        SGI_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    pub fn sgc(&self) -> SGC_R {
        SGC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    pub fn sgi(&mut self) -> SGI_W<SGR_SPEC> {
        SGI_W::new(self, 0)
    }
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    pub fn sgc(&mut self) -> SGC_W<SGR_SPEC> {
        SGC_W::new(self, 20)
    }
}
#[doc = "Source Gather Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SGR_SPEC;
impl crate::RegisterSpec for SGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr::R`](R) reader structure"]
impl crate::Readable for SGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sgr::W`](W) writer structure"]
impl crate::Writable for SGR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SGR to value 0"]
impl crate::Resettable for SGR_SPEC {
    const RESET_VALUE: u32 = 0;
}
