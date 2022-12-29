#[doc = "Register `SSTAT` reader"]
pub struct R(crate::R<SSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTAT` writer"]
pub struct W(crate::W<SSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTAT_SPEC>;
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
impl From<crate::W<SSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTAT` reader - Source Status"]
pub type SSTAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SSTAT` writer - Source Status"]
pub type SSTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSTAT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&self) -> SSTAT_R {
        SSTAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    #[must_use]
    pub fn sstat(&mut self) -> SSTAT_W<0> {
        SSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstat](index.html) module"]
pub struct SSTAT_SPEC;
impl crate::RegisterSpec for SSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstat::R](R) reader structure"]
impl crate::Readable for SSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstat::W](W) writer structure"]
impl crate::Writable for SSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSTAT to value 0"]
impl crate::Resettable for SSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
