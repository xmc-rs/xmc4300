#[doc = "Register `PETE` reader"]
pub struct R(crate::R<PETE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PETE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PETE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PETE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PETE` writer"]
pub struct W(crate::W<PETE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PETE_SPEC>;
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
impl From<crate::W<PETE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PETE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PETEPS` reader - Parity Error Trap Enable for PSRAM"]
pub type PETEPS_R = crate::BitReader<PETEPS_A>;
#[doc = "Parity Error Trap Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEPS_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEPS_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPS_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEPS_A {
        match self.bits {
            false => PETEPS_A::CONST_0,
            true => PETEPS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEPS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEPS_A::CONST_1
    }
}
#[doc = "Field `PETEPS` writer - Parity Error Trap Enable for PSRAM"]
pub type PETEPS_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEPS_A>;
impl<'a, const O: u8> PETEPS_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEPS_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEPS_A::CONST_1)
    }
}
#[doc = "Field `PETEDS1` reader - Parity Error Trap Enable for DSRAM1"]
pub type PETEDS1_R = crate::BitReader<PETEDS1_A>;
#[doc = "Parity Error Trap Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEDS1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEDS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEDS1_A {
        match self.bits {
            false => PETEDS1_A::CONST_0,
            true => PETEDS1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEDS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEDS1_A::CONST_1
    }
}
#[doc = "Field `PETEDS1` writer - Parity Error Trap Enable for DSRAM1"]
pub type PETEDS1_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEDS1_A>;
impl<'a, const O: u8> PETEDS1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEDS1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEDS1_A::CONST_1)
    }
}
#[doc = "Field `PETEU0` reader - Parity Error Trap Enable for USIC0 Memory"]
pub type PETEU0_R = crate::BitReader<PETEU0_A>;
#[doc = "Parity Error Trap Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEU0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEU0_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU0_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEU0_A {
        match self.bits {
            false => PETEU0_A::CONST_0,
            true => PETEU0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEU0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEU0_A::CONST_1
    }
}
#[doc = "Field `PETEU0` writer - Parity Error Trap Enable for USIC0 Memory"]
pub type PETEU0_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEU0_A>;
impl<'a, const O: u8> PETEU0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEU0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEU0_A::CONST_1)
    }
}
#[doc = "Field `PETEU1` reader - Parity Error Trap Enable for USIC1 Memory"]
pub type PETEU1_R = crate::BitReader<PETEU1_A>;
#[doc = "Parity Error Trap Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEU1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEU1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU1_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEU1_A {
        match self.bits {
            false => PETEU1_A::CONST_0,
            true => PETEU1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEU1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEU1_A::CONST_1
    }
}
#[doc = "Field `PETEU1` writer - Parity Error Trap Enable for USIC1 Memory"]
pub type PETEU1_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEU1_A>;
impl<'a, const O: u8> PETEU1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEU1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEU1_A::CONST_1)
    }
}
#[doc = "Field `PETEMC` reader - Parity Error Trap Enable for MultiCAN Memory"]
pub type PETEMC_R = crate::BitReader<PETEMC_A>;
#[doc = "Parity Error Trap Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEMC_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEMC_A> for bool {
    #[inline(always)]
    fn from(variant: PETEMC_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEMC_A {
        match self.bits {
            false => PETEMC_A::CONST_0,
            true => PETEMC_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEMC_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEMC_A::CONST_1
    }
}
#[doc = "Field `PETEMC` writer - Parity Error Trap Enable for MultiCAN Memory"]
pub type PETEMC_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEMC_A>;
impl<'a, const O: u8> PETEMC_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEMC_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEMC_A::CONST_1)
    }
}
#[doc = "Field `PETEPPRF` reader - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PETEPPRF_R = crate::BitReader<PETEPPRF_A>;
#[doc = "Parity Error Trap Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEPPRF_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPPRF_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEPPRF_A {
        match self.bits {
            false => PETEPPRF_A::CONST_0,
            true => PETEPPRF_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEPPRF_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEPPRF_A::CONST_1
    }
}
#[doc = "Field `PETEPPRF` writer - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PETEPPRF_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEPPRF_A>;
impl<'a, const O: u8> PETEPPRF_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEPPRF_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEPPRF_A::CONST_1)
    }
}
#[doc = "Field `PETEUSB` reader - Parity Error Trap Enable for USB Memory"]
pub type PETEUSB_R = crate::BitReader<PETEUSB_A>;
#[doc = "Parity Error Trap Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEUSB_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PETEUSB_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEUSB_A {
        match self.bits {
            false => PETEUSB_A::CONST_0,
            true => PETEUSB_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEUSB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEUSB_A::CONST_1
    }
}
#[doc = "Field `PETEUSB` writer - Parity Error Trap Enable for USB Memory"]
pub type PETEUSB_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEUSB_A>;
impl<'a, const O: u8> PETEUSB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEUSB_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEUSB_A::CONST_1)
    }
}
#[doc = "Field `PETEETH0TX` reader - Parity Error Trap Enable for ETH 0TX Memory"]
pub type PETEETH0TX_R = crate::BitReader<PETEETH0TX_A>;
#[doc = "Parity Error Trap Enable for ETH 0TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEETH0TX_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PETEETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEETH0TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEETH0TX_A {
        match self.bits {
            false => PETEETH0TX_A::CONST_0,
            true => PETEETH0TX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEETH0TX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEETH0TX_A::CONST_1
    }
}
#[doc = "Field `PETEETH0TX` writer - Parity Error Trap Enable for ETH 0TX Memory"]
pub type PETEETH0TX_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEETH0TX_A>;
impl<'a, const O: u8> PETEETH0TX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEETH0TX_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEETH0TX_A::CONST_1)
    }
}
#[doc = "Field `PETEETH0RX` reader - Parity Error Trap Enable for ETH0 RX Memory"]
pub type PETEETH0RX_R = crate::BitReader<PETEETH0RX_A>;
#[doc = "Parity Error Trap Enable for ETH0 RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEETH0RX_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PETEETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEETH0RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEETH0RX_A {
        match self.bits {
            false => PETEETH0RX_A::CONST_0,
            true => PETEETH0RX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEETH0RX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEETH0RX_A::CONST_1
    }
}
#[doc = "Field `PETEETH0RX` writer - Parity Error Trap Enable for ETH0 RX Memory"]
pub type PETEETH0RX_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEETH0RX_A>;
impl<'a, const O: u8> PETEETH0RX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEETH0RX_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEETH0RX_A::CONST_1)
    }
}
#[doc = "Field `PETESD0` reader - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
pub type PETESD0_R = crate::BitReader<PETESD0_A>;
#[doc = "Parity Error Trap Enable for SDMMC SRAM 0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETESD0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETESD0_A> for bool {
    #[inline(always)]
    fn from(variant: PETESD0_A) -> Self {
        variant as u8 != 0
    }
}
impl PETESD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETESD0_A {
        match self.bits {
            false => PETESD0_A::CONST_0,
            true => PETESD0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETESD0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETESD0_A::CONST_1
    }
}
#[doc = "Field `PETESD0` writer - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
pub type PETESD0_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETESD0_A>;
impl<'a, const O: u8> PETESD0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETESD0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETESD0_A::CONST_1)
    }
}
#[doc = "Field `PETESD1` reader - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
pub type PETESD1_R = crate::BitReader<PETESD1_A>;
#[doc = "Parity Error Trap Enable for SDMMC SRAM 1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETESD1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETESD1_A> for bool {
    #[inline(always)]
    fn from(variant: PETESD1_A) -> Self {
        variant as u8 != 0
    }
}
impl PETESD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETESD1_A {
        match self.bits {
            false => PETESD1_A::CONST_0,
            true => PETESD1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETESD1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETESD1_A::CONST_1
    }
}
#[doc = "Field `PETESD1` writer - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
pub type PETESD1_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETESD1_A>;
impl<'a, const O: u8> PETESD1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETESD1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETESD1_A::CONST_1)
    }
}
#[doc = "Field `PETEECAT0` reader - Parity Error Trap Enable for ECAT0 SRAM Memory"]
pub type PETEECAT0_R = crate::BitReader<PETEECAT0_A>;
#[doc = "Parity Error Trap Enable for ECAT0 SRAM Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEECAT0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PETEECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: PETEECAT0_A) -> Self {
        variant as u8 != 0
    }
}
impl PETEECAT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEECAT0_A {
        match self.bits {
            false => PETEECAT0_A::CONST_0,
            true => PETEECAT0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PETEECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PETEECAT0_A::CONST_1
    }
}
#[doc = "Field `PETEECAT0` writer - Parity Error Trap Enable for ECAT0 SRAM Memory"]
pub type PETEECAT0_W<'a, const O: u8> = crate::BitWriter<'a, PETE_SPEC, O, PETEECAT0_A>;
impl<'a, const O: u8> PETEECAT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETEECAT0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETEECAT0_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&self) -> PETEPS_R {
        PETEPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&self) -> PETEDS1_R {
        PETEDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&self) -> PETEU0_R {
        PETEU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&self) -> PETEU1_R {
        PETEU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&self) -> PETEMC_R {
        PETEMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&self) -> PETEPPRF_R {
        PETEPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&self) -> PETEUSB_R {
        PETEUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    pub fn peteeth0tx(&self) -> PETEETH0TX_R {
        PETEETH0TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    pub fn peteeth0rx(&self) -> PETEETH0RX_R {
        PETEETH0RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    pub fn petesd0(&self) -> PETESD0_R {
        PETESD0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    pub fn petesd1(&self) -> PETESD1_R {
        PETESD1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Parity Error Trap Enable for ECAT0 SRAM Memory"]
    #[inline(always)]
    pub fn peteecat0(&self) -> PETEECAT0_R {
        PETEECAT0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn peteps(&mut self) -> PETEPS_W<0> {
        PETEPS_W::new(self)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn peteds1(&mut self) -> PETEDS1_W<1> {
        PETEDS1_W::new(self)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu0(&mut self) -> PETEU0_W<8> {
        PETEU0_W::new(self)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu1(&mut self) -> PETEU1_W<9> {
        PETEU1_W::new(self)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petemc(&mut self) -> PETEMC_W<12> {
        PETEMC_W::new(self)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petepprf(&mut self) -> PETEPPRF_W<13> {
        PETEPPRF_W::new(self)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteusb(&mut self) -> PETEUSB_W<16> {
        PETEUSB_W::new(self)
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteeth0tx(&mut self) -> PETEETH0TX_W<17> {
        PETEETH0TX_W::new(self)
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteeth0rx(&mut self) -> PETEETH0RX_W<18> {
        PETEETH0RX_W::new(self)
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petesd0(&mut self) -> PETESD0_W<19> {
        PETESD0_W::new(self)
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petesd1(&mut self) -> PETESD1_W<20> {
        PETESD1_W::new(self)
    }
    #[doc = "Bit 24 - Parity Error Trap Enable for ECAT0 SRAM Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteecat0(&mut self) -> PETEECAT0_W<24> {
        PETEECAT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Trap Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pete](index.html) module"]
pub struct PETE_SPEC;
impl crate::RegisterSpec for PETE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pete::R](R) reader structure"]
impl crate::Readable for PETE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pete::W](W) writer structure"]
impl crate::Writable for PETE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PETE to value 0"]
impl crate::Resettable for PETE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
