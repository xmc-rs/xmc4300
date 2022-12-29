#[doc = "Register `DC_SYNC1_CYC_TIME` reader"]
pub struct R(crate::R<DC_SYNC1_CYC_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SYNC1_CYC_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SYNC1_CYC_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SYNC1_CYC_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_SYNC1_CYC_TIME` writer"]
pub struct W(crate::W<DC_SYNC1_CYC_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SYNC1_CYC_TIME_SPEC>;
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
impl From<crate::W<DC_SYNC1_CYC_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SYNC1_CYC_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_SYNC1_SYNC0` reader - Time between SYNC1 pulses and SYNC0 pulse"]
pub type TIME_SYNC1_SYNC0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIME_SYNC1_SYNC0` writer - Time between SYNC1 pulses and SYNC0 pulse"]
pub type TIME_SYNC1_SYNC0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC_SYNC1_CYC_TIME_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    pub fn time_sync1_sync0(&self) -> TIME_SYNC1_SYNC0_R {
        TIME_SYNC1_SYNC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync1_sync0(&mut self) -> TIME_SYNC1_SYNC0_W<0> {
        TIME_SYNC1_SYNC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNC1 Cycle Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sync1_cyc_time](index.html) module"]
pub struct DC_SYNC1_CYC_TIME_SPEC;
impl crate::RegisterSpec for DC_SYNC1_CYC_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_sync1_cyc_time::R](R) reader structure"]
impl crate::Readable for DC_SYNC1_CYC_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_sync1_cyc_time::W](W) writer structure"]
impl crate::Writable for DC_SYNC1_CYC_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_SYNC1_CYC_TIME to value 0"]
impl crate::Resettable for DC_SYNC1_CYC_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
