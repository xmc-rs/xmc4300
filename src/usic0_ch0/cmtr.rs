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
pub struct CTV_R(crate::FieldReader<u16, u16>);
impl CTV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTV` writer - Captured Timer Value"]
pub struct CTV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
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
    pub fn ctv(&mut self) -> CTV_W {
        CTV_W { w: self }
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
}
#[doc = "`reset()` method sets CMTR to value 0"]
impl crate::Resettable for CMTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
