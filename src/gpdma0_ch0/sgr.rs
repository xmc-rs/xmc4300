#[doc = "Register `SGR` reader"]
pub struct R(crate::R<SGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SGR` writer"]
pub struct W(crate::W<SGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGI` reader - Source gather interval"]
pub type SGI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SGI` writer - Source gather interval"]
pub type SGI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SGR_SPEC, u32, u32, 20, O>;
#[doc = "Field `SGC` reader - Source gather count"]
pub type SGC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SGC` writer - Source gather count"]
pub type SGC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SGR_SPEC, u16, u16, 12, O>;
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
    pub fn sgi(&mut self) -> SGI_W<0> {
        SGI_W::new(self)
    }
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    #[must_use]
    pub fn sgc(&mut self) -> SGC_W<20> {
        SGC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Gather Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sgr](index.html) module"]
pub struct SGR_SPEC;
impl crate::RegisterSpec for SGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sgr::R](R) reader structure"]
impl crate::Readable for SGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sgr::W](W) writer structure"]
impl crate::Writable for SGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SGR to value 0"]
impl crate::Resettable for SGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
