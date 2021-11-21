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
pub struct TSHWR_R(crate::FieldReader<u16, u16>);
impl TSHWR_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSHWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSHWR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSHWR` writer - Timestamp Higher Word Register"]
pub struct TSHWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSHWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn tshwr(&mut self) -> TSHWR_W {
        TSHWR_W { w: self }
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
}
#[doc = "`reset()` method sets SYSTEM_TIME_HIGHER_WORD_SECONDS to value 0"]
impl crate::Resettable for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
