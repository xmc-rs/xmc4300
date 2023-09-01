#[doc = "Register `SGR` reader"]
pub type R = crate::R<SGR_SPEC>;
#[doc = "Register `SGR` writer"]
pub type W = crate::W<SGR_SPEC>;
#[doc = "Field `SGI` reader - Source gather interval"]
pub type SGI_R = crate::FieldReader<u32>;
#[doc = "Field `SGI` writer - Source gather interval"]
pub type SGI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `SGC` reader - Source gather count"]
pub type SGC_R = crate::FieldReader<u16>;
#[doc = "Field `SGC` writer - Source gather count"]
pub type SGC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
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
    #[must_use]
    pub fn sgi(&mut self) -> SGI_W<SGR_SPEC, 0> {
        SGI_W::new(self)
    }
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    #[must_use]
    pub fn sgc(&mut self) -> SGC_W<SGR_SPEC, 20> {
        SGC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source Gather Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SGR_SPEC;
impl crate::RegisterSpec for SGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr::R`](R) reader structure"]
impl crate::Readable for SGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sgr::W`](W) writer structure"]
impl crate::Writable for SGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SGR to value 0"]
impl crate::Resettable for SGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
