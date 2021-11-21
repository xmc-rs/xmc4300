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
pub struct TIME_SYNC1_SYNC0_R(crate::FieldReader<u32, u32>);
impl TIME_SYNC1_SYNC0_R {
    pub(crate) fn new(bits: u32) -> Self {
        TIME_SYNC1_SYNC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_SYNC1_SYNC0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_SYNC1_SYNC0` writer - Time between SYNC1 pulses and SYNC0 pulse"]
pub struct TIME_SYNC1_SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_SYNC1_SYNC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    pub fn time_sync1_sync0(&self) -> TIME_SYNC1_SYNC0_R {
        TIME_SYNC1_SYNC0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between SYNC1 pulses and SYNC0 pulse"]
    #[inline(always)]
    pub fn time_sync1_sync0(&mut self) -> TIME_SYNC1_SYNC0_W {
        TIME_SYNC1_SYNC0_W { w: self }
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
}
#[doc = "`reset()` method sets DC_SYNC1_CYC_TIME to value 0"]
impl crate::Resettable for DC_SYNC1_CYC_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
