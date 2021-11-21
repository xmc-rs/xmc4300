#[doc = "Register `TARGET_TIME_NANOSECONDS` reader"]
pub struct R(crate::R<TARGET_TIME_NANOSECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET_TIME_NANOSECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET_TIME_NANOSECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET_TIME_NANOSECONDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET_TIME_NANOSECONDS` writer"]
pub struct W(crate::W<TARGET_TIME_NANOSECONDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET_TIME_NANOSECONDS_SPEC>;
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
impl From<crate::W<TARGET_TIME_NANOSECONDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET_TIME_NANOSECONDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSLO` reader - Target Timestamp Low Register"]
pub struct TTSLO_R(crate::FieldReader<u32, u32>);
impl TTSLO_R {
    pub(crate) fn new(bits: u32) -> Self {
        TTSLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTSLO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register"]
pub struct TTSLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Field `TRGTBUSY` reader - Target Time Register Busy"]
pub struct TRGTBUSY_R(crate::FieldReader<bool, bool>);
impl TRGTBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGTBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&mut self) -> TTSLO_W {
        TTSLO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target Time Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target_time_nanoseconds](index.html) module"]
pub struct TARGET_TIME_NANOSECONDS_SPEC;
impl crate::RegisterSpec for TARGET_TIME_NANOSECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target_time_nanoseconds::R](R) reader structure"]
impl crate::Readable for TARGET_TIME_NANOSECONDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target_time_nanoseconds::W](W) writer structure"]
impl crate::Writable for TARGET_TIME_NANOSECONDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARGET_TIME_NANOSECONDS to value 0"]
impl crate::Resettable for TARGET_TIME_NANOSECONDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
