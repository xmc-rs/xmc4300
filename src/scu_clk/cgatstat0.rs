#[doc = "Register `CGATSTAT0` reader"]
pub struct R(crate::R<CGATSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGATSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGATSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGATSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "VADC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADC_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<VADC_A> for bool {
    #[inline(always)]
    fn from(variant: VADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADC` reader - VADC Gating Status"]
pub struct VADC_R(crate::FieldReader<bool, VADC_A>);
impl VADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADC_A {
        match self.bits {
            false => VADC_A::CONST_0,
            true => VADC_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == VADC_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == VADC_A::CONST_1
    }
}
impl core::ops::Deref for VADC_R {
    type Target = crate::FieldReader<bool, VADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU40 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<CCU40_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40` reader - CCU40 Gating Status"]
pub struct CCU40_R(crate::FieldReader<bool, CCU40_A>);
impl CCU40_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU40_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40_A {
        match self.bits {
            false => CCU40_A::CONST_0,
            true => CCU40_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCU40_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCU40_A::CONST_1
    }
}
impl core::ops::Deref for CCU40_R {
    type Target = crate::FieldReader<bool, CCU40_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU41 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<CCU41_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41` reader - CCU41 Gating Status"]
pub struct CCU41_R(crate::FieldReader<bool, CCU41_A>);
impl CCU41_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU41_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU41_A {
        match self.bits {
            false => CCU41_A::CONST_0,
            true => CCU41_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCU41_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCU41_A::CONST_1
    }
}
impl core::ops::Deref for CCU41_R {
    type Target = crate::FieldReader<bool, CCU41_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU80 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<CCU80_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80` reader - CCU80 Gating Status"]
pub struct CCU80_R(crate::FieldReader<bool, CCU80_A>);
impl CCU80_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU80_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU80_A {
        match self.bits {
            false => CCU80_A::CONST_0,
            true => CCU80_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCU80_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCU80_A::CONST_1
    }
}
impl core::ops::Deref for CCU80_R {
    type Target = crate::FieldReader<bool, CCU80_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "POSIF0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<POSIF0_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0` reader - POSIF0 Gating Status"]
pub struct POSIF0_R(crate::FieldReader<bool, POSIF0_A>);
impl POSIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        POSIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF0_A {
        match self.bits {
            false => POSIF0_A::CONST_0,
            true => POSIF0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == POSIF0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == POSIF0_A::CONST_1
    }
}
impl core::ops::Deref for POSIF0_R {
    type Target = crate::FieldReader<bool, POSIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USIC0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<USIC0_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0` reader - USIC0 Gating Status"]
pub struct USIC0_R(crate::FieldReader<bool, USIC0_A>);
impl USIC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        USIC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0_A {
        match self.bits {
            false => USIC0_A::CONST_0,
            true => USIC0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == USIC0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == USIC0_A::CONST_1
    }
}
impl core::ops::Deref for USIC0_R {
    type Target = crate::FieldReader<bool, USIC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ERU1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1_A {
    #[doc = "0: Gating de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Gating asserted"]
    CONST_1 = 1,
}
impl From<ERU1_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1` reader - ERU1 Gating Status"]
pub struct ERU1_R(crate::FieldReader<bool, ERU1_A>);
impl ERU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERU1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU1_A {
        match self.bits {
            false => ERU1_A::CONST_0,
            true => ERU1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ERU1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ERU1_A::CONST_1
    }
}
impl core::ops::Deref for ERU1_R {
    type Target = crate::FieldReader<bool, ERU1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline(always)]
    pub fn vadc(&self) -> VADC_R {
        VADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline(always)]
    pub fn ccu40(&self) -> CCU40_R {
        CCU40_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline(always)]
    pub fn ccu41(&self) -> CCU41_R {
        CCU41_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline(always)]
    pub fn ccu80(&self) -> CCU80_R {
        CCU80_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline(always)]
    pub fn posif0(&self) -> POSIF0_R {
        POSIF0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline(always)]
    pub fn usic0(&self) -> USIC0_R {
        USIC0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline(always)]
    pub fn eru1(&self) -> ERU1_R {
        ERU1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Peripheral 0 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat0](index.html) module"]
pub struct CGATSTAT0_SPEC;
impl crate::RegisterSpec for CGATSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgatstat0::R](R) reader structure"]
impl crate::Readable for CGATSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CGATSTAT0 to value 0"]
impl crate::Resettable for CGATSTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
