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
pub type AM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AM` writer - Acceptance Mask for Message Identifier"]
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOAMR_SPEC, u32, u32, 29, O>;
#[doc = "Field `MIDE` reader - Acceptance Mask Bit for Message IDE Bit"]
pub type MIDE_R = crate::BitReader<MIDE_A>;
#[doc = "Acceptance Mask Bit for Message IDE Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MIDE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MIDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIDE_A::VALUE2
    }
}
#[doc = "Field `MIDE` writer - Acceptance Mask Bit for Message IDE Bit"]
pub type MIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOAMR_SPEC, MIDE_A, O>;
impl<'a, const O: u8> MIDE_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<0> {
        AM_W::new(self)
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mide(&mut self) -> MIDE_W<29> {
        MIDE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOAMR to value 0x3fff_ffff"]
impl crate::Resettable for MOAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_ffff;
}
