#[doc = "Register `PRS` reader"]
pub struct R(crate::R<PRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRS` writer"]
pub struct W(crate::W<PRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_SPEC>;
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
impl From<crate::W<PRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRS` reader - Period Register"]
pub struct PRS_R(crate::FieldReader<u16, u16>);
impl PRS_R {
    pub(crate) fn new(bits: u16) -> Self {
        PRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRS` writer - Period Register"]
pub struct PRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Shadow Period Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs](index.html) module"]
pub struct PRS_SPEC;
impl crate::RegisterSpec for PRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs::R](R) reader structure"]
impl crate::Readable for PRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs::W](W) writer structure"]
impl crate::Writable for PRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRS to value 0"]
impl crate::Resettable for PRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
