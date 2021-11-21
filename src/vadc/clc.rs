#[doc = "Register `CLC` reader"]
pub struct R(crate::R<CLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLC` writer"]
pub struct W(crate::W<CLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLC_SPEC>;
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
impl From<crate::W<CLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Module Disable Request Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISR_A {
    #[doc = "0: On request: enable the module clock"]
    VALUE1 = 0,
    #[doc = "1: Off request: stop the module clock"]
    VALUE2 = 1,
}
impl From<DISR_A> for bool {
    #[inline(always)]
    fn from(variant: DISR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISR` reader - Module Disable Request Bit"]
pub struct DISR_R(crate::FieldReader<bool, DISR_A>);
impl DISR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISR_A {
        match self.bits {
            false => DISR_A::VALUE1,
            true => DISR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DISR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DISR_A::VALUE2
    }
}
impl core::ops::Deref for DISR_R {
    type Target = crate::FieldReader<bool, DISR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISR` writer - Module Disable Request Bit"]
pub struct DISR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On request: enable the module clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISR_A::VALUE1)
    }
    #[doc = "Off request: stop the module clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Module Disable Status Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISS_A {
    #[doc = "0: Module clock is enabled"]
    VALUE1 = 0,
    #[doc = "1: Off: module is not clocked"]
    VALUE2 = 1,
}
impl From<DISS_A> for bool {
    #[inline(always)]
    fn from(variant: DISS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISS` reader - Module Disable Status Bit"]
pub struct DISS_R(crate::FieldReader<bool, DISS_A>);
impl DISS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISS_A {
        match self.bits {
            false => DISS_A::VALUE1,
            true => DISS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DISS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DISS_A::VALUE2
    }
}
impl core::ops::Deref for DISS_R {
    type Target = crate::FieldReader<bool, DISS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sleep Mode Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDIS_A {
    #[doc = "0: Sleep mode request is enabled and functional"]
    VALUE1 = 0,
    #[doc = "1: Module disregards the sleep mode control signal"]
    VALUE2 = 1,
}
impl From<EDIS_A> for bool {
    #[inline(always)]
    fn from(variant: EDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDIS` reader - Sleep Mode Enable Control"]
pub struct EDIS_R(crate::FieldReader<bool, EDIS_A>);
impl EDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDIS_A {
        match self.bits {
            false => EDIS_A::VALUE1,
            true => EDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EDIS_A::VALUE2
    }
}
impl core::ops::Deref for EDIS_R {
    type Target = crate::FieldReader<bool, EDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDIS` writer - Sleep Mode Enable Control"]
pub struct EDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sleep mode request is enabled and functional"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDIS_A::VALUE1)
    }
    #[doc = "Module disregards the sleep mode control signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDIS_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&self) -> EDIS_R {
        EDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&mut self) -> DISR_W {
        DISR_W { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&mut self) -> EDIS_W {
        EDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clc](index.html) module"]
pub struct CLC_SPEC;
impl crate::RegisterSpec for CLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clc::R](R) reader structure"]
impl crate::Readable for CLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clc::W](W) writer structure"]
impl crate::Writable for CLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLC to value 0x03"]
impl crate::Resettable for CLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
