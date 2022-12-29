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
pub type SYSDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSDIV` writer - System Clock Division Value"]
pub type SYSDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MLINKCLKCR_SPEC, u8, u8, 8, O>;
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
pub type SYSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MLINKCLKCR_SPEC, SYSSEL_A, O>;
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
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub type CPUDIV_R = crate::BitReader<CPUDIV_A>;
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CPUDIV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CPUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CPUDIV_A::CONST_1
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Divider Enable"]
pub type CPUDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MLINKCLKCR_SPEC, CPUDIV_A, O>;
impl<'a, const O: u8> CPUDIV_W<'a, O> {
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
}
#[doc = "Field `PBDIV` reader - PB Clock Divider Enable"]
pub type PBDIV_R = crate::BitReader<PBDIV_A>;
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PBDIV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PBDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PBDIV_A::CONST_1
    }
}
#[doc = "Field `PBDIV` writer - PB Clock Divider Enable"]
pub type PBDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MLINKCLKCR_SPEC, PBDIV_A, O>;
impl<'a, const O: u8> PBDIV_W<'a, O> {
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
}
#[doc = "Field `CCUDIV` reader - CCU Clock Divider Enable"]
pub type CCUDIV_R = crate::BitReader<CCUDIV_A>;
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CCUDIV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CCUDIV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCUDIV_A::CONST_1
    }
}
#[doc = "Field `CCUDIV` writer - CCU Clock Divider Enable"]
pub type CCUDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MLINKCLKCR_SPEC, CCUDIV_A, O>;
impl<'a, const O: u8> CCUDIV_W<'a, O> {
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
}
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub type WDTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub type WDTDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MLINKCLKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub type WDTSEL_R = crate::FieldReader<u8, WDTSEL_A>;
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WDTSEL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == WDTSEL_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == WDTSEL_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == WDTSEL_A::CONST_10
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub type WDTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MLINKCLKCR_SPEC, u8, WDTSEL_A, 2, O>;
impl<'a, const O: u8> WDTSEL_W<'a, O> {
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
        SYSSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CCUDIV_R {
        CCUDIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysdiv(&mut self) -> SYSDIV_W<0> {
        SYSDIV_W::new(self)
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SYSSEL_W<8> {
        SYSSEL_W::new(self)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CPUDIV_W<10> {
        CPUDIV_W::new(self)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pbdiv(&mut self) -> PBDIV_W<12> {
        PBDIV_W::new(self)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccudiv(&mut self) -> CCUDIV_W<14> {
        CCUDIV_W::new(self)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtdiv(&mut self) -> WDTDIV_W<16> {
        WDTDIV_W::new(self)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtsel(&mut self) -> WDTSEL_W<24> {
        WDTSEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MLINKCLKCR to value 0"]
impl crate::Resettable for MLINKCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
