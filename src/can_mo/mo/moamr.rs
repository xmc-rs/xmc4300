#[doc = "Register `MOAMR` reader"]
pub struct R(crate::R<MOAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOAMR` writer"]
pub struct W(crate::W<MOAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOAMR_SPEC>;
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
impl From<crate::W<MOAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AM` reader - Acceptance Mask for Message Identifier"]
pub struct AM_R(crate::FieldReader<u32, u32>);
impl AM_R {
    pub(crate) fn new(bits: u32) -> Self {
        AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM` writer - Acceptance Mask for Message Identifier"]
pub struct AM_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Acceptance Mask Bit for Message IDE Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIDE_A {
    #[doc = "0: Message object n accepts the reception of both, standard and extended frames."]
    VALUE1 = 0,
    #[doc = "1: Message object n receives frames only with matching IDE bit."]
    VALUE2 = 1,
}
impl From<MIDE_A> for bool {
    #[inline(always)]
    fn from(variant: MIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIDE` reader - Acceptance Mask Bit for Message IDE Bit"]
pub struct MIDE_R(crate::FieldReader<bool, MIDE_A>);
impl MIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIDE_A {
        match self.bits {
            false => MIDE_A::VALUE1,
            true => MIDE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIDE_A::VALUE2
    }
}
impl core::ops::Deref for MIDE_R {
    type Target = crate::FieldReader<bool, MIDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIDE` writer - Acceptance Mask Bit for Message IDE Bit"]
pub struct MIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Message object n accepts the reception of both, standard and extended frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MIDE_A::VALUE1)
    }
    #[doc = "Message object n receives frames only with matching IDE bit."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MIDE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W {
        AM_W { w: self }
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    pub fn mide(&mut self) -> MIDE_W {
        MIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Acceptance Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moamr](index.html) module"]
pub struct MOAMR_SPEC;
impl crate::RegisterSpec for MOAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moamr::R](R) reader structure"]
impl crate::Readable for MOAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moamr::W](W) writer structure"]
impl crate::Writable for MOAMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOAMR to value 0x3fff_ffff"]
impl crate::Resettable for MOAMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff_ffff
    }
}
