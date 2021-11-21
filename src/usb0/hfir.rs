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
pub struct FRINT_R(crate::FieldReader<u16, u16>);
impl FRINT_R {
    pub(crate) fn new(bits: u16) -> Self {
        FRINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRINT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FrInt` writer - Frame Interval"]
pub struct FRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Reload Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFIRRLDCTRL_A {
    #[doc = "0: HFIR cannot be reloaded dynamically"]
    VALUE1 = 0,
    #[doc = "1: HFIR can be dynamically reloaded during runtime"]
    VALUE2 = 1,
}
impl From<HFIRRLDCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: HFIRRLDCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFIRRldCtrl` reader - Reload Control"]
pub struct HFIRRLDCTRL_R(crate::FieldReader<bool, HFIRRLDCTRL_A>);
impl HFIRRLDCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFIRRLDCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFIRRLDCTRL_A {
        match self.bits {
            false => HFIRRLDCTRL_A::VALUE1,
            true => HFIRRLDCTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HFIRRLDCTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HFIRRLDCTRL_A::VALUE2
    }
}
impl core::ops::Deref for HFIRRLDCTRL_R {
    type Target = crate::FieldReader<bool, HFIRRLDCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFIRRldCtrl` writer - Reload Control"]
pub struct HFIRRLDCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFIRRLDCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFIRRLDCTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFIRRLDCTRL_A::VALUE1)
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFIRRLDCTRL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn fr_int(&self) -> FRINT_R {
        FRINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrld_ctrl(&self) -> HFIRRLDCTRL_R {
        HFIRRLDCTRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn fr_int(&mut self) -> FRINT_W {
        FRINT_W { w: self }
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrld_ctrl(&mut self) -> HFIRRLDCTRL_W {
        HFIRRLDCTRL_W { w: self }
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
}
#[doc = "`reset()` method sets HFIR to value 0xea60"]
impl crate::Resettable for HFIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xea60
    }
}
