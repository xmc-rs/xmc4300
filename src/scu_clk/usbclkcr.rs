#[doc = "Register `USBCLKCR` reader"]
pub struct R(crate::R<USBCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCLKCR` writer"]
pub struct W(crate::W<USBCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCLKCR_SPEC>;
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
impl From<crate::W<USBCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBDIV` reader - USB Clock Divider Value"]
pub struct USBDIV_R(crate::FieldReader<u8, u8>);
impl USBDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDIV` writer - USB Clock Divider Value"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "USB Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSEL_A {
    #[doc = "0: USB PLL Clock"]
    CONST_0 = 0,
    #[doc = "1: PLL Clock"]
    CONST_1 = 1,
}
impl From<USBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSEL` reader - USB Clock Selection Value"]
pub struct USBSEL_R(crate::FieldReader<bool, USBSEL_A>);
impl USBSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSEL_A {
        match self.bits {
            false => USBSEL_A::CONST_0,
            true => USBSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == USBSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == USBSEL_A::CONST_1
    }
}
impl core::ops::Deref for USBSEL_R {
    type Target = crate::FieldReader<bool, USBSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSEL` writer - USB Clock Selection Value"]
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB PLL Clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBSEL_A::CONST_0)
    }
    #[doc = "PLL Clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBSEL_A::CONST_1)
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
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W {
        USBSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkcr](index.html) module"]
pub struct USBCLKCR_SPEC;
impl crate::RegisterSpec for USBCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbclkcr::R](R) reader structure"]
impl crate::Readable for USBCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbclkcr::W](W) writer structure"]
impl crate::Writable for USBCLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCLKCR to value 0"]
impl crate::Resettable for USBCLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
