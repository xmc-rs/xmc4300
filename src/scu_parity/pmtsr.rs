#[doc = "Reader of register PMTSR"]
pub type R = crate::R<u32, super::PMTSR>;
#[doc = "Writer for register PMTSR"]
pub type W = crate::W<u32, super::PMTSR>;
#[doc = "Register PMTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Test Enable Control for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENPS_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTENPS_A> for bool {
    #[inline(always)]
    fn from(variant: MTENPS_A) -> Self {
        match variant {
            MTENPS_A::CONST_0 => false,
            MTENPS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTENPS`"]
pub type MTENPS_R = crate::R<bool, MTENPS_A>;
impl MTENPS_R {
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
        *self == MTENPS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTENPS_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTENPS`"]
pub struct MTENPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTENPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Test Enable Control for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENDS1_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: MTENDS1_A) -> Self {
        match variant {
            MTENDS1_A::CONST_0 => false,
            MTENDS1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTENDS1`"]
pub type MTENDS1_R = crate::R<bool, MTENDS1_A>;
impl MTENDS1_R {
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
        *self == MTENDS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTENDS1_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTENDS1`"]
pub struct MTENDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTENDS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Test Enable Control for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU0_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTEU0_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU0_A) -> Self {
        match variant {
            MTEU0_A::CONST_0 => false,
            MTEU0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTEU0`"]
pub type MTEU0_R = crate::R<bool, MTEU0_A>;
impl MTEU0_R {
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
        *self == MTEU0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTEU0_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTEU0`"]
pub struct MTEU0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEU0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Test Enable Control for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU1_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTEU1_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU1_A) -> Self {
        match variant {
            MTEU1_A::CONST_0 => false,
            MTEU1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTEU1`"]
pub type MTEU1_R = crate::R<bool, MTEU1_A>;
impl MTEU1_R {
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
        *self == MTEU1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTEU1_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTEU1`"]
pub struct MTEU1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEU1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Test Enable Control for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEMC_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTEMC_A> for bool {
    #[inline(always)]
    fn from(variant: MTEMC_A) -> Self {
        match variant {
            MTEMC_A::CONST_0 => false,
            MTEMC_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTEMC`"]
pub type MTEMC_R = crate::R<bool, MTEMC_A>;
impl MTEMC_R {
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
        *self == MTEMC_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTEMC_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTEMC`"]
pub struct MTEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Test Enable Control for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEPPRF_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: MTEPPRF_A) -> Self {
        match variant {
            MTEPPRF_A::CONST_0 => false,
            MTEPPRF_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTEPPRF`"]
pub type MTEPPRF_R = crate::R<bool, MTEPPRF_A>;
impl MTEPPRF_R {
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
        *self == MTEPPRF_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTEPPRF_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTEPPRF`"]
pub struct MTEPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEPPRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Test Enable Control for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTUSB_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTUSB_A> for bool {
    #[inline(always)]
    fn from(variant: MTUSB_A) -> Self {
        match variant {
            MTUSB_A::CONST_0 => false,
            MTUSB_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTUSB`"]
pub type MTUSB_R = crate::R<bool, MTUSB_A>;
impl MTUSB_R {
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
        *self == MTUSB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTUSB_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTUSB`"]
pub struct MTUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MTUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTUSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Test Enable Control for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTETH0TX_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: MTETH0TX_A) -> Self {
        match variant {
            MTETH0TX_A::CONST_0 => false,
            MTETH0TX_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTETH0TX`"]
pub type MTETH0TX_R = crate::R<bool, MTETH0TX_A>;
impl MTETH0TX_R {
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
        *self == MTETH0TX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTETH0TX_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTETH0TX`"]
pub struct MTETH0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> MTETH0TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTETH0TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Test Enable Control for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTETH0RX_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: MTETH0RX_A) -> Self {
        match variant {
            MTETH0RX_A::CONST_0 => false,
            MTETH0RX_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTETH0RX`"]
pub type MTETH0RX_R = crate::R<bool, MTETH0RX_A>;
impl MTETH0RX_R {
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
        *self == MTETH0RX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTETH0RX_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTETH0RX`"]
pub struct MTETH0RX_W<'a> {
    w: &'a mut W,
}
impl<'a> MTETH0RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTETH0RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Test Enable Control for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTSD0_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTSD0_A> for bool {
    #[inline(always)]
    fn from(variant: MTSD0_A) -> Self {
        match variant {
            MTSD0_A::CONST_0 => false,
            MTSD0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTSD0`"]
pub type MTSD0_R = crate::R<bool, MTSD0_A>;
impl MTSD0_R {
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
        *self == MTSD0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTSD0_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTSD0`"]
pub struct MTSD0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTSD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTSD0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Test Enable Control for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTSD1_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTSD1_A> for bool {
    #[inline(always)]
    fn from(variant: MTSD1_A) -> Self {
        match variant {
            MTSD1_A::CONST_0 => false,
            MTSD1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTSD1`"]
pub type MTSD1_R = crate::R<bool, MTSD1_A>;
impl MTSD1_R {
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
        *self == MTSD1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTSD1_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTSD1`"]
pub struct MTSD1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTSD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTSD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Test Enable Control for ECAT0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTECAT0_A {
    #[doc = "0: Standard operation"]
    CONST_0,
    #[doc = "1: Parity bits under test"]
    CONST_1,
}
impl From<MTECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: MTECAT0_A) -> Self {
        match variant {
            MTECAT0_A::CONST_0 => false,
            MTECAT0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MTECAT0`"]
pub type MTECAT0_R = crate::R<bool, MTECAT0_A>;
impl MTECAT0_R {
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
        *self == MTECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MTECAT0_A::CONST_1
    }
}
#[doc = "Write proxy for field `MTECAT0`"]
pub struct MTECAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTECAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTECAT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
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
}
