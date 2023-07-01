#[doc = "Register `PEEN` reader"]
pub struct R(crate::R<PEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEEN` writer"]
pub struct W(crate::W<PEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEEN_SPEC>;
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
impl From<crate::W<PEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEENPS` reader - Parity Error Enable for PSRAM"]
pub type PEENPS_R = crate::BitReader<PEENPS_A>;
#[doc = "Parity Error Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENPS_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPS_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENPS_A {
        match self.bits {
            false => PEENPS_A::CONST_0,
            true => PEENPS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENPS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENPS_A::CONST_1
    }
}
#[doc = "Field `PEENPS` writer - Parity Error Enable for PSRAM"]
pub type PEENPS_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENPS_A>;
impl<'a, const O: u8> PEENPS_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENPS_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENPS_A::CONST_1)
    }
}
#[doc = "Field `PEENDS1` reader - Parity Error Enable for DSRAM1"]
pub type PEENDS1_R = crate::BitReader<PEENDS1_A>;
#[doc = "Parity Error Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENDS1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENDS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENDS1_A {
        match self.bits {
            false => PEENDS1_A::CONST_0,
            true => PEENDS1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENDS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENDS1_A::CONST_1
    }
}
#[doc = "Field `PEENDS1` writer - Parity Error Enable for DSRAM1"]
pub type PEENDS1_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENDS1_A>;
impl<'a, const O: u8> PEENDS1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENDS1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENDS1_A::CONST_1)
    }
}
#[doc = "Field `PEENU0` reader - Parity Error Enable for USIC0 Memory"]
pub type PEENU0_R = crate::BitReader<PEENU0_A>;
#[doc = "Parity Error Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENU0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU0_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENU0_A {
        match self.bits {
            false => PEENU0_A::CONST_0,
            true => PEENU0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENU0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENU0_A::CONST_1
    }
}
#[doc = "Field `PEENU0` writer - Parity Error Enable for USIC0 Memory"]
pub type PEENU0_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENU0_A>;
impl<'a, const O: u8> PEENU0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENU0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENU0_A::CONST_1)
    }
}
#[doc = "Field `PEENU1` reader - Parity Error Enable for USIC1 Memory"]
pub type PEENU1_R = crate::BitReader<PEENU1_A>;
#[doc = "Parity Error Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENU1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU1_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENU1_A {
        match self.bits {
            false => PEENU1_A::CONST_0,
            true => PEENU1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENU1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENU1_A::CONST_1
    }
}
#[doc = "Field `PEENU1` writer - Parity Error Enable for USIC1 Memory"]
pub type PEENU1_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENU1_A>;
impl<'a, const O: u8> PEENU1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENU1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENU1_A::CONST_1)
    }
}
#[doc = "Field `PEENMC` reader - Parity Error Enable for MultiCAN Memory"]
pub type PEENMC_R = crate::BitReader<PEENMC_A>;
#[doc = "Parity Error Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENMC_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEENMC_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENMC_A {
        match self.bits {
            false => PEENMC_A::CONST_0,
            true => PEENMC_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENMC_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENMC_A::CONST_1
    }
}
#[doc = "Field `PEENMC` writer - Parity Error Enable for MultiCAN Memory"]
pub type PEENMC_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENMC_A>;
impl<'a, const O: u8> PEENMC_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENMC_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENMC_A::CONST_1)
    }
}
#[doc = "Field `PEENPPRF` reader - Parity Error Enable for PMU Prefetch Memory"]
pub type PEENPPRF_R = crate::BitReader<PEENPPRF_A>;
#[doc = "Parity Error Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENPPRF_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPPRF_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENPPRF_A {
        match self.bits {
            false => PEENPPRF_A::CONST_0,
            true => PEENPPRF_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENPPRF_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENPPRF_A::CONST_1
    }
}
#[doc = "Field `PEENPPRF` writer - Parity Error Enable for PMU Prefetch Memory"]
pub type PEENPPRF_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENPPRF_A>;
impl<'a, const O: u8> PEENPPRF_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENPPRF_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENPPRF_A::CONST_1)
    }
}
#[doc = "Field `PEENUSB` reader - Parity Error Enable for USB Memory"]
pub type PEENUSB_R = crate::BitReader<PEENUSB_A>;
#[doc = "Parity Error Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENUSB_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEENUSB_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENUSB_A {
        match self.bits {
            false => PEENUSB_A::CONST_0,
            true => PEENUSB_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENUSB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENUSB_A::CONST_1
    }
}
#[doc = "Field `PEENUSB` writer - Parity Error Enable for USB Memory"]
pub type PEENUSB_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENUSB_A>;
impl<'a, const O: u8> PEENUSB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENUSB_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENUSB_A::CONST_1)
    }
}
#[doc = "Field `PEENETH0TX` reader - Parity Error Enable for ETH TX Memory"]
pub type PEENETH0TX_R = crate::BitReader<PEENETH0TX_A>;
#[doc = "Parity Error Enable for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENETH0TX_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PEENETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENETH0TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENETH0TX_A {
        match self.bits {
            false => PEENETH0TX_A::CONST_0,
            true => PEENETH0TX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENETH0TX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENETH0TX_A::CONST_1
    }
}
#[doc = "Field `PEENETH0TX` writer - Parity Error Enable for ETH TX Memory"]
pub type PEENETH0TX_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENETH0TX_A>;
impl<'a, const O: u8> PEENETH0TX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENETH0TX_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENETH0TX_A::CONST_1)
    }
}
#[doc = "Field `PEENETH0RX` reader - Parity Error Enable for ETH RX Memory"]
pub type PEENETH0RX_R = crate::BitReader<PEENETH0RX_A>;
#[doc = "Parity Error Enable for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENETH0RX_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PEENETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENETH0RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENETH0RX_A {
        match self.bits {
            false => PEENETH0RX_A::CONST_0,
            true => PEENETH0RX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENETH0RX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENETH0RX_A::CONST_1
    }
}
#[doc = "Field `PEENETH0RX` writer - Parity Error Enable for ETH RX Memory"]
pub type PEENETH0RX_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENETH0RX_A>;
impl<'a, const O: u8> PEENETH0RX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENETH0RX_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENETH0RX_A::CONST_1)
    }
}
#[doc = "Field `PEENSD0` reader - Parity Error Enable for SDMMC Memory 0"]
pub type PEENSD0_R = crate::BitReader<PEENSD0_A>;
#[doc = "Parity Error Enable for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENSD0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENSD0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENSD0_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENSD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENSD0_A {
        match self.bits {
            false => PEENSD0_A::CONST_0,
            true => PEENSD0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENSD0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENSD0_A::CONST_1
    }
}
#[doc = "Field `PEENSD0` writer - Parity Error Enable for SDMMC Memory 0"]
pub type PEENSD0_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENSD0_A>;
impl<'a, const O: u8> PEENSD0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENSD0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENSD0_A::CONST_1)
    }
}
#[doc = "Field `PEENSD1` reader - Parity Error Enable for SDMMC Memory 1"]
pub type PEENSD1_R = crate::BitReader<PEENSD1_A>;
#[doc = "Parity Error Enable for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENSD1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENSD1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENSD1_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENSD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENSD1_A {
        match self.bits {
            false => PEENSD1_A::CONST_0,
            true => PEENSD1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENSD1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENSD1_A::CONST_1
    }
}
#[doc = "Field `PEENSD1` writer - Parity Error Enable for SDMMC Memory 1"]
pub type PEENSD1_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENSD1_A>;
impl<'a, const O: u8> PEENSD1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENSD1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENSD1_A::CONST_1)
    }
}
#[doc = "Field `PEENECAT0` reader - Parity Error Enable for ECAT0 Memory"]
pub type PEENECAT0_R = crate::BitReader<PEENECAT0_A>;
#[doc = "Parity Error Enable for ECAT0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENECAT0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PEENECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENECAT0_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENECAT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENECAT0_A {
        match self.bits {
            false => PEENECAT0_A::CONST_0,
            true => PEENECAT0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEENECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEENECAT0_A::CONST_1
    }
}
#[doc = "Field `PEENECAT0` writer - Parity Error Enable for ECAT0 Memory"]
pub type PEENECAT0_W<'a, const O: u8> = crate::BitWriter<'a, PEEN_SPEC, O, PEENECAT0_A>;
impl<'a, const O: u8> PEENECAT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEENECAT0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEENECAT0_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&self) -> PEENPS_R {
        PEENPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&self) -> PEENDS1_R {
        PEENDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&self) -> PEENU0_R {
        PEENU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&self) -> PEENU1_R {
        PEENU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&self) -> PEENMC_R {
        PEENMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&self) -> PEENPPRF_R {
        PEENPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&self) -> PEENUSB_R {
        PEENUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    pub fn peeneth0tx(&self) -> PEENETH0TX_R {
        PEENETH0TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    pub fn peeneth0rx(&self) -> PEENETH0RX_R {
        PEENETH0RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    pub fn peensd0(&self) -> PEENSD0_R {
        PEENSD0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    pub fn peensd1(&self) -> PEENSD1_R {
        PEENSD1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline(always)]
    pub fn peenecat0(&self) -> PEENECAT0_R {
        PEENECAT0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn peenps(&mut self) -> PEENPS_W<0> {
        PEENPS_W::new(self)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn peends1(&mut self) -> PEENDS1_W<1> {
        PEENDS1_W::new(self)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenu0(&mut self) -> PEENU0_W<8> {
        PEENU0_W::new(self)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenu1(&mut self) -> PEENU1_W<9> {
        PEENU1_W::new(self)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenmc(&mut self) -> PEENMC_W<12> {
        PEENMC_W::new(self)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenpprf(&mut self) -> PEENPPRF_W<13> {
        PEENPPRF_W::new(self)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenusb(&mut self) -> PEENUSB_W<16> {
        PEENUSB_W::new(self)
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peeneth0tx(&mut self) -> PEENETH0TX_W<17> {
        PEENETH0TX_W::new(self)
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peeneth0rx(&mut self) -> PEENETH0RX_W<18> {
        PEENETH0RX_W::new(self)
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    #[must_use]
    pub fn peensd0(&mut self) -> PEENSD0_W<19> {
        PEENSD0_W::new(self)
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    #[must_use]
    pub fn peensd1(&mut self) -> PEENSD1_W<20> {
        PEENSD1_W::new(self)
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenecat0(&mut self) -> PEENECAT0_W<24> {
        PEENECAT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peen](index.html) module"]
pub struct PEEN_SPEC;
impl crate::RegisterSpec for PEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peen::R](R) reader structure"]
impl crate::Readable for PEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peen::W](W) writer structure"]
impl crate::Writable for PEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEEN to value 0"]
impl crate::Resettable for PEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
