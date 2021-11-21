#[doc = "Register `WDTCLKCR` reader"]
pub struct R(crate::R<WDTCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCLKCR` writer"]
pub struct W(crate::W<WDTCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCLKCR_SPEC>;
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
impl From<crate::W<WDTCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub struct WDTDIV_R(crate::FieldReader<u8, u8>);
impl WDTDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub struct WDTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_00 = 0,
    #[doc = "1: fSTDBY clock"]
    CONST_01 = 1,
    #[doc = "2: fPLL clock"]
    CONST_10 = 2,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub struct WDTSEL_R(crate::FieldReader<u8, WDTSEL_A>);
impl WDTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDTSEL_A> {
        match self.bits {
            0 => Some(WDTSEL_A::CONST_00),
            1 => Some(WDTSEL_A::CONST_01),
            2 => Some(WDTSEL_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        **self == WDTSEL_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        **self == WDTSEL_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        **self == WDTSEL_A::CONST_10
    }
}
impl core::ops::Deref for WDTSEL_R {
    type Target = crate::FieldReader<u8, WDTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub struct WDTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_00)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_01)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut W {
        self.variant(WDTSEL_A::CONST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&mut self) -> WDTDIV_W {
        WDTDIV_W { w: self }
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&mut self) -> WDTSEL_W {
        WDTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclkcr](index.html) module"]
pub struct WDTCLKCR_SPEC;
impl crate::RegisterSpec for WDTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtclkcr::R](R) reader structure"]
impl crate::Readable for WDTCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtclkcr::W](W) writer structure"]
impl crate::Writable for WDTCLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCLKCR to value 0"]
impl crate::Resettable for WDTCLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
