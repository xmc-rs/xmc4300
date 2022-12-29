#[doc = "Register `SYSTEM_TIME_SECONDS_UPDATE` reader"]
pub struct R(crate::R<SYSTEM_TIME_SECONDS_UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTEM_TIME_SECONDS_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTEM_TIME_SECONDS_UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTEM_TIME_SECONDS_UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTEM_TIME_SECONDS_UPDATE` writer"]
pub struct W(crate::W<SYSTEM_TIME_SECONDS_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTEM_TIME_SECONDS_UPDATE_SPEC>;
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
impl From<crate::W<SYSTEM_TIME_SECONDS_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTEM_TIME_SECONDS_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSS` writer - Timestamp Second"]
pub type TSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTEM_TIME_SECONDS_UPDATE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<0> {
        TSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Time - Seconds Update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_time_seconds_update](index.html) module"]
pub struct SYSTEM_TIME_SECONDS_UPDATE_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_SECONDS_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [system_time_seconds_update::R](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_SECONDS_UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [system_time_seconds_update::W](W) writer structure"]
impl crate::Writable for SYSTEM_TIME_SECONDS_UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTEM_TIME_SECONDS_UPDATE to value 0"]
impl crate::Resettable for SYSTEM_TIME_SECONDS_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
