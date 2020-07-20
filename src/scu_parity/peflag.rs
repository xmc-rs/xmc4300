#[doc = "Reader of register PEFLAG"]
pub type R = crate::R<u32, super::PEFLAG>;
#[doc = "Writer for register PEFLAG"]
pub type W = crate::W<u32, super::PEFLAG>;
#[doc = "Register PEFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::PEFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Error Flag for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFPS_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEFPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEFPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEFPS`"]
pub type PEFPS_R = crate::R<bool, PEFPS_A>;
impl PEFPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFPS_A {
        match self.bits {
            false => PEFPS_A::CONST_0,
            true => PEFPS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEFPS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEFPS_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEFPS`"]
pub struct PEFPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFPS_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFPS_A::CONST_1)
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
#[doc = "Parity Error Flag for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFDS1_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEFDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEFDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEFDS1`"]
pub type PEFDS1_R = crate::R<bool, PEFDS1_A>;
impl PEFDS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFDS1_A {
        match self.bits {
            false => PEFDS1_A::CONST_0,
            true => PEFDS1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEFDS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEFDS1_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEFDS1`"]
pub struct PEFDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFDS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFDS1_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFDS1_A::CONST_1)
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
#[doc = "Parity Error Flag for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFU0_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEFU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEFU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEFU0`"]
pub type PEFU0_R = crate::R<bool, PEFU0_A>;
impl PEFU0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFU0_A {
        match self.bits {
            false => PEFU0_A::CONST_0,
            true => PEFU0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEFU0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEFU0_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEFU0`"]
pub struct PEFU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFU0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFU0_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFU0_A::CONST_1)
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
#[doc = "Parity Error Flag for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFU1_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEFU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEFU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEFU1`"]
pub type PEFU1_R = crate::R<bool, PEFU1_A>;
impl PEFU1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFU1_A {
        match self.bits {
            false => PEFU1_A::CONST_0,
            true => PEFU1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEFU1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEFU1_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEFU1`"]
pub struct PEFU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFU1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFU1_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFU1_A::CONST_1)
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
#[doc = "Parity Error Flag for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFMC_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEFMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEFMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEFMC`"]
pub type PEFMC_R = crate::R<bool, PEFMC_A>;
impl PEFMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFMC_A {
        match self.bits {
            false => PEFMC_A::CONST_0,
            true => PEFMC_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEFMC_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEFMC_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEFMC`"]
pub struct PEFMC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFMC_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFMC_A::CONST_1)
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
#[doc = "Parity Error Flag for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFPPRF_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEFPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEFPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEFPPRF`"]
pub type PEFPPRF_R = crate::R<bool, PEFPPRF_A>;
impl PEFPPRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFPPRF_A {
        match self.bits {
            false => PEFPPRF_A::CONST_0,
            true => PEFPPRF_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEFPPRF_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEFPPRF_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEFPPRF`"]
pub struct PEFPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFPPRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFPPRF_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFPPRF_A::CONST_1)
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
#[doc = "Parity Error Flag for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEUSB_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEUSB`"]
pub type PEUSB_R = crate::R<bool, PEUSB_A>;
impl PEUSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEUSB_A {
        match self.bits {
            false => PEUSB_A::CONST_0,
            true => PEUSB_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEUSB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEUSB_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEUSB`"]
pub struct PEUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PEUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEUSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEUSB_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEUSB_A::CONST_1)
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
#[doc = "Parity Error Flag for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEETH0TX_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PEETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEETH0TX`"]
pub type PEETH0TX_R = crate::R<bool, PEETH0TX_A>;
impl PEETH0TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEETH0TX_A {
        match self.bits {
            false => PEETH0TX_A::CONST_0,
            true => PEETH0TX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEETH0TX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEETH0TX_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEETH0TX`"]
pub struct PEETH0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PEETH0TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEETH0TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEETH0TX_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEETH0TX_A::CONST_1)
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
#[doc = "Parity Error Flag for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEETH0RX_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PEETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEETH0RX`"]
pub type PEETH0RX_R = crate::R<bool, PEETH0RX_A>;
impl PEETH0RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEETH0RX_A {
        match self.bits {
            false => PEETH0RX_A::CONST_0,
            true => PEETH0RX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEETH0RX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEETH0RX_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEETH0RX`"]
pub struct PEETH0RX_W<'a> {
    w: &'a mut W,
}
impl<'a> PEETH0RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEETH0RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEETH0RX_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEETH0RX_A::CONST_1)
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
#[doc = "Parity Error Flag for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD0_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PESD0_A> for bool {
    #[inline(always)]
    fn from(variant: PESD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PESD0`"]
pub type PESD0_R = crate::R<bool, PESD0_A>;
impl PESD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESD0_A {
        match self.bits {
            false => PESD0_A::CONST_0,
            true => PESD0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PESD0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PESD0_A::CONST_1
    }
}
#[doc = "Write proxy for field `PESD0`"]
pub struct PESD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PESD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PESD0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PESD0_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PESD0_A::CONST_1)
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
#[doc = "Parity Error Flag for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD1_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PESD1_A> for bool {
    #[inline(always)]
    fn from(variant: PESD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PESD1`"]
pub type PESD1_R = crate::R<bool, PESD1_A>;
impl PESD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESD1_A {
        match self.bits {
            false => PESD1_A::CONST_0,
            true => PESD1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PESD1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PESD1_A::CONST_1
    }
}
#[doc = "Write proxy for field `PESD1`"]
pub struct PESD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PESD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PESD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PESD1_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PESD1_A::CONST_1)
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
#[doc = "Parity Error Flag for ECAT0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEECAT0_A {
    #[doc = "0: No parity error detected"]
    CONST_0 = 0,
    #[doc = "1: Parity error detected"]
    CONST_1 = 1,
}
impl From<PEECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: PEECAT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEECAT0`"]
pub type PEECAT0_R = crate::R<bool, PEECAT0_A>;
impl PEECAT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEECAT0_A {
        match self.bits {
            false => PEECAT0_A::CONST_0,
            true => PEECAT0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PEECAT0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PEECAT0_A::CONST_1
    }
}
#[doc = "Write proxy for field `PEECAT0`"]
pub struct PEECAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEECAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEECAT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEECAT0_A::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEECAT0_A::CONST_1)
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
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    pub fn pefps(&self) -> PEFPS_R {
        PEFPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    pub fn pefds1(&self) -> PEFDS1_R {
        PEFDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    pub fn pefu0(&self) -> PEFU0_R {
        PEFU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    pub fn pefu1(&self) -> PEFU1_R {
        PEFU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    pub fn pefmc(&self) -> PEFMC_R {
        PEFMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn pefpprf(&self) -> PEFPPRF_R {
        PEFPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    pub fn peusb(&self) -> PEUSB_R {
        PEUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline(always)]
    pub fn peeth0tx(&self) -> PEETH0TX_R {
        PEETH0TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline(always)]
    pub fn peeth0rx(&self) -> PEETH0RX_R {
        PEETH0RX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline(always)]
    pub fn pesd0(&self) -> PESD0_R {
        PESD0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline(always)]
    pub fn pesd1(&self) -> PESD1_R {
        PESD1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Parity Error Flag for ECAT0 Memory"]
    #[inline(always)]
    pub fn peecat0(&self) -> PEECAT0_R {
        PEECAT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    pub fn pefps(&mut self) -> PEFPS_W {
        PEFPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    pub fn pefds1(&mut self) -> PEFDS1_W {
        PEFDS1_W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    pub fn pefu0(&mut self) -> PEFU0_W {
        PEFU0_W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    pub fn pefu1(&mut self) -> PEFU1_W {
        PEFU1_W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    pub fn pefmc(&mut self) -> PEFMC_W {
        PEFMC_W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn pefpprf(&mut self) -> PEFPPRF_W {
        PEFPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    pub fn peusb(&mut self) -> PEUSB_W {
        PEUSB_W { w: self }
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline(always)]
    pub fn peeth0tx(&mut self) -> PEETH0TX_W {
        PEETH0TX_W { w: self }
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline(always)]
    pub fn peeth0rx(&mut self) -> PEETH0RX_W {
        PEETH0RX_W { w: self }
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline(always)]
    pub fn pesd0(&mut self) -> PESD0_W {
        PESD0_W { w: self }
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline(always)]
    pub fn pesd1(&mut self) -> PESD1_W {
        PESD1_W { w: self }
    }
    #[doc = "Bit 24 - Parity Error Flag for ECAT0 Memory"]
    #[inline(always)]
    pub fn peecat0(&mut self) -> PEECAT0_W {
        PEECAT0_W { w: self }
    }
}
