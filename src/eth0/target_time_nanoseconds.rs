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
pub type TTSLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register"]
pub type TTSLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TARGET_TIME_NANOSECONDS_SPEC, u32, u32, 31, O>;
#[doc = "Field `TRGTBUSY` reader - Target Time Register Busy"]
pub type TRGTBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    #[must_use]
    pub fn ttslo(&mut self) -> TTSLO_W<0> {
        TTSLO_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET_TIME_NANOSECONDS to value 0"]
impl crate::Resettable for TARGET_TIME_NANOSECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
