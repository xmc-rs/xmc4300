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
pub struct SYSDIV_R(crate::FieldReader<u8, u8>);
impl SYSDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYSDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSDIV` writer - System Clock Division Value"]
pub struct SYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub struct SYSSEL_R(crate::FieldReader<bool, SYSSEL_A>);
impl SYSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SYSSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SYSSEL_A::CONST_1
    }
}
impl core::ops::Deref for SYSSEL_R {
    type Target = crate::FieldReader<bool, SYSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub struct SYSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&mut self) -> SYSDIV_W {
        SYSDIV_W { w: self }
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W {
        SYSSEL_W { w: self }
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
}
#[doc = "`reset()` method sets SYSCLKCR to value 0"]
impl crate::Resettable for SYSCLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
