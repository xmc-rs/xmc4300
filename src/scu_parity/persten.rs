#[doc = "Register `PERSTEN` reader"]
pub struct R(crate::R<PERSTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSTEN` writer"]
pub struct W(crate::W<PERSTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSTEN_SPEC>;
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
impl From<crate::W<PERSTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSEN` reader - System Reset Enable upon Parity Error Trap"]
pub type RSEN_R = crate::BitReader<RSEN_A>;
#[doc = "System Reset Enable upon Parity Error Trap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSEN_A {
    #[doc = "0: Reset request disabled"]
    CONST_0 = 0,
    #[doc = "1: Reset request enabled"]
    CONST_1 = 1,
}
impl From<RSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSEN_A {
        match self.bits {
            false => RSEN_A::CONST_0,
            true => RSEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RSEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RSEN_A::CONST_1
    }
}
#[doc = "Field `RSEN` writer - System Reset Enable upon Parity Error Trap"]
pub type RSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERSTEN_SPEC, RSEN_A, O>;
impl<'a, const O: u8> RSEN_W<'a, O> {
    #[doc = "Reset request disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RSEN_A::CONST_0)
    }
    #[doc = "Reset request enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RSEN_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    pub fn rsen(&self) -> RSEN_R {
        RSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    #[must_use]
    pub fn rsen(&mut self) -> RSEN_W<0> {
        RSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Reset Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [persten](index.html) module"]
pub struct PERSTEN_SPEC;
impl crate::RegisterSpec for PERSTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [persten::R](R) reader structure"]
impl crate::Readable for PERSTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [persten::W](W) writer structure"]
impl crate::Writable for PERSTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSTEN to value 0"]
impl crate::Resettable for PERSTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
