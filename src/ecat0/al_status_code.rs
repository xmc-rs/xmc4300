#[doc = "Register `AL_STATUS_CODE` reader"]
pub struct R(crate::R<AL_STATUS_CODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AL_STATUS_CODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AL_STATUS_CODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AL_STATUS_CODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AL_STATUS_CODE` writer"]
pub struct W(crate::W<AL_STATUS_CODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AL_STATUS_CODE_SPEC>;
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
impl From<crate::W<AL_STATUS_CODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AL_STATUS_CODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AL_S_CODE` reader - AL Status Code"]
pub type AL_S_CODE_R = crate::FieldReader<u16>;
#[doc = "Field `AL_S_CODE` writer - AL Status Code"]
pub type AL_S_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, AL_STATUS_CODE_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    pub fn al_s_code(&self) -> AL_S_CODE_R {
        AL_S_CODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    #[must_use]
    pub fn al_s_code(&mut self) -> AL_S_CODE_W<0> {
        AL_S_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AL Status Code\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_status_code](index.html) module"]
pub struct AL_STATUS_CODE_SPEC;
impl crate::RegisterSpec for AL_STATUS_CODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [al_status_code::R](R) reader structure"]
impl crate::Readable for AL_STATUS_CODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [al_status_code::W](W) writer structure"]
impl crate::Writable for AL_STATUS_CODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AL_STATUS_CODE to value 0"]
impl crate::Resettable for AL_STATUS_CODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
