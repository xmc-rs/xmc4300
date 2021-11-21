#[doc = "Register `PRSTAT0` reader"]
pub struct R(crate::R<PRSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "VADC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADCRS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Reset asserted"]
    CONST_1 = 1,
}
impl From<VADCRS_A> for bool {
    #[inline(always)]
    fn from(variant: VADCRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` reader - VADC Reset Status"]
pub struct VADCRS_R(crate::FieldReader<bool, VADCRS_A>);
impl VADCRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VADCRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADCRS_A {
        match self.bits {
            false => VADCRS_A::CONST_0,
            true => VADCRS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == VADCRS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == VADCRS_A::CONST_1
    }
}
impl core::ops::Deref for VADCRS_R {
    type Target = crate::FieldReader<bool, VADCRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU40 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Reset asserted"]
    CONST_1 = 1,
}
impl From<CCU40RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` reader - CCU40 Reset Status"]
pub struct CCU40RS_R(crate::FieldReader<bool, CCU40RS_A>);
impl CCU40RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU40RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40RS_A {
        match self.bits {
            false => CCU40RS_A::CONST_0,
            true => CCU40RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCU40RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCU40RS_A::CONST_1
    }
}
impl core::ops::Deref for CCU40RS_R {
    type Target = crate::FieldReader<bool, CCU40RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU41 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Reset asserted"]
    CONST_1 = 1,
}
impl From<CCU41RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` reader - CCU41 Reset Status"]
pub struct CCU41RS_R(crate::FieldReader<bool, CCU41RS_A>);
impl CCU41RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU41RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU41RS_A {
        match self.bits {
            false => CCU41RS_A::CONST_0,
            true => CCU41RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCU41RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCU41RS_A::CONST_1
    }
}
impl core::ops::Deref for CCU41RS_R {
    type Target = crate::FieldReader<bool, CCU41RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU80 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Reset asserted"]
    CONST_1 = 1,
}
impl From<CCU80RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` reader - CCU80 Reset Status"]
pub struct CCU80RS_R(crate::FieldReader<bool, CCU80RS_A>);
impl CCU80RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU80RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU80RS_A {
        match self.bits {
            false => CCU80RS_A::CONST_0,
            true => CCU80RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == CCU80RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == CCU80RS_A::CONST_1
    }
}
impl core::ops::Deref for CCU80RS_R {
    type Target = crate::FieldReader<bool, CCU80RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USIC0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Reset asserted"]
    CONST_1 = 1,
}
impl From<USIC0RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` reader - USIC0 Reset Status"]
pub struct USIC0RS_R(crate::FieldReader<bool, USIC0RS_A>);
impl USIC0RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        USIC0RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0RS_A {
        match self.bits {
            false => USIC0RS_A::CONST_0,
            true => USIC0RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == USIC0RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == USIC0RS_A::CONST_1
    }
}
impl core::ops::Deref for USIC0RS_R {
    type Target = crate::FieldReader<bool, USIC0RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ERU1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Reset asserted"]
    CONST_1 = 1,
}
impl From<ERU1RS_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` reader - ERU1 Reset Status"]
pub struct ERU1RS_R(crate::FieldReader<bool, ERU1RS_A>);
impl ERU1RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERU1RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU1RS_A {
        match self.bits {
            false => ERU1RS_A::CONST_0,
            true => ERU1RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ERU1RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ERU1RS_A::CONST_1
    }
}
impl core::ops::Deref for ERU1RS_R {
    type Target = crate::FieldReader<bool, ERU1RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - VADC Reset Status"]
    #[inline(always)]
    pub fn vadcrs(&self) -> VADCRS_R {
        VADCRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline(always)]
    pub fn ccu40rs(&self) -> CCU40RS_R {
        CCU40RS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline(always)]
    pub fn ccu41rs(&self) -> CCU41RS_R {
        CCU41RS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline(always)]
    pub fn ccu80rs(&self) -> CCU80RS_R {
        CCU80RS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline(always)]
    pub fn usic0rs(&self) -> USIC0RS_R {
        USIC0RS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline(always)]
    pub fn eru1rs(&self) -> ERU1RS_R {
        ERU1RS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "RCU Peripheral 0 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat0](index.html) module"]
pub struct PRSTAT0_SPEC;
impl crate::RegisterSpec for PRSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstat0::R](R) reader structure"]
impl crate::Readable for PRSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTAT0 to value 0x0001_0f9f"]
impl crate::Resettable for PRSTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0f9f
    }
}
