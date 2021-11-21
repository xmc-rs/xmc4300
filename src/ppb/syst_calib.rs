#[doc = "Register `SYST_CALIB` reader"]
pub struct R(crate::R<SYST_CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CALIB` writer"]
pub struct W(crate::W<SYST_CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CALIB_SPEC>;
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
impl From<crate::W<SYST_CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - Ten Milliseconds Reload Value"]
pub struct TENMS_R(crate::FieldReader<u32, u32>);
impl TENMS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TENMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENMS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENMS` writer - Ten Milliseconds Reload Value"]
pub struct TENMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TENMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Ten Milliseconds Skewed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEW_A {
    #[doc = "0: TENMS value is exact"]
    VALUE1 = 0,
    #[doc = "1: TENMS value is inexact, or not given."]
    VALUE2 = 1,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEW` reader - Ten Milliseconds Skewed"]
pub struct SKEW_R(crate::FieldReader<bool, SKEW_A>);
impl SKEW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SKEW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::VALUE1,
            true => SKEW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SKEW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SKEW_A::VALUE2
    }
}
impl core::ops::Deref for SKEW_R {
    type Target = crate::FieldReader<bool, SKEW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKEW` writer - Ten Milliseconds Skewed"]
pub struct SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SKEW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TENMS value is exact"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SKEW_A::VALUE1)
    }
    #[doc = "TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SKEW_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "No Reference Clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "0: reference clock provided"]
    VALUE1 = 0,
    #[doc = "1: no reference clock provided."]
    VALUE2 = 1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - No Reference Clock"]
pub struct NOREF_R(crate::FieldReader<bool, NOREF_A>);
impl NOREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::VALUE1,
            true => NOREF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NOREF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NOREF_A::VALUE2
    }
}
impl core::ops::Deref for NOREF_R {
    type Target = crate::FieldReader<bool, NOREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOREF` writer - No Reference Clock"]
pub struct NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOREF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "reference clock provided"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOREF_A::VALUE1)
    }
    #[doc = "no reference clock provided."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOREF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W {
        TENMS_W { w: self }
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Calibration Value Register r\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_calib](index.html) module"]
pub struct SYST_CALIB_SPEC;
impl crate::RegisterSpec for SYST_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_calib::R](R) reader structure"]
impl crate::Readable for SYST_CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_calib::W](W) writer structure"]
impl crate::Writable for SYST_CALIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYST_CALIB to value 0xc000_0000"]
impl crate::Resettable for SYST_CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
