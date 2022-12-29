#[doc = "Register `HFIR` reader"]
pub struct R(crate::R<HFIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFIR` writer"]
pub struct W(crate::W<HFIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFIR_SPEC>;
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
impl From<crate::W<HFIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FrInt` reader - Frame Interval"]
pub type FR_INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FrInt` writer - Frame Interval"]
pub type FR_INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFIR_SPEC, u16, u16, 16, O>;
#[doc = "Field `HFIRRldCtrl` reader - Reload Control"]
pub type HFIRRLD_CTRL_R = crate::BitReader<HFIRRLD_CTRL_A>;
#[doc = "Reload Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFIRRLD_CTRL_A {
    #[doc = "0: HFIR cannot be reloaded dynamically"]
    VALUE1 = 0,
    #[doc = "1: HFIR can be dynamically reloaded during runtime"]
    VALUE2 = 1,
}
impl From<HFIRRLD_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: HFIRRLD_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl HFIRRLD_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFIRRLD_CTRL_A {
        match self.bits {
            false => HFIRRLD_CTRL_A::VALUE1,
            true => HFIRRLD_CTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HFIRRLD_CTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HFIRRLD_CTRL_A::VALUE2
    }
}
#[doc = "Field `HFIRRldCtrl` writer - Reload Control"]
pub type HFIRRLD_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFIR_SPEC, HFIRRLD_CTRL_A, O>;
impl<'a, const O: u8> HFIRRLD_CTRL_W<'a, O> {
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFIRRLD_CTRL_A::VALUE1)
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFIRRLD_CTRL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn fr_int(&self) -> FR_INT_R {
        FR_INT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrld_ctrl(&self) -> HFIRRLD_CTRL_R {
        HFIRRLD_CTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    #[must_use]
    pub fn fr_int(&mut self) -> FR_INT_W<0> {
        FR_INT_W::new(self)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    #[must_use]
    pub fn hfirrld_ctrl(&mut self) -> HFIRRLD_CTRL_W<16> {
        HFIRRLD_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Frame Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfir](index.html) module"]
pub struct HFIR_SPEC;
impl crate::RegisterSpec for HFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfir::R](R) reader structure"]
impl crate::Readable for HFIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfir::W](W) writer structure"]
impl crate::Writable for HFIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFIR to value 0xea60"]
impl crate::Resettable for HFIR_SPEC {
    const RESET_VALUE: Self::Ux = 0xea60;
}
