#[doc = "Reader of register PEEN"]
pub type R = crate::R<u32, super::PEEN>;
#[doc = "Writer for register PEEN"]
pub type W = crate::W<u32, super::PEEN>;
#[doc = "Register PEEN `reset()`'s with value 0"]
impl crate::ResetValue for super::PEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Error Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENPS_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPS_A) -> Self {
        match variant {
            PEENPS_A::CONST_0 => false,
            PEENPS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENPS`"]
pub type PEENPS_R = crate::R<bool, PEENPS_A>;
impl PEENPS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENPS`"]
pub struct PEENPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENDS1_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENDS1_A) -> Self {
        match variant {
            PEENDS1_A::CONST_0 => false,
            PEENDS1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENDS1`"]
pub type PEENDS1_R = crate::R<bool, PEENDS1_A>;
impl PEENDS1_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENDS1`"]
pub struct PEENDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENDS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENU0_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU0_A) -> Self {
        match variant {
            PEENU0_A::CONST_0 => false,
            PEENU0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENU0`"]
pub type PEENU0_R = crate::R<bool, PEENU0_A>;
impl PEENU0_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENU0`"]
pub struct PEENU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENU0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENU1_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU1_A) -> Self {
        match variant {
            PEENU1_A::CONST_0 => false,
            PEENU1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENU1`"]
pub type PEENU1_R = crate::R<bool, PEENU1_A>;
impl PEENU1_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENU1`"]
pub struct PEENU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENU1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENMC_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEENMC_A) -> Self {
        match variant {
            PEENMC_A::CONST_0 => false,
            PEENMC_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENMC`"]
pub type PEENMC_R = crate::R<bool, PEENMC_A>;
impl PEENMC_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENMC`"]
pub struct PEENMC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENPPRF_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPPRF_A) -> Self {
        match variant {
            PEENPPRF_A::CONST_0 => false,
            PEENPPRF_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENPPRF`"]
pub type PEENPPRF_R = crate::R<bool, PEENPPRF_A>;
impl PEENPPRF_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENPPRF`"]
pub struct PEENPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENPPRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENUSB_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEENUSB_A) -> Self {
        match variant {
            PEENUSB_A::CONST_0 => false,
            PEENUSB_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENUSB`"]
pub type PEENUSB_R = crate::R<bool, PEENUSB_A>;
impl PEENUSB_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENUSB`"]
pub struct PEENUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENUSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENETH0TX_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PEENETH0TX_A) -> Self {
        match variant {
            PEENETH0TX_A::CONST_0 => false,
            PEENETH0TX_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENETH0TX`"]
pub type PEENETH0TX_R = crate::R<bool, PEENETH0TX_A>;
impl PEENETH0TX_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENETH0TX`"]
pub struct PEENETH0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENETH0TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENETH0TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENETH0RX_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PEENETH0RX_A) -> Self {
        match variant {
            PEENETH0RX_A::CONST_0 => false,
            PEENETH0RX_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENETH0RX`"]
pub type PEENETH0RX_R = crate::R<bool, PEENETH0RX_A>;
impl PEENETH0RX_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENETH0RX`"]
pub struct PEENETH0RX_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENETH0RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENETH0RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENSD0_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENSD0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENSD0_A) -> Self {
        match variant {
            PEENSD0_A::CONST_0 => false,
            PEENSD0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENSD0`"]
pub type PEENSD0_R = crate::R<bool, PEENSD0_A>;
impl PEENSD0_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENSD0`"]
pub struct PEENSD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENSD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENSD0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENSD1_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENSD1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENSD1_A) -> Self {
        match variant {
            PEENSD1_A::CONST_0 => false,
            PEENSD1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENSD1`"]
pub type PEENSD1_R = crate::R<bool, PEENSD1_A>;
impl PEENSD1_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENSD1`"]
pub struct PEENSD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENSD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENSD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Parity Error Enable for ECAT0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENECAT0_A {
    #[doc = "0: Disabled"]
    CONST_0,
    #[doc = "1: Enabled"]
    CONST_1,
}
impl From<PEENECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENECAT0_A) -> Self {
        match variant {
            PEENECAT0_A::CONST_0 => false,
            PEENECAT0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PEENECAT0`"]
pub type PEENECAT0_R = crate::R<bool, PEENECAT0_A>;
impl PEENECAT0_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PEENECAT0`"]
pub struct PEENECAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENECAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENECAT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&self) -> PEENPS_R {
        PEENPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&self) -> PEENDS1_R {
        PEENDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&self) -> PEENU0_R {
        PEENU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&self) -> PEENU1_R {
        PEENU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&self) -> PEENMC_R {
        PEENMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&self) -> PEENPPRF_R {
        PEENPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&self) -> PEENUSB_R {
        PEENUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    pub fn peeneth0tx(&self) -> PEENETH0TX_R {
        PEENETH0TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    pub fn peeneth0rx(&self) -> PEENETH0RX_R {
        PEENETH0RX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    pub fn peensd0(&self) -> PEENSD0_R {
        PEENSD0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    pub fn peensd1(&self) -> PEENSD1_R {
        PEENSD1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline(always)]
    pub fn peenecat0(&self) -> PEENECAT0_R {
        PEENECAT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&mut self) -> PEENPS_W {
        PEENPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&mut self) -> PEENDS1_W {
        PEENDS1_W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&mut self) -> PEENU0_W {
        PEENU0_W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&mut self) -> PEENU1_W {
        PEENU1_W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&mut self) -> PEENMC_W {
        PEENMC_W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&mut self) -> PEENPPRF_W {
        PEENPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&mut self) -> PEENUSB_W {
        PEENUSB_W { w: self }
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    pub fn peeneth0tx(&mut self) -> PEENETH0TX_W {
        PEENETH0TX_W { w: self }
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    pub fn peeneth0rx(&mut self) -> PEENETH0RX_W {
        PEENETH0RX_W { w: self }
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    pub fn peensd0(&mut self) -> PEENSD0_W {
        PEENSD0_W { w: self }
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    pub fn peensd1(&mut self) -> PEENSD1_W {
        PEENSD1_W { w: self }
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline(always)]
    pub fn peenecat0(&mut self) -> PEENECAT0_W {
        PEENECAT0_W { w: self }
    }
}
