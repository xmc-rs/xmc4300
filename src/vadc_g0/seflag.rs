#[doc = "Register `SEFLAG` reader"]
pub struct R(crate::R<SEFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEFLAG` writer"]
pub struct W(crate::W<SEFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEFLAG_SPEC>;
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
impl From<crate::W<SEFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV0_A {
    #[doc = "0: No source event"]
    VALUE1 = 0,
    #[doc = "1: A source event has occurred"]
    VALUE2 = 1,
}
impl From<SEV0_A> for bool {
    #[inline(always)]
    fn from(variant: SEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV0` reader - Source Event 0/1"]
pub struct SEV0_R(crate::FieldReader<bool, SEV0_A>);
impl SEV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEV0_A {
        match self.bits {
            false => SEV0_A::VALUE1,
            true => SEV0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SEV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SEV0_A::VALUE2
    }
}
impl core::ops::Deref for SEV0_R {
    type Target = crate::FieldReader<bool, SEV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEV0` writer - Source Event 0/1"]
pub struct SEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0_A::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0_A::VALUE2)
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
#[doc = "Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV1_A {
    #[doc = "0: No source event"]
    VALUE1 = 0,
    #[doc = "1: A source event has occurred"]
    VALUE2 = 1,
}
impl From<SEV1_A> for bool {
    #[inline(always)]
    fn from(variant: SEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV1` reader - Source Event 0/1"]
pub struct SEV1_R(crate::FieldReader<bool, SEV1_A>);
impl SEV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEV1_A {
        match self.bits {
            false => SEV1_A::VALUE1,
            true => SEV1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SEV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SEV1_A::VALUE2
    }
}
impl core::ops::Deref for SEV1_R {
    type Target = crate::FieldReader<bool, SEV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEV1` writer - Source Event 0/1"]
pub struct SEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV1_A::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev0(&self) -> SEV0_R {
        SEV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev1(&self) -> SEV1_R {
        SEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev0(&mut self) -> SEV0_W {
        SEV0_W { w: self }
    }
    #[doc = "Bit 1 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev1(&mut self) -> SEV1_W {
        SEV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seflag](index.html) module"]
pub struct SEFLAG_SPEC;
impl crate::RegisterSpec for SEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seflag::R](R) reader structure"]
impl crate::Readable for SEFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seflag::W](W) writer structure"]
impl crate::Writable for SEFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEFLAG to value 0"]
impl crate::Resettable for SEFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
