#[doc = "Register `MCHKCON` reader"]
pub struct R(crate::R<MCHKCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCHKCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCHKCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCHKCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCHKCON` writer"]
pub struct W(crate::W<MCHKCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCHKCON_SPEC>;
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
impl From<crate::W<MCHKCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCHKCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select Memory Check for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELPS_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELPS_A> for bool {
    #[inline(always)]
    fn from(variant: SELPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELPS` reader - Select Memory Check for PSRAM"]
pub struct SELPS_R(crate::FieldReader<bool, SELPS_A>);
impl SELPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELPS_A {
        match self.bits {
            false => SELPS_A::CONST_0,
            true => SELPS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELPS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELPS_A::CONST_1
    }
}
impl core::ops::Deref for SELPS_R {
    type Target = crate::FieldReader<bool, SELPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELPS` writer - Select Memory Check for PSRAM"]
pub struct SELPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELPS_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELPS_A::CONST_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Select Memory Check for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELDS1_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELDS1_A> for bool {
    #[inline(always)]
    fn from(variant: SELDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELDS1` reader - Select Memory Check for DSRAM1"]
pub struct SELDS1_R(crate::FieldReader<bool, SELDS1_A>);
impl SELDS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELDS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELDS1_A {
        match self.bits {
            false => SELDS1_A::CONST_0,
            true => SELDS1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELDS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELDS1_A::CONST_1
    }
}
impl core::ops::Deref for SELDS1_R {
    type Target = crate::FieldReader<bool, SELDS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELDS1` writer - Select Memory Check for DSRAM1"]
pub struct SELDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SELDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELDS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELDS1_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELDS1_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Select Memory Check for USIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0DRA_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<USIC0DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0DRA` reader - Select Memory Check for USIC0"]
pub struct USIC0DRA_R(crate::FieldReader<bool, USIC0DRA_A>);
impl USIC0DRA_R {
    pub(crate) fn new(bits: bool) -> Self {
        USIC0DRA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0DRA_A {
        match self.bits {
            false => USIC0DRA_A::CONST_0,
            true => USIC0DRA_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == USIC0DRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == USIC0DRA_A::CONST_1
    }
}
impl core::ops::Deref for USIC0DRA_R {
    type Target = crate::FieldReader<bool, USIC0DRA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USIC0DRA` writer - Select Memory Check for USIC0"]
pub struct USIC0DRA_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC0DRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC0DRA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC0DRA_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC0DRA_A::CONST_1)
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
#[doc = "Select Memory Check for USIC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1DRA_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<USIC1DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1DRA` reader - Select Memory Check for USIC1"]
pub struct USIC1DRA_R(crate::FieldReader<bool, USIC1DRA_A>);
impl USIC1DRA_R {
    pub(crate) fn new(bits: bool) -> Self {
        USIC1DRA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC1DRA_A {
        match self.bits {
            false => USIC1DRA_A::CONST_0,
            true => USIC1DRA_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == USIC1DRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == USIC1DRA_A::CONST_1
    }
}
impl core::ops::Deref for USIC1DRA_R {
    type Target = crate::FieldReader<bool, USIC1DRA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USIC1DRA` writer - Select Memory Check for USIC1"]
pub struct USIC1DRA_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC1DRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC1DRA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC1DRA_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC1DRA_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Select Memory Check for MultiCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCANDRA_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<MCANDRA_A> for bool {
    #[inline(always)]
    fn from(variant: MCANDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANDRA` reader - Select Memory Check for MultiCAN"]
pub struct MCANDRA_R(crate::FieldReader<bool, MCANDRA_A>);
impl MCANDRA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCANDRA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCANDRA_A {
        match self.bits {
            false => MCANDRA_A::CONST_0,
            true => MCANDRA_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MCANDRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MCANDRA_A::CONST_1
    }
}
impl core::ops::Deref for MCANDRA_R {
    type Target = crate::FieldReader<bool, MCANDRA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCANDRA` writer - Select Memory Check for MultiCAN"]
pub struct MCANDRA_W<'a> {
    w: &'a mut W,
}
impl<'a> MCANDRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCANDRA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MCANDRA_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MCANDRA_A::CONST_1)
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
#[doc = "Select Memory Check for PMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRFDRA_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<PPRFDRA_A> for bool {
    #[inline(always)]
    fn from(variant: PPRFDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPRFDRA` reader - Select Memory Check for PMU"]
pub struct PPRFDRA_R(crate::FieldReader<bool, PPRFDRA_A>);
impl PPRFDRA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPRFDRA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPRFDRA_A {
        match self.bits {
            false => PPRFDRA_A::CONST_0,
            true => PPRFDRA_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPRFDRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPRFDRA_A::CONST_1
    }
}
impl core::ops::Deref for PPRFDRA_R {
    type Target = crate::FieldReader<bool, PPRFDRA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPRFDRA` writer - Select Memory Check for PMU"]
pub struct PPRFDRA_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRFDRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPRFDRA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPRFDRA_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPRFDRA_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Select Memory Check for USB SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELUSB_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELUSB_A> for bool {
    #[inline(always)]
    fn from(variant: SELUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELUSB` reader - Select Memory Check for USB SRAM"]
pub struct SELUSB_R(crate::FieldReader<bool, SELUSB_A>);
impl SELUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELUSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELUSB_A {
        match self.bits {
            false => SELUSB_A::CONST_0,
            true => SELUSB_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELUSB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELUSB_A::CONST_1
    }
}
impl core::ops::Deref for SELUSB_R {
    type Target = crate::FieldReader<bool, SELUSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELUSB` writer - Select Memory Check for USB SRAM"]
pub struct SELUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SELUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELUSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELUSB_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELUSB_A::CONST_1)
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
#[doc = "Select Memory Check for ETH0 TX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELETH0TX_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: SELETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELETH0TX` reader - Select Memory Check for ETH0 TX SRAM"]
pub struct SELETH0TX_R(crate::FieldReader<bool, SELETH0TX_A>);
impl SELETH0TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELETH0TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELETH0TX_A {
        match self.bits {
            false => SELETH0TX_A::CONST_0,
            true => SELETH0TX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELETH0TX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELETH0TX_A::CONST_1
    }
}
impl core::ops::Deref for SELETH0TX_R {
    type Target = crate::FieldReader<bool, SELETH0TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELETH0TX` writer - Select Memory Check for ETH0 TX SRAM"]
pub struct SELETH0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELETH0TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELETH0TX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELETH0TX_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELETH0TX_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Select Memory Check for ETH0 RX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELETH0RX_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: SELETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELETH0RX` reader - Select Memory Check for ETH0 RX SRAM"]
pub struct SELETH0RX_R(crate::FieldReader<bool, SELETH0RX_A>);
impl SELETH0RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELETH0RX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELETH0RX_A {
        match self.bits {
            false => SELETH0RX_A::CONST_0,
            true => SELETH0RX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELETH0RX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELETH0RX_A::CONST_1
    }
}
impl core::ops::Deref for SELETH0RX_R {
    type Target = crate::FieldReader<bool, SELETH0RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELETH0RX` writer - Select Memory Check for ETH0 RX SRAM"]
pub struct SELETH0RX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELETH0RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELETH0RX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELETH0RX_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELETH0RX_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Select Memory Check for SDMMC SRAM 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELSD0_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELSD0_A> for bool {
    #[inline(always)]
    fn from(variant: SELSD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELSD0` reader - Select Memory Check for SDMMC SRAM 0"]
pub struct SELSD0_R(crate::FieldReader<bool, SELSD0_A>);
impl SELSD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELSD0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELSD0_A {
        match self.bits {
            false => SELSD0_A::CONST_0,
            true => SELSD0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELSD0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELSD0_A::CONST_1
    }
}
impl core::ops::Deref for SELSD0_R {
    type Target = crate::FieldReader<bool, SELSD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELSD0` writer - Select Memory Check for SDMMC SRAM 0"]
pub struct SELSD0_W<'a> {
    w: &'a mut W,
}
impl<'a> SELSD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELSD0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELSD0_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELSD0_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Select Memory Check for SDMMC SRAM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELSD1_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELSD1_A> for bool {
    #[inline(always)]
    fn from(variant: SELSD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELSD1` reader - Select Memory Check for SDMMC SRAM 1"]
pub struct SELSD1_R(crate::FieldReader<bool, SELSD1_A>);
impl SELSD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELSD1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELSD1_A {
        match self.bits {
            false => SELSD1_A::CONST_0,
            true => SELSD1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELSD1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELSD1_A::CONST_1
    }
}
impl core::ops::Deref for SELSD1_R {
    type Target = crate::FieldReader<bool, SELSD1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELSD1` writer - Select Memory Check for SDMMC SRAM 1"]
pub struct SELSD1_W<'a> {
    w: &'a mut W,
}
impl<'a> SELSD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELSD1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELSD1_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELSD1_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Select Memory Check for ECAT0 SRAM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECAT0_A {
    #[doc = "0: Not selected"]
    CONST_0 = 0,
    #[doc = "1: Selected"]
    CONST_1 = 1,
}
impl From<SELECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: SELECAT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECAT0` reader - Select Memory Check for ECAT0 SRAM 1"]
pub struct SELECAT0_R(crate::FieldReader<bool, SELECAT0_A>);
impl SELECAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELECAT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELECAT0_A {
        match self.bits {
            false => SELECAT0_A::CONST_0,
            true => SELECAT0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == SELECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == SELECAT0_A::CONST_1
    }
}
impl core::ops::Deref for SELECAT0_R {
    type Target = crate::FieldReader<bool, SELECAT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELECAT0` writer - Select Memory Check for ECAT0 SRAM 1"]
pub struct SELECAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELECAT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELECAT0_A::CONST_0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELECAT0_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&self) -> SELPS_R {
        SELPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&self) -> SELDS1_R {
        SELDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&self) -> USIC0DRA_R {
        USIC0DRA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&self) -> USIC1DRA_R {
        USIC1DRA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&self) -> MCANDRA_R {
        MCANDRA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&self) -> PPRFDRA_R {
        PPRFDRA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&self) -> SELUSB_R {
        SELUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    pub fn seleth0tx(&self) -> SELETH0TX_R {
        SELETH0TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    pub fn seleth0rx(&self) -> SELETH0RX_R {
        SELETH0RX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    pub fn selsd0(&self) -> SELSD0_R {
        SELSD0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    pub fn selsd1(&self) -> SELSD1_R {
        SELSD1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline(always)]
    pub fn selecat0(&self) -> SELECAT0_R {
        SELECAT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&mut self) -> SELPS_W {
        SELPS_W { w: self }
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&mut self) -> SELDS1_W {
        SELDS1_W { w: self }
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&mut self) -> USIC0DRA_W {
        USIC0DRA_W { w: self }
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&mut self) -> USIC1DRA_W {
        USIC1DRA_W { w: self }
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&mut self) -> MCANDRA_W {
        MCANDRA_W { w: self }
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&mut self) -> PPRFDRA_W {
        PPRFDRA_W { w: self }
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&mut self) -> SELUSB_W {
        SELUSB_W { w: self }
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    pub fn seleth0tx(&mut self) -> SELETH0TX_W {
        SELETH0TX_W { w: self }
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    pub fn seleth0rx(&mut self) -> SELETH0RX_W {
        SELETH0RX_W { w: self }
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    pub fn selsd0(&mut self) -> SELSD0_W {
        SELSD0_W { w: self }
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    pub fn selsd1(&mut self) -> SELSD1_W {
        SELSD1_W { w: self }
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline(always)]
    pub fn selecat0(&mut self) -> SELECAT0_W {
        SELECAT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Checking Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mchkcon](index.html) module"]
pub struct MCHKCON_SPEC;
impl crate::RegisterSpec for MCHKCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mchkcon::R](R) reader structure"]
impl crate::Readable for MCHKCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mchkcon::W](W) writer structure"]
impl crate::Writable for MCHKCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCHKCON to value 0"]
impl crate::Resettable for MCHKCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
