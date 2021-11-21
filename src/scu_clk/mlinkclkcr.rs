#[doc = "Register `MLINKCLKCR` reader"]
pub struct R(crate::R<MLINKCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLINKCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLINKCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLINKCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLINKCLKCR` writer"]
pub struct W(crate::W<MLINKCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLINKCLKCR_SPEC>;
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
impl From<crate::W<MLINKCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLINKCLKCR_SPEC>) -> Self {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUDIV_A {
    #[doc = "0: fCPU = fSYS"]
    CONST_0 = 0,
    #[doc = "1: fCPU = fSYS / 2"]
    CONST_1 = 1,
}
impl From<CPUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub struct CPUDIV_R(crate::FieldReader<bool, CPUDIV_A>);
impl CPUDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPUDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUDIV_A {
        match self.bits {
            false => CPUDIV_A::CONST_0,
            true => CPUDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CPUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CPUDIV_A::CONST_1
    }
}
impl core::ops::Deref for CPUDIV_R {
    type Target = crate::FieldReader<bool, CPUDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Divider Enable"]
pub struct CPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUDIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CPUDIV_A::CONST_0)
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CPUDIV_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBDIV_A {
    #[doc = "0: fPERIPH = fCPU"]
    CONST_0 = 0,
    #[doc = "1: fPERIPH = fCPU / 2"]
    CONST_1 = 1,
}
impl From<PBDIV_A> for bool {
    #[inline(always)]
    fn from(variant: PBDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBDIV` reader - PB Clock Divider Enable"]
pub struct PBDIV_R(crate::FieldReader<bool, PBDIV_A>);
impl PBDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBDIV_A {
        match self.bits {
            false => PBDIV_A::CONST_0,
            true => PBDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PBDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PBDIV_A::CONST_1
    }
}
impl core::ops::Deref for PBDIV_R {
    type Target = crate::FieldReader<bool, PBDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBDIV` writer - PB Clock Divider Enable"]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBDIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PBDIV_A::CONST_0)
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PBDIV_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUDIV_A {
    #[doc = "0: fCCU = fSYS"]
    CONST_0 = 0,
    #[doc = "1: fCCU = fSYS / 2"]
    CONST_1 = 1,
}
impl From<CCUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CCUDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUDIV` reader - CCU Clock Divider Enable"]
pub struct CCUDIV_R(crate::FieldReader<bool, CCUDIV_A>);
impl CCUDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCUDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUDIV_A {
        match self.bits {
            false => CCUDIV_A::CONST_0,
            true => CCUDIV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCUDIV_A::CONST_1
    }
}
impl core::ops::Deref for CCUDIV_R {
    type Target = crate::FieldReader<bool, CCUDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCUDIV` writer - CCU Clock Divider Enable"]
pub struct CCUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUDIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUDIV_A::CONST_0)
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUDIV_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
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
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CCUDIV_R {
        CCUDIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&mut self) -> SYSDIV_W {
        SYSDIV_W { w: self }
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W {
        SYSSEL_W { w: self }
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W {
        CPUDIV_W { w: self }
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&mut self) -> CCUDIV_W {
        CCUDIV_W { w: self }
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&mut self) -> WDTDIV_W {
        WDTDIV_W { w: self }
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
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
#[doc = "Multi-Link Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlinkclkcr](index.html) module"]
pub struct MLINKCLKCR_SPEC;
impl crate::RegisterSpec for MLINKCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlinkclkcr::R](R) reader structure"]
impl crate::Readable for MLINKCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlinkclkcr::W](W) writer structure"]
impl crate::Writable for MLINKCLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLINKCLKCR to value 0"]
impl crate::Resettable for MLINKCLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
