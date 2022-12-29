#[doc = "Register `DC_LATCH0_CONT` reader"]
pub struct R(crate::R<DC_LATCH0_CONT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_LATCH0_CONT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_LATCH0_CONT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_LATCH0_CONT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_LATCH0_CONT` writer"]
pub struct W(crate::W<DC_LATCH0_CONT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_LATCH0_CONT_SPEC>;
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
impl From<crate::W<DC_LATCH0_CONT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_LATCH0_CONT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L0_POS` reader - Latch0 positive edge"]
pub type L0_POS_R = crate::BitReader<L0_POS_A>;
#[doc = "Latch0 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L0_POS_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L0_POS_A> for bool {
    #[inline(always)]
    fn from(variant: L0_POS_A) -> Self {
        variant as u8 != 0
    }
}
impl L0_POS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L0_POS_A {
        match self.bits {
            false => L0_POS_A::VALUE1,
            true => L0_POS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0_POS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0_POS_A::VALUE2
    }
}
#[doc = "Field `L0_POS` writer - Latch0 positive edge"]
pub type L0_POS_W<'a, const O: u8> = crate::BitWriter<'a, u8, DC_LATCH0_CONT_SPEC, L0_POS_A, O>;
impl<'a, const O: u8> L0_POS_W<'a, O> {
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(L0_POS_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(L0_POS_A::VALUE2)
    }
}
#[doc = "Field `L0_NEG` reader - Latch0 negative edge"]
pub type L0_NEG_R = crate::BitReader<L0_NEG_A>;
#[doc = "Latch0 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L0_NEG_A {
    #[doc = "0: Continuous Latch active"]
    VALUE1 = 0,
    #[doc = "1: Single event (only first event active)"]
    VALUE2 = 1,
}
impl From<L0_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: L0_NEG_A) -> Self {
        variant as u8 != 0
    }
}
impl L0_NEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L0_NEG_A {
        match self.bits {
            false => L0_NEG_A::VALUE1,
            true => L0_NEG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L0_NEG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L0_NEG_A::VALUE2
    }
}
#[doc = "Field `L0_NEG` writer - Latch0 negative edge"]
pub type L0_NEG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DC_LATCH0_CONT_SPEC, L0_NEG_A, O>;
impl<'a, const O: u8> L0_NEG_W<'a, O> {
    #[doc = "Continuous Latch active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(L0_NEG_A::VALUE1)
    }
    #[doc = "Single event (only first event active)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(L0_NEG_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    pub fn l0_pos(&self) -> L0_POS_R {
        L0_POS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    pub fn l0_neg(&self) -> L0_NEG_R {
        L0_NEG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latch0 positive edge"]
    #[inline(always)]
    #[must_use]
    pub fn l0_pos(&mut self) -> L0_POS_W<0> {
        L0_POS_W::new(self)
    }
    #[doc = "Bit 1 - Latch0 negative edge"]
    #[inline(always)]
    #[must_use]
    pub fn l0_neg(&mut self) -> L0_NEG_W<1> {
        L0_NEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Latch0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch0_cont](index.html) module"]
pub struct DC_LATCH0_CONT_SPEC;
impl crate::RegisterSpec for DC_LATCH0_CONT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_latch0_cont::R](R) reader structure"]
impl crate::Readable for DC_LATCH0_CONT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_latch0_cont::W](W) writer structure"]
impl crate::Writable for DC_LATCH0_CONT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_LATCH0_CONT to value 0"]
impl crate::Resettable for DC_LATCH0_CONT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
