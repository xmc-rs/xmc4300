#[doc = "Register `DC_SYNC0_CYC_TIME` reader"]
pub struct R(crate::R<DC_SYNC0_CYC_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SYNC0_CYC_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SYNC0_CYC_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SYNC0_CYC_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_SYNC0_CYC_TIME` writer"]
pub struct W(crate::W<DC_SYNC0_CYC_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SYNC0_CYC_TIME_SPEC>;
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
impl From<crate::W<DC_SYNC0_CYC_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SYNC0_CYC_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_BETWEEN_SYNC0` reader - Time between two consecutive SYNC0 pulses"]
pub type TIME_BETWEEN_SYNC0_R = crate::FieldReader<TIME_BETWEEN_SYNC0_A>;
#[doc = "Time between two consecutive SYNC0 pulses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TIME_BETWEEN_SYNC0_A {
    #[doc = "0: Single shot mode, generate only one SYNC0 pulse"]
    VALUE1 = 0,
}
impl From<TIME_BETWEEN_SYNC0_A> for u32 {
    #[inline(always)]
    fn from(variant: TIME_BETWEEN_SYNC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIME_BETWEEN_SYNC0_A {
    type Ux = u32;
}
impl TIME_BETWEEN_SYNC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIME_BETWEEN_SYNC0_A> {
        match self.bits {
            0 => Some(TIME_BETWEEN_SYNC0_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIME_BETWEEN_SYNC0_A::VALUE1
    }
}
#[doc = "Field `TIME_BETWEEN_SYNC0` writer - Time between two consecutive SYNC0 pulses"]
pub type TIME_BETWEEN_SYNC0_W<'a, const O: u8> = crate::FieldWriter<'a, DC_SYNC0_CYC_TIME_SPEC, 32, O, TIME_BETWEEN_SYNC0_A>;
impl<'a, const O: u8> TIME_BETWEEN_SYNC0_W<'a, O> {
    #[doc = "Single shot mode, generate only one SYNC0 pulse"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TIME_BETWEEN_SYNC0_A::VALUE1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    pub fn time_between_sync0(&self) -> TIME_BETWEEN_SYNC0_R {
        TIME_BETWEEN_SYNC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    #[must_use]
    pub fn time_between_sync0(&mut self) -> TIME_BETWEEN_SYNC0_W<0> {
        TIME_BETWEEN_SYNC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNC0 Cycle Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sync0_cyc_time](index.html) module"]
pub struct DC_SYNC0_CYC_TIME_SPEC;
impl crate::RegisterSpec for DC_SYNC0_CYC_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_sync0_cyc_time::R](R) reader structure"]
impl crate::Readable for DC_SYNC0_CYC_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_sync0_cyc_time::W](W) writer structure"]
impl crate::Writable for DC_SYNC0_CYC_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_SYNC0_CYC_TIME to value 0"]
impl crate::Resettable for DC_SYNC0_CYC_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
