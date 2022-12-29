#[doc = "Register `SYSTEM_TIME_HIGHER_WORD_SECONDS` reader"]
pub struct R(crate::R<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTEM_TIME_HIGHER_WORD_SECONDS` writer"]
pub struct W(crate::W<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>;
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
impl From<crate::W<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSHWR` reader - Timestamp Higher Word Register"]
pub type TSHWR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSHWR` writer - Timestamp Higher Word Register"]
pub type TSHWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    pub fn tshwr(&self) -> TSHWR_R {
        TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    #[must_use]
    pub fn tshwr(&mut self) -> TSHWR_W<0> {
        TSHWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Time - Higher Word Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_time_higher_word_seconds](index.html) module"]
pub struct SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [system_time_higher_word_seconds::R](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [system_time_higher_word_seconds::W](W) writer structure"]
impl crate::Writable for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTEM_TIME_HIGHER_WORD_SECONDS to value 0"]
impl crate::Resettable for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
