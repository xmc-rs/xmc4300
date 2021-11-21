#[doc = "Register `CLKSTAT` reader"]
pub struct R(crate::R<CLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "USB Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCST_A {
    #[doc = "0: Clock disabled"]
    CONST_0 = 0,
    #[doc = "1: Clock enabled"]
    CONST_1 = 1,
}
impl From<USBCST_A> for bool {
    #[inline(always)]
    fn from(variant: USBCST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCST` reader - USB Clock Status"]
pub struct USBCST_R(crate::FieldReader<bool, USBCST_A>);
impl USBCST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCST_A {
        match self.bits {
            false => USBCST_A::CONST_0,
            true => USBCST_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == USBCST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == USBCST_A::CONST_1
    }
}
impl core::ops::Deref for USBCST_R {
    type Target = crate::FieldReader<bool, USBCST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MMC Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCST_A {
    #[doc = "0: Clock disabled"]
    CONST_0 = 0,
    #[doc = "1: Clock enabled"]
    CONST_1 = 1,
}
impl From<MMCCST_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCST` reader - MMC Clock Status"]
pub struct MMCCST_R(crate::FieldReader<bool, MMCCST_A>);
impl MMCCST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCCST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMCCST_A {
        match self.bits {
            false => MMCCST_A::CONST_0,
            true => MMCCST_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MMCCST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MMCCST_A::CONST_1
    }
}
impl core::ops::Deref for MMCCST_R {
    type Target = crate::FieldReader<bool, MMCCST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Ethernet Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CST_A {
    #[doc = "0: Clock disabled"]
    CONST_0 = 0,
    #[doc = "1: Clock enabled"]
    CONST_1 = 1,
}
impl From<ETH0CST_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CST` reader - Ethernet Clock Status"]
pub struct ETH0CST_R(crate::FieldReader<bool, ETH0CST_A>);
impl ETH0CST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH0CST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH0CST_A {
        match self.bits {
            false => ETH0CST_A::CONST_0,
            true => ETH0CST_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ETH0CST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ETH0CST_A::CONST_1
    }
}
impl core::ops::Deref for ETH0CST_R {
    type Target = crate::FieldReader<bool, ETH0CST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCST_A {
    #[doc = "0: Clock disabled"]
    CONST_0 = 0,
    #[doc = "1: Clock enabled"]
    CONST_1 = 1,
}
impl From<CCUCST_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCST` reader - CCU Clock Status"]
pub struct CCUCST_R(crate::FieldReader<bool, CCUCST_A>);
impl CCUCST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCUCST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUCST_A {
        match self.bits {
            false => CCUCST_A::CONST_0,
            true => CCUCST_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCUCST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCUCST_A::CONST_1
    }
}
impl core::ops::Deref for CCUCST_R {
    type Target = crate::FieldReader<bool, CCUCST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WDT Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCST_A {
    #[doc = "0: Clock disabled"]
    CONST_0 = 0,
    #[doc = "1: Clock enabled"]
    CONST_1 = 1,
}
impl From<WDTCST_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCST` reader - WDT Clock Status"]
pub struct WDTCST_R(crate::FieldReader<bool, WDTCST_A>);
impl WDTCST_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTCST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCST_A {
        match self.bits {
            false => WDTCST_A::CONST_0,
            true => WDTCST_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == WDTCST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == WDTCST_A::CONST_1
    }
}
impl core::ops::Deref for WDTCST_R {
    type Target = crate::FieldReader<bool, WDTCST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline(always)]
    pub fn usbcst(&self) -> USBCST_R {
        USBCST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Clock Status"]
    #[inline(always)]
    pub fn mmccst(&self) -> MMCCST_R {
        MMCCST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Ethernet Clock Status"]
    #[inline(always)]
    pub fn eth0cst(&self) -> ETH0CST_R {
        ETH0CST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline(always)]
    pub fn ccucst(&self) -> CCUCST_R {
        CCUCST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline(always)]
    pub fn wdtcst(&self) -> WDTCST_R {
        WDTCST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstat](index.html) module"]
pub struct CLKSTAT_SPEC;
impl crate::RegisterSpec for CLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkstat::R](R) reader structure"]
impl crate::Readable for CLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLKSTAT to value 0"]
impl crate::Resettable for CLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
