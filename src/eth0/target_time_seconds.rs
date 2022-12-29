#[doc = "Register `TARGET_TIME_SECONDS` reader"]
pub struct R(crate::R<TARGET_TIME_SECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET_TIME_SECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET_TIME_SECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET_TIME_SECONDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET_TIME_SECONDS` writer"]
pub struct W(crate::W<TARGET_TIME_SECONDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET_TIME_SECONDS_SPEC>;
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
impl From<crate::W<TARGET_TIME_SECONDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET_TIME_SECONDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTR` reader - Target Time Seconds Register"]
pub type TSTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSTR` writer - Target Time Seconds Register"]
pub type TSTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TARGET_TIME_SECONDS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstr(&self) -> TSTR_R {
        TSTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    #[must_use]
    pub fn tstr(&mut self) -> TSTR_W<0> {
        TSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target Time Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target_time_seconds](index.html) module"]
pub struct TARGET_TIME_SECONDS_SPEC;
impl crate::RegisterSpec for TARGET_TIME_SECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target_time_seconds::R](R) reader structure"]
impl crate::Readable for TARGET_TIME_SECONDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target_time_seconds::W](W) writer structure"]
impl crate::Writable for TARGET_TIME_SECONDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET_TIME_SECONDS to value 0"]
impl crate::Resettable for TARGET_TIME_SECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
