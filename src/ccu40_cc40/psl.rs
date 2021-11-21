#[doc = "Register `PSL` reader"]
pub struct R(crate::R<PSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSL` writer"]
pub struct W(crate::W<PSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSL_SPEC>;
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
impl From<crate::W<PSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Output Passive Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL_A> for bool {
    #[inline(always)]
    fn from(variant: PSL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL` reader - Output Passive Level"]
pub struct PSL_R(crate::FieldReader<bool, PSL_A>);
impl PSL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL_A {
        match self.bits {
            false => PSL_A::VALUE1,
            true => PSL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PSL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSL_A::VALUE2
    }
}
impl core::ops::Deref for PSL_R {
    type Target = crate::FieldReader<bool, PSL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSL` writer - Output Passive Level"]
pub struct PSL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&self) -> PSL_R {
        PSL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&mut self) -> PSL_W {
        PSL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Passive Level Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psl](index.html) module"]
pub struct PSL_SPEC;
impl crate::RegisterSpec for PSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psl::R](R) reader structure"]
impl crate::Readable for PSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psl::W](W) writer structure"]
impl crate::Writable for PSL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSL to value 0"]
impl crate::Resettable for PSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
