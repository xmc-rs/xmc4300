#[doc = "Register `ECATCLKCR` reader"]
pub struct R(crate::R<ECATCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECATCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECATCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECATCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECATCLKCR` writer"]
pub struct W(crate::W<ECATCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECATCLKCR_SPEC>;
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
impl From<crate::W<ECATCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECATCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECADIV` reader - EtherCAT Clock Divider Value"]
pub struct ECADIV_R(crate::FieldReader<u8, u8>);
impl ECADIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECADIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECADIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECADIV` writer - EtherCAT Clock Divider Value"]
pub struct ECADIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ECADIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "EtherCAT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECATSEL_A {
    #[doc = "0: fPLLUSB clock"]
    CONST_0 = 0,
    #[doc = "1: fPLL clock"]
    CONST_1 = 1,
}
impl From<ECATSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ECATSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECATSEL` reader - EtherCAT Clock Selection Value"]
pub struct ECATSEL_R(crate::FieldReader<bool, ECATSEL_A>);
impl ECATSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECATSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECATSEL_A {
        match self.bits {
            false => ECATSEL_A::CONST_0,
            true => ECATSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ECATSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ECATSEL_A::CONST_1
    }
}
impl core::ops::Deref for ECATSEL_R {
    type Target = crate::FieldReader<bool, ECATSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECATSEL` writer - EtherCAT Clock Selection Value"]
pub struct ECATSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ECATSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECATSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "fPLLUSB clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECATSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECATSEL_A::CONST_1)
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
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    pub fn ecadiv(&self) -> ECADIV_R {
        ECADIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    pub fn ecatsel(&self) -> ECATSEL_R {
        ECATSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    pub fn ecadiv(&mut self) -> ECADIV_W {
        ECADIV_W { w: self }
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    pub fn ecatsel(&mut self) -> ECATSEL_W {
        ECATSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EtherCAT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecatclkcr](index.html) module"]
pub struct ECATCLKCR_SPEC;
impl crate::RegisterSpec for ECATCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecatclkcr::R](R) reader structure"]
impl crate::Readable for ECATCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecatclkcr::W](W) writer structure"]
impl crate::Writable for ECATCLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECATCLKCR to value 0"]
impl crate::Resettable for ECATCLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
