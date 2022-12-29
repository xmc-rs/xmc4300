#[doc = "Register `CMTR` reader"]
pub struct R(crate::R<CMTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMTR` writer"]
pub struct W(crate::W<CMTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMTR_SPEC>;
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
impl From<crate::W<CMTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTV` reader - Captured Timer Value"]
pub type CTV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTV` writer - Captured Timer Value"]
pub type CTV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMTR_SPEC, u16, u16, 10, O>;
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
    pub fn ctv(&mut self) -> CTV_W<0> {
        CTV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Mode Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmtr](index.html) module"]
pub struct CMTR_SPEC;
impl crate::RegisterSpec for CMTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmtr::R](R) reader structure"]
impl crate::Readable for CMTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmtr::W](W) writer structure"]
impl crate::Writable for CMTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMTR to value 0"]
impl crate::Resettable for CMTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
