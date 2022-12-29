#[doc = "Register `SYSCLKCR` reader"]
pub struct R(crate::R<SYSCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCLKCR` writer"]
pub struct W(crate::W<SYSCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCLKCR_SPEC>;
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
impl From<crate::W<SYSCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSDIV` reader - System Clock Division Value"]
pub type SYSDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSDIV` writer - System Clock Division Value"]
pub type SYSDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCLKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SYSSEL_R = crate::BitReader<SYSSEL_A>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_0 = 0,
    #[doc = "1: fPLL clock"]
    CONST_1 = 1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::CONST_0,
            true => SYSSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SYSSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SYSSEL_A::CONST_1
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SYSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCLKCR_SPEC, SYSSEL_A, O>;
impl<'a, const O: u8> SYSSEL_W<'a, O> {
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysdiv(&mut self) -> SYSDIV_W<0> {
        SYSDIV_W::new(self)
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SYSSEL_W<16> {
        SYSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclkcr](index.html) module"]
pub struct SYSCLKCR_SPEC;
impl crate::RegisterSpec for SYSCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysclkcr::R](R) reader structure"]
impl crate::Readable for SYSCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysclkcr::W](W) writer structure"]
impl crate::Writable for SYSCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCLKCR to value 0"]
impl crate::Resettable for SYSCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
