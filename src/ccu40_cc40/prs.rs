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
pub type PRS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRS` writer - Period Register"]
pub type PRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRS_SPEC, u16, u16, 16, O>;
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
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<0> {
        PRS_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRS to value 0"]
impl crate::Resettable for PRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
