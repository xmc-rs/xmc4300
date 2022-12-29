#[doc = "Register `CLC` reader"]
pub struct R(crate::R<CLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLC` writer"]
pub struct W(crate::W<CLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLC_SPEC>;
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
impl From<crate::W<CLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISR` reader - Module Disable Request Bit"]
pub type DISR_R = crate::BitReader<bool>;
#[doc = "Field `DISR` writer - Module Disable Request Bit"]
pub type DISR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLC_SPEC, bool, O>;
#[doc = "Field `DISS` reader - Module Disable Status Bit"]
pub type DISS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn disr(&mut self) -> DISR_W<0> {
        DISR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clc](index.html) module"]
pub struct CLC_SPEC;
impl crate::RegisterSpec for CLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clc::R](R) reader structure"]
impl crate::Readable for CLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clc::W](W) writer structure"]
impl crate::Writable for CLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLC to value 0x03"]
impl crate::Resettable for CLC_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
