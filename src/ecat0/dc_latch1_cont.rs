#[doc = "Register `DC_LATCH1_CONT` reader"]
pub struct R(crate::R<DC_LATCH1_CONT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_LATCH1_CONT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_LATCH1_CONT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_LATCH1_CONT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_LATCH1_CONT` writer"]
pub struct W(crate::W<DC_LATCH1_CONT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_LATCH1_CONT_SPEC>;
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
impl From<crate::W<DC_LATCH1_CONT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_LATCH1_CONT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Latch1 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1_POS_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L1_POS_A> for bool {
    #[inline(always)]
    fn from(variant: L1_POS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1_POS` reader - Latch1 positive edge"]
pub struct L1_POS_R(crate::FieldReader<bool, L1_POS_A>);
impl L1_POS_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1_POS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1_POS_A {
        match self.bits {
            false => L1_POS_A::VALUE1,
            true => L1_POS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == L1_POS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == L1_POS_A::VALUE2
    }
}
impl core::ops::Deref for L1_POS_R {
    type Target = crate::FieldReader<bool, L1_POS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1_POS` writer - Latch1 positive edge"]
pub struct L1_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1_POS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L1_POS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(L1_POS_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(L1_POS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Latch1 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1_NEG_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L1_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: L1_NEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1_NEG` reader - Latch1 negative edge"]
pub struct L1_NEG_R(crate::FieldReader<bool, L1_NEG_A>);
impl L1_NEG_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1_NEG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1_NEG_A {
        match self.bits {
            false => L1_NEG_A::VALUE1,
            true => L1_NEG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == L1_NEG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == L1_NEG_A::VALUE2
    }
}
impl core::ops::Deref for L1_NEG_R {
    type Target = crate::FieldReader<bool, L1_NEG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1_NEG` writer - Latch1 negative edge"]
pub struct L1_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> L1_NEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L1_NEG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(L1_NEG_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(L1_NEG_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline(always)]
    pub fn l1_pos(&self) -> L1_POS_R {
        L1_POS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline(always)]
    pub fn l1_neg(&self) -> L1_NEG_R {
        L1_NEG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latch1 positive edge"]
    #[inline(always)]
    pub fn l1_pos(&mut self) -> L1_POS_W {
        L1_POS_W { w: self }
    }
    #[doc = "Bit 1 - Latch1 negative edge"]
    #[inline(always)]
    pub fn l1_neg(&mut self) -> L1_NEG_W {
        L1_NEG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Latch1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch1_cont](index.html) module"]
pub struct DC_LATCH1_CONT_SPEC;
impl crate::RegisterSpec for DC_LATCH1_CONT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_latch1_cont::R](R) reader structure"]
impl crate::Readable for DC_LATCH1_CONT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_latch1_cont::W](W) writer structure"]
impl crate::Writable for DC_LATCH1_CONT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC_LATCH1_CONT to value 0"]
impl crate::Resettable for DC_LATCH1_CONT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
