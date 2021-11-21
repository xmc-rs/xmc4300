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
pub struct SSTAT_R(crate::FieldReader<u32, u32>);
impl SSTAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        SSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSTAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTAT` writer - Source Status"]
pub struct SSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&self) -> SSTAT_R {
        SSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status"]
    #[inline(always)]
    pub fn sstat(&mut self) -> SSTAT_W {
        SSTAT_W { w: self }
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
}
#[doc = "`reset()` method sets SSTAT to value 0"]
impl crate::Resettable for SSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
