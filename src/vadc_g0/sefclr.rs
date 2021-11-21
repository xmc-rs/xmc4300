#[doc = "Register `SEFCLR` writer"]
pub struct W(crate::W<SEFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEFCLR_SPEC>;
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
impl From<crate::W<SEFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    VALUE2 = 1,
}
impl From<SEV0_AW> for bool {
    #[inline(always)]
    fn from(variant: SEV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV0` writer - Clear Source Event 0/1"]
pub struct SEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0_AW::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0_AW::VALUE2)
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
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    VALUE2 = 1,
}
impl From<SEV1_AW> for bool {
    #[inline(always)]
    fn from(variant: SEV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV1` writer - Clear Source Event 0/1"]
pub struct SEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV1_AW::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV1_AW::VALUE2)
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
impl W {
    #[doc = "Bit 0 - Clear Source Event 0/1"]
    #[inline(always)]
    pub fn sev0(&mut self) -> SEV0_W {
        SEV0_W { w: self }
    }
    #[doc = "Bit 1 - Clear Source Event 0/1"]
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
#[doc = "Source Event Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sefclr](index.html) module"]
pub struct SEFCLR_SPEC;
impl crate::RegisterSpec for SEFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sefclr::W](W) writer structure"]
impl crate::Writable for SEFCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEFCLR to value 0"]
impl crate::Resettable for SEFCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
