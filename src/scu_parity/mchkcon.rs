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
#[doc = "Field `SELPS` reader - Select Memory Check for PSRAM"]
pub type SELPS_R = crate::BitReader<SELPS_A>;
#[doc = "Select Memory Check for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELPS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELPS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELPS_A::CONST_1
    }
}
#[doc = "Field `SELPS` writer - Select Memory Check for PSRAM"]
pub type SELPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELPS_A, O>;
impl<'a, const O: u8> SELPS_W<'a, O> {
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
}
#[doc = "Field `SELDS1` reader - Select Memory Check for DSRAM1"]
pub type SELDS1_R = crate::BitReader<SELDS1_A>;
#[doc = "Select Memory Check for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELDS1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELDS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELDS1_A::CONST_1
    }
}
#[doc = "Field `SELDS1` writer - Select Memory Check for DSRAM1"]
pub type SELDS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELDS1_A, O>;
impl<'a, const O: u8> SELDS1_W<'a, O> {
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
}
#[doc = "Field `USIC0DRA` reader - Select Memory Check for USIC0"]
pub type USIC0DRA_R = crate::BitReader<USIC0DRA_A>;
#[doc = "Select Memory Check for USIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl USIC0DRA_R {
    #[doc = "Get enumerated values variant"]
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
        *self == USIC0DRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USIC0DRA_A::CONST_1
    }
}
#[doc = "Field `USIC0DRA` writer - Select Memory Check for USIC0"]
pub type USIC0DRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, USIC0DRA_A, O>;
impl<'a, const O: u8> USIC0DRA_W<'a, O> {
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
}
#[doc = "Field `USIC1DRA` reader - Select Memory Check for USIC1"]
pub type USIC1DRA_R = crate::BitReader<USIC1DRA_A>;
#[doc = "Select Memory Check for USIC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl USIC1DRA_R {
    #[doc = "Get enumerated values variant"]
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
        *self == USIC1DRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USIC1DRA_A::CONST_1
    }
}
#[doc = "Field `USIC1DRA` writer - Select Memory Check for USIC1"]
pub type USIC1DRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, USIC1DRA_A, O>;
impl<'a, const O: u8> USIC1DRA_W<'a, O> {
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
}
#[doc = "Field `MCANDRA` reader - Select Memory Check for MultiCAN"]
pub type MCANDRA_R = crate::BitReader<MCANDRA_A>;
#[doc = "Select Memory Check for MultiCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MCANDRA_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MCANDRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MCANDRA_A::CONST_1
    }
}
#[doc = "Field `MCANDRA` writer - Select Memory Check for MultiCAN"]
pub type MCANDRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, MCANDRA_A, O>;
impl<'a, const O: u8> MCANDRA_W<'a, O> {
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
}
#[doc = "Field `PPRFDRA` reader - Select Memory Check for PMU"]
pub type PPRFDRA_R = crate::BitReader<PPRFDRA_A>;
#[doc = "Select Memory Check for PMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PPRFDRA_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PPRFDRA_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPRFDRA_A::CONST_1
    }
}
#[doc = "Field `PPRFDRA` writer - Select Memory Check for PMU"]
pub type PPRFDRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, PPRFDRA_A, O>;
impl<'a, const O: u8> PPRFDRA_W<'a, O> {
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
}
#[doc = "Field `SELUSB` reader - Select Memory Check for USB SRAM"]
pub type SELUSB_R = crate::BitReader<SELUSB_A>;
#[doc = "Select Memory Check for USB SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELUSB_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELUSB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELUSB_A::CONST_1
    }
}
#[doc = "Field `SELUSB` writer - Select Memory Check for USB SRAM"]
pub type SELUSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELUSB_A, O>;
impl<'a, const O: u8> SELUSB_W<'a, O> {
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
}
#[doc = "Field `SELETH0TX` reader - Select Memory Check for ETH0 TX SRAM"]
pub type SELETH0TX_R = crate::BitReader<SELETH0TX_A>;
#[doc = "Select Memory Check for ETH0 TX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELETH0TX_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELETH0TX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELETH0TX_A::CONST_1
    }
}
#[doc = "Field `SELETH0TX` writer - Select Memory Check for ETH0 TX SRAM"]
pub type SELETH0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELETH0TX_A, O>;
impl<'a, const O: u8> SELETH0TX_W<'a, O> {
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
}
#[doc = "Field `SELETH0RX` reader - Select Memory Check for ETH0 RX SRAM"]
pub type SELETH0RX_R = crate::BitReader<SELETH0RX_A>;
#[doc = "Select Memory Check for ETH0 RX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELETH0RX_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELETH0RX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELETH0RX_A::CONST_1
    }
}
#[doc = "Field `SELETH0RX` writer - Select Memory Check for ETH0 RX SRAM"]
pub type SELETH0RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELETH0RX_A, O>;
impl<'a, const O: u8> SELETH0RX_W<'a, O> {
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
}
#[doc = "Field `SELSD0` reader - Select Memory Check for SDMMC SRAM 0"]
pub type SELSD0_R = crate::BitReader<SELSD0_A>;
#[doc = "Select Memory Check for SDMMC SRAM 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELSD0_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELSD0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELSD0_A::CONST_1
    }
}
#[doc = "Field `SELSD0` writer - Select Memory Check for SDMMC SRAM 0"]
pub type SELSD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELSD0_A, O>;
impl<'a, const O: u8> SELSD0_W<'a, O> {
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
}
#[doc = "Field `SELSD1` reader - Select Memory Check for SDMMC SRAM 1"]
pub type SELSD1_R = crate::BitReader<SELSD1_A>;
#[doc = "Select Memory Check for SDMMC SRAM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELSD1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELSD1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELSD1_A::CONST_1
    }
}
#[doc = "Field `SELSD1` writer - Select Memory Check for SDMMC SRAM 1"]
pub type SELSD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELSD1_A, O>;
impl<'a, const O: u8> SELSD1_W<'a, O> {
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
}
#[doc = "Field `SELECAT0` reader - Select Memory Check for ECAT0 SRAM 1"]
pub type SELECAT0_R = crate::BitReader<SELECAT0_A>;
#[doc = "Select Memory Check for ECAT0 SRAM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELECAT0_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SELECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SELECAT0_A::CONST_1
    }
}
#[doc = "Field `SELECAT0` writer - Select Memory Check for ECAT0 SRAM 1"]
pub type SELECAT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCHKCON_SPEC, SELECAT0_A, O>;
impl<'a, const O: u8> SELECAT0_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&self) -> SELPS_R {
        SELPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&self) -> SELDS1_R {
        SELDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&self) -> USIC0DRA_R {
        USIC0DRA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&self) -> USIC1DRA_R {
        USIC1DRA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&self) -> MCANDRA_R {
        MCANDRA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&self) -> PPRFDRA_R {
        PPRFDRA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&self) -> SELUSB_R {
        SELUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    pub fn seleth0tx(&self) -> SELETH0TX_R {
        SELETH0TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    pub fn seleth0rx(&self) -> SELETH0RX_R {
        SELETH0RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    pub fn selsd0(&self) -> SELSD0_R {
        SELSD0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    pub fn selsd1(&self) -> SELSD1_R {
        SELSD1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline(always)]
    pub fn selecat0(&self) -> SELECAT0_R {
        SELECAT0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selps(&mut self) -> SELPS_W<0> {
        SELPS_W::new(self)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn selds1(&mut self) -> SELDS1_W<1> {
        SELDS1_W::new(self)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    #[must_use]
    pub fn usic0dra(&mut self) -> USIC0DRA_W<8> {
        USIC0DRA_W::new(self)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    #[must_use]
    pub fn usic1dra(&mut self) -> USIC1DRA_W<9> {
        USIC1DRA_W::new(self)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    #[must_use]
    pub fn mcandra(&mut self) -> MCANDRA_W<12> {
        MCANDRA_W::new(self)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    #[must_use]
    pub fn pprfdra(&mut self) -> PPRFDRA_W<13> {
        PPRFDRA_W::new(self)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selusb(&mut self) -> SELUSB_W<16> {
        SELUSB_W::new(self)
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn seleth0tx(&mut self) -> SELETH0TX_W<17> {
        SELETH0TX_W::new(self)
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn seleth0rx(&mut self) -> SELETH0RX_W<18> {
        SELETH0RX_W::new(self)
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    #[must_use]
    pub fn selsd0(&mut self) -> SELSD0_W<19> {
        SELSD0_W::new(self)
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    #[must_use]
    pub fn selsd1(&mut self) -> SELSD1_W<20> {
        SELSD1_W::new(self)
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline(always)]
    #[must_use]
    pub fn selecat0(&mut self) -> SELECAT0_W<24> {
        SELECAT0_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCHKCON to value 0"]
impl crate::Resettable for MCHKCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
