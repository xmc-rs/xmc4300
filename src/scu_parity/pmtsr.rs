#[doc = "Register `PMTSR` reader"]
pub struct R(crate::R<PMTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMTSR` writer"]
pub struct W(crate::W<PMTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMTSR_SPEC>;
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
impl From<crate::W<PMTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Test Enable Control for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENPS_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTENPS_A> for bool {
    #[inline(always)]
    fn from(variant: MTENPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTENPS` reader - Test Enable Control for PSRAM"]
pub struct MTENPS_R(crate::FieldReader<bool, MTENPS_A>);
impl MTENPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENPS_A {
        match self.bits {
            false => MTENPS_A::CONST_0,
            true => MTENPS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTENPS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTENPS_A::CONST_1
    }
}
impl core::ops::Deref for MTENPS_R {
    type Target = crate::FieldReader<bool, MTENPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENPS` writer - Test Enable Control for PSRAM"]
pub struct MTENPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTENPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTENPS_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTENPS_A::CONST_1)
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
#[doc = "Test Enable Control for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENDS1_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: MTENDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTENDS1` reader - Test Enable Control for DSRAM1"]
pub struct MTENDS1_R(crate::FieldReader<bool, MTENDS1_A>);
impl MTENDS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENDS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENDS1_A {
        match self.bits {
            false => MTENDS1_A::CONST_0,
            true => MTENDS1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTENDS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTENDS1_A::CONST_1
    }
}
impl core::ops::Deref for MTENDS1_R {
    type Target = crate::FieldReader<bool, MTENDS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENDS1` writer - Test Enable Control for DSRAM1"]
pub struct MTENDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTENDS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTENDS1_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTENDS1_A::CONST_1)
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
#[doc = "Test Enable Control for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU0_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTEU0_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEU0` reader - Test Enable Control for USIC0 Memory"]
pub struct MTEU0_R(crate::FieldReader<bool, MTEU0_A>);
impl MTEU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEU0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEU0_A {
        match self.bits {
            false => MTEU0_A::CONST_0,
            true => MTEU0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTEU0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTEU0_A::CONST_1
    }
}
impl core::ops::Deref for MTEU0_R {
    type Target = crate::FieldReader<bool, MTEU0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEU0` writer - Test Enable Control for USIC0 Memory"]
pub struct MTEU0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEU0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTEU0_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTEU0_A::CONST_1)
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
#[doc = "Test Enable Control for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU1_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTEU1_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEU1` reader - Test Enable Control for USIC1 Memory"]
pub struct MTEU1_R(crate::FieldReader<bool, MTEU1_A>);
impl MTEU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEU1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEU1_A {
        match self.bits {
            false => MTEU1_A::CONST_0,
            true => MTEU1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTEU1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTEU1_A::CONST_1
    }
}
impl core::ops::Deref for MTEU1_R {
    type Target = crate::FieldReader<bool, MTEU1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEU1` writer - Test Enable Control for USIC1 Memory"]
pub struct MTEU1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEU1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTEU1_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTEU1_A::CONST_1)
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
#[doc = "Test Enable Control for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEMC_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTEMC_A> for bool {
    #[inline(always)]
    fn from(variant: MTEMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEMC` reader - Test Enable Control for MultiCAN Memory"]
pub struct MTEMC_R(crate::FieldReader<bool, MTEMC_A>);
impl MTEMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEMC_A {
        match self.bits {
            false => MTEMC_A::CONST_0,
            true => MTEMC_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTEMC_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTEMC_A::CONST_1
    }
}
impl core::ops::Deref for MTEMC_R {
    type Target = crate::FieldReader<bool, MTEMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEMC` writer - Test Enable Control for MultiCAN Memory"]
pub struct MTEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEMC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTEMC_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTEMC_A::CONST_1)
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
#[doc = "Test Enable Control for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEPPRF_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: MTEPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEPPRF` reader - Test Enable Control for PMU Prefetch Memory"]
pub struct MTEPPRF_R(crate::FieldReader<bool, MTEPPRF_A>);
impl MTEPPRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEPPRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEPPRF_A {
        match self.bits {
            false => MTEPPRF_A::CONST_0,
            true => MTEPPRF_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTEPPRF_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTEPPRF_A::CONST_1
    }
}
impl core::ops::Deref for MTEPPRF_R {
    type Target = crate::FieldReader<bool, MTEPPRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEPPRF` writer - Test Enable Control for PMU Prefetch Memory"]
pub struct MTEPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEPPRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTEPPRF_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTEPPRF_A::CONST_1)
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
#[doc = "Test Enable Control for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTUSB_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTUSB_A> for bool {
    #[inline(always)]
    fn from(variant: MTUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTUSB` reader - Test Enable Control for USB Memory"]
pub struct MTUSB_R(crate::FieldReader<bool, MTUSB_A>);
impl MTUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTUSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTUSB_A {
        match self.bits {
            false => MTUSB_A::CONST_0,
            true => MTUSB_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTUSB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTUSB_A::CONST_1
    }
}
impl core::ops::Deref for MTUSB_R {
    type Target = crate::FieldReader<bool, MTUSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTUSB` writer - Test Enable Control for USB Memory"]
pub struct MTUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MTUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTUSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTUSB_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTUSB_A::CONST_1)
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
#[doc = "Test Enable Control for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTETH0TX_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: MTETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTETH0TX` reader - Test Enable Control for ETH TX Memory"]
pub struct MTETH0TX_R(crate::FieldReader<bool, MTETH0TX_A>);
impl MTETH0TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTETH0TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTETH0TX_A {
        match self.bits {
            false => MTETH0TX_A::CONST_0,
            true => MTETH0TX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTETH0TX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTETH0TX_A::CONST_1
    }
}
impl core::ops::Deref for MTETH0TX_R {
    type Target = crate::FieldReader<bool, MTETH0TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTETH0TX` writer - Test Enable Control for ETH TX Memory"]
pub struct MTETH0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> MTETH0TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTETH0TX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTETH0TX_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTETH0TX_A::CONST_1)
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
#[doc = "Test Enable Control for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTETH0RX_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: MTETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTETH0RX` reader - Test Enable Control for ETH RX Memory"]
pub struct MTETH0RX_R(crate::FieldReader<bool, MTETH0RX_A>);
impl MTETH0RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTETH0RX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTETH0RX_A {
        match self.bits {
            false => MTETH0RX_A::CONST_0,
            true => MTETH0RX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTETH0RX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTETH0RX_A::CONST_1
    }
}
impl core::ops::Deref for MTETH0RX_R {
    type Target = crate::FieldReader<bool, MTETH0RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTETH0RX` writer - Test Enable Control for ETH RX Memory"]
pub struct MTETH0RX_W<'a> {
    w: &'a mut W,
}
impl<'a> MTETH0RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTETH0RX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTETH0RX_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTETH0RX_A::CONST_1)
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
#[doc = "Test Enable Control for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTSD0_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTSD0_A> for bool {
    #[inline(always)]
    fn from(variant: MTSD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTSD0` reader - Test Enable Control for SDMMC Memory 0"]
pub struct MTSD0_R(crate::FieldReader<bool, MTSD0_A>);
impl MTSD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTSD0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSD0_A {
        match self.bits {
            false => MTSD0_A::CONST_0,
            true => MTSD0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTSD0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTSD0_A::CONST_1
    }
}
impl core::ops::Deref for MTSD0_R {
    type Target = crate::FieldReader<bool, MTSD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTSD0` writer - Test Enable Control for SDMMC Memory 0"]
pub struct MTSD0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTSD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTSD0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTSD0_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTSD0_A::CONST_1)
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
#[doc = "Test Enable Control for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTSD1_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTSD1_A> for bool {
    #[inline(always)]
    fn from(variant: MTSD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTSD1` reader - Test Enable Control for SDMMC Memory 1"]
pub struct MTSD1_R(crate::FieldReader<bool, MTSD1_A>);
impl MTSD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTSD1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSD1_A {
        match self.bits {
            false => MTSD1_A::CONST_0,
            true => MTSD1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTSD1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTSD1_A::CONST_1
    }
}
impl core::ops::Deref for MTSD1_R {
    type Target = crate::FieldReader<bool, MTSD1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTSD1` writer - Test Enable Control for SDMMC Memory 1"]
pub struct MTSD1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTSD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTSD1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTSD1_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTSD1_A::CONST_1)
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
#[doc = "Test Enable Control for ECAT0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTECAT0_A {
    #[doc = "0: Standard operation"]
    CONST_0 = 0,
    #[doc = "1: Parity bits under test"]
    CONST_1 = 1,
}
impl From<MTECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: MTECAT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTECAT0` reader - Test Enable Control for ECAT0 Memory"]
pub struct MTECAT0_R(crate::FieldReader<bool, MTECAT0_A>);
impl MTECAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTECAT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTECAT0_A {
        match self.bits {
            false => MTECAT0_A::CONST_0,
            true => MTECAT0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == MTECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == MTECAT0_A::CONST_1
    }
}
impl core::ops::Deref for MTECAT0_R {
    type Target = crate::FieldReader<bool, MTECAT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTECAT0` writer - Test Enable Control for ECAT0 Memory"]
pub struct MTECAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTECAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTECAT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MTECAT0_A::CONST_0)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MTECAT0_A::CONST_1)
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
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    pub fn mtenps(&self) -> MTENPS_R {
        MTENPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    pub fn mtends1(&self) -> MTENDS1_R {
        MTENDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    pub fn mteu0(&self) -> MTEU0_R {
        MTEU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    pub fn mteu1(&self) -> MTEU1_R {
        MTEU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    pub fn mtemc(&self) -> MTEMC_R {
        MTEMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn mtepprf(&self) -> MTEPPRF_R {
        MTEPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    pub fn mtusb(&self) -> MTUSB_R {
        MTUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline(always)]
    pub fn mteth0tx(&self) -> MTETH0TX_R {
        MTETH0TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline(always)]
    pub fn mteth0rx(&self) -> MTETH0RX_R {
        MTETH0RX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline(always)]
    pub fn mtsd0(&self) -> MTSD0_R {
        MTSD0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline(always)]
    pub fn mtsd1(&self) -> MTSD1_R {
        MTSD1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Test Enable Control for ECAT0 Memory"]
    #[inline(always)]
    pub fn mtecat0(&self) -> MTECAT0_R {
        MTECAT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    pub fn mtenps(&mut self) -> MTENPS_W {
        MTENPS_W { w: self }
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    pub fn mtends1(&mut self) -> MTENDS1_W {
        MTENDS1_W { w: self }
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    pub fn mteu0(&mut self) -> MTEU0_W {
        MTEU0_W { w: self }
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    pub fn mteu1(&mut self) -> MTEU1_W {
        MTEU1_W { w: self }
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    pub fn mtemc(&mut self) -> MTEMC_W {
        MTEMC_W { w: self }
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn mtepprf(&mut self) -> MTEPPRF_W {
        MTEPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    pub fn mtusb(&mut self) -> MTUSB_W {
        MTUSB_W { w: self }
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline(always)]
    pub fn mteth0tx(&mut self) -> MTETH0TX_W {
        MTETH0TX_W { w: self }
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline(always)]
    pub fn mteth0rx(&mut self) -> MTETH0RX_W {
        MTETH0RX_W { w: self }
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline(always)]
    pub fn mtsd0(&mut self) -> MTSD0_W {
        MTSD0_W { w: self }
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline(always)]
    pub fn mtsd1(&mut self) -> MTSD1_W {
        MTSD1_W { w: self }
    }
    #[doc = "Bit 24 - Test Enable Control for ECAT0 Memory"]
    #[inline(always)]
    pub fn mtecat0(&mut self) -> MTECAT0_W {
        MTECAT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Memory Test Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmtsr](index.html) module"]
pub struct PMTSR_SPEC;
impl crate::RegisterSpec for PMTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmtsr::R](R) reader structure"]
impl crate::Readable for PMTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmtsr::W](W) writer structure"]
impl crate::Writable for PMTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMTSR to value 0"]
impl crate::Resettable for PMTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
