#[doc = "Reader of register AL_EVENT_MASK"]
pub type R = crate::R<u32, super::AL_EVENT_MASK>;
#[doc = "Writer for register AL_EVENT_MASK"]
pub type W = crate::W<u32, super::AL_EVENT_MASK>;
#[doc = "Register AL_EVENT_MASK `reset()`'s with value 0x00ff_ff2f"]
impl crate::ResetValue for super::AL_EVENT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ff2f
    }
}
#[doc = "AL Control event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_CE_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<AL_CE_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: AL_CE_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AL_CE_MASK`"]
pub type AL_CE_MASK_R = crate::R<bool, AL_CE_MASK_A>;
impl AL_CE_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AL_CE_MASK_A {
        match self.bits {
            false => AL_CE_MASK_A::VALUE1,
            true => AL_CE_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AL_CE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AL_CE_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `AL_CE_MASK`"]
pub struct AL_CE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AL_CE_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AL_CE_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AL_CE_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AL_CE_MASK_A::VALUE2)
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
#[doc = "DC Latch event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LE_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<DC_LE_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: DC_LE_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DC_LE_MASK`"]
pub type DC_LE_MASK_R = crate::R<bool, DC_LE_MASK_A>;
impl DC_LE_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_LE_MASK_A {
        match self.bits {
            false => DC_LE_MASK_A::VALUE1,
            true => DC_LE_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `DC_LE_MASK`"]
pub struct DC_LE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_LE_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DC_LE_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DC_LE_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DC_LE_MASK_A::VALUE2)
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
#[doc = "State of DC SYNC0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_S0_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<ST_S0_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: ST_S0_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ST_S0_MASK`"]
pub type ST_S0_MASK_R = crate::R<bool, ST_S0_MASK_A>;
impl ST_S0_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_S0_MASK_A {
        match self.bits {
            false => ST_S0_MASK_A::VALUE1,
            true => ST_S0_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ST_S0_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ST_S0_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `ST_S0_MASK`"]
pub struct ST_S0_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_S0_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ST_S0_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ST_S0_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ST_S0_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "State of DC SYNC1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_S1_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<ST_S1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: ST_S1_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ST_S1_MASK`"]
pub type ST_S1_MASK_R = crate::R<bool, ST_S1_MASK_A>;
impl ST_S1_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_S1_MASK_A {
        match self.bits {
            false => ST_S1_MASK_A::VALUE1,
            true => ST_S1_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ST_S1_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ST_S1_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `ST_S1_MASK`"]
pub struct ST_S1_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_S1_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ST_S1_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ST_S1_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ST_S1_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "SyncManager activation register changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_A_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SM_A_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SM_A_MASK`"]
pub type SM_A_MASK_R = crate::R<bool, SM_A_MASK_A>;
impl SM_A_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A_MASK_A {
        match self.bits {
            false => SM_A_MASK_A::VALUE1,
            true => SM_A_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SM_A_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SM_A_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SM_A_MASK`"]
pub struct SM_A_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_A_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM_A_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SM_A_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SM_A_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "EEPROM Emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEP_E_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<EEP_E_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: EEP_E_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEP_E_MASK`"]
pub type EEP_E_MASK_R = crate::R<bool, EEP_E_MASK_A>;
impl EEP_E_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEP_E_MASK_A {
        match self.bits {
            false => EEP_E_MASK_A::VALUE1,
            true => EEP_E_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EEP_E_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EEP_E_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `EEP_E_MASK`"]
pub struct EEP_E_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EEP_E_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEP_E_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EEP_E_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EEP_E_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Watchdog Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_D_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<WP_D_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: WP_D_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WP_D_MASK`"]
pub type WP_D_MASK_R = crate::R<bool, WP_D_MASK_A>;
impl WP_D_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_D_MASK_A {
        match self.bits {
            false => WP_D_MASK_A::VALUE1,
            true => WP_D_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WP_D_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WP_D_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `WP_D_MASK`"]
pub struct WP_D_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_D_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP_D_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WP_D_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WP_D_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_0_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_0_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_0_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_0_MASK`"]
pub type SMI_0_MASK_R = crate::R<bool, SMI_0_MASK_A>;
impl SMI_0_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_0_MASK_A {
        match self.bits {
            false => SMI_0_MASK_A::VALUE1,
            true => SMI_0_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_0_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_0_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_0_MASK`"]
pub struct SMI_0_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_0_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_0_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_0_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_0_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_1_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_1_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_1_MASK`"]
pub type SMI_1_MASK_R = crate::R<bool, SMI_1_MASK_A>;
impl SMI_1_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_1_MASK_A {
        match self.bits {
            false => SMI_1_MASK_A::VALUE1,
            true => SMI_1_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_1_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_1_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_1_MASK`"]
pub struct SMI_1_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_1_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_1_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_1_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_1_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_2_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_2_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_2_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_2_MASK`"]
pub type SMI_2_MASK_R = crate::R<bool, SMI_2_MASK_A>;
impl SMI_2_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_2_MASK_A {
        match self.bits {
            false => SMI_2_MASK_A::VALUE1,
            true => SMI_2_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_2_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_2_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_2_MASK`"]
pub struct SMI_2_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_2_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_2_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_2_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_2_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_3_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_3_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_3_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_3_MASK`"]
pub type SMI_3_MASK_R = crate::R<bool, SMI_3_MASK_A>;
impl SMI_3_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_3_MASK_A {
        match self.bits {
            false => SMI_3_MASK_A::VALUE1,
            true => SMI_3_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_3_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_3_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_3_MASK`"]
pub struct SMI_3_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_3_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_3_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_3_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_3_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_4_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_4_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_4_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_4_MASK`"]
pub type SMI_4_MASK_R = crate::R<bool, SMI_4_MASK_A>;
impl SMI_4_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_4_MASK_A {
        match self.bits {
            false => SMI_4_MASK_A::VALUE1,
            true => SMI_4_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_4_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_4_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_4_MASK`"]
pub struct SMI_4_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_4_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_4_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_4_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_4_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_5_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_5_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_5_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_5_MASK`"]
pub type SMI_5_MASK_R = crate::R<bool, SMI_5_MASK_A>;
impl SMI_5_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_5_MASK_A {
        match self.bits {
            false => SMI_5_MASK_A::VALUE1,
            true => SMI_5_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_5_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_5_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_5_MASK`"]
pub struct SMI_5_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_5_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_5_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_5_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_5_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_6_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_6_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_6_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_6_MASK`"]
pub type SMI_6_MASK_R = crate::R<bool, SMI_6_MASK_A>;
impl SMI_6_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_6_MASK_A {
        match self.bits {
            false => SMI_6_MASK_A::VALUE1,
            true => SMI_6_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_6_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_6_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_6_MASK`"]
pub struct SMI_6_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_6_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_6_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_6_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_6_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_7_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_7_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_7_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_7_MASK`"]
pub type SMI_7_MASK_R = crate::R<bool, SMI_7_MASK_A>;
impl SMI_7_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_7_MASK_A {
        match self.bits {
            false => SMI_7_MASK_A::VALUE1,
            true => SMI_7_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_7_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_7_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_7_MASK`"]
pub struct SMI_7_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_7_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_7_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_7_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_7_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_8_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_8_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_8_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_8_MASK`"]
pub type SMI_8_MASK_R = crate::R<bool, SMI_8_MASK_A>;
impl SMI_8_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_8_MASK_A {
        match self.bits {
            false => SMI_8_MASK_A::VALUE1,
            true => SMI_8_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_8_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_8_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_8_MASK`"]
pub struct SMI_8_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_8_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_8_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_8_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_8_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_9_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_9_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_9_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_9_MASK`"]
pub type SMI_9_MASK_R = crate::R<bool, SMI_9_MASK_A>;
impl SMI_9_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_9_MASK_A {
        match self.bits {
            false => SMI_9_MASK_A::VALUE1,
            true => SMI_9_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_9_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_9_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_9_MASK`"]
pub struct SMI_9_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_9_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_9_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_9_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_9_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_10_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_10_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_10_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_10_MASK`"]
pub type SMI_10_MASK_R = crate::R<bool, SMI_10_MASK_A>;
impl SMI_10_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_10_MASK_A {
        match self.bits {
            false => SMI_10_MASK_A::VALUE1,
            true => SMI_10_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_10_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_10_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_10_MASK`"]
pub struct SMI_10_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_10_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_10_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_10_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_10_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_11_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_11_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_11_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_11_MASK`"]
pub type SMI_11_MASK_R = crate::R<bool, SMI_11_MASK_A>;
impl SMI_11_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_11_MASK_A {
        match self.bits {
            false => SMI_11_MASK_A::VALUE1,
            true => SMI_11_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_11_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_11_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_11_MASK`"]
pub struct SMI_11_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_11_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_11_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_11_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_11_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_12_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_12_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_12_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_12_MASK`"]
pub type SMI_12_MASK_R = crate::R<bool, SMI_12_MASK_A>;
impl SMI_12_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_12_MASK_A {
        match self.bits {
            false => SMI_12_MASK_A::VALUE1,
            true => SMI_12_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_12_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_12_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_12_MASK`"]
pub struct SMI_12_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_12_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_12_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_12_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_12_MASK_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_13_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_13_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_13_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_13_MASK`"]
pub type SMI_13_MASK_R = crate::R<bool, SMI_13_MASK_A>;
impl SMI_13_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_13_MASK_A {
        match self.bits {
            false => SMI_13_MASK_A::VALUE1,
            true => SMI_13_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_13_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_13_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_13_MASK`"]
pub struct SMI_13_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_13_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_13_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_13_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_13_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_14_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_14_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_14_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_14_MASK`"]
pub type SMI_14_MASK_R = crate::R<bool, SMI_14_MASK_A>;
impl SMI_14_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_14_MASK_A {
        match self.bits {
            false => SMI_14_MASK_A::VALUE1,
            true => SMI_14_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_14_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_14_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_14_MASK`"]
pub struct SMI_14_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_14_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_14_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_14_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_14_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_15_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_15_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_15_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMI_15_MASK`"]
pub type SMI_15_MASK_R = crate::R<bool, SMI_15_MASK_A>;
impl SMI_15_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_15_MASK_A {
        match self.bits {
            false => SMI_15_MASK_A::VALUE1,
            true => SMI_15_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_15_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_15_MASK_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_15_MASK`"]
pub struct SMI_15_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_15_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_15_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_15_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_15_MASK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    pub fn al_ce_mask(&self) -> AL_CE_MASK_R {
        AL_CE_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le_mask(&self) -> DC_LE_MASK_R {
        DC_LE_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    pub fn st_s0_mask(&self) -> ST_S0_MASK_R {
        ST_S0_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    pub fn st_s1_mask(&self) -> ST_S1_MASK_R {
        ST_S1_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    pub fn sm_a_mask(&self) -> SM_A_MASK_R {
        SM_A_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    pub fn eep_e_mask(&self) -> EEP_E_MASK_R {
        EEP_E_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    pub fn wp_d_mask(&self) -> WP_D_MASK_R {
        WP_D_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_0_mask(&self) -> SMI_0_MASK_R {
        SMI_0_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_1_mask(&self) -> SMI_1_MASK_R {
        SMI_1_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2_mask(&self) -> SMI_2_MASK_R {
        SMI_2_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3_mask(&self) -> SMI_3_MASK_R {
        SMI_3_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4_mask(&self) -> SMI_4_MASK_R {
        SMI_4_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5_mask(&self) -> SMI_5_MASK_R {
        SMI_5_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6_mask(&self) -> SMI_6_MASK_R {
        SMI_6_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7_mask(&self) -> SMI_7_MASK_R {
        SMI_7_MASK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8_mask(&self) -> SMI_8_MASK_R {
        SMI_8_MASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9_mask(&self) -> SMI_9_MASK_R {
        SMI_9_MASK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10_mask(&self) -> SMI_10_MASK_R {
        SMI_10_MASK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11_mask(&self) -> SMI_11_MASK_R {
        SMI_11_MASK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12_mask(&self) -> SMI_12_MASK_R {
        SMI_12_MASK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13_mask(&self) -> SMI_13_MASK_R {
        SMI_13_MASK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14_mask(&self) -> SMI_14_MASK_R {
        SMI_14_MASK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_15_mask(&self) -> SMI_15_MASK_R {
        SMI_15_MASK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    pub fn al_ce_mask(&mut self) -> AL_CE_MASK_W {
        AL_CE_MASK_W { w: self }
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le_mask(&mut self) -> DC_LE_MASK_W {
        DC_LE_MASK_W { w: self }
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    pub fn st_s0_mask(&mut self) -> ST_S0_MASK_W {
        ST_S0_MASK_W { w: self }
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    pub fn st_s1_mask(&mut self) -> ST_S1_MASK_W {
        ST_S1_MASK_W { w: self }
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    pub fn sm_a_mask(&mut self) -> SM_A_MASK_W {
        SM_A_MASK_W { w: self }
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    pub fn eep_e_mask(&mut self) -> EEP_E_MASK_W {
        EEP_E_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    pub fn wp_d_mask(&mut self) -> WP_D_MASK_W {
        WP_D_MASK_W { w: self }
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_0_mask(&mut self) -> SMI_0_MASK_W {
        SMI_0_MASK_W { w: self }
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_1_mask(&mut self) -> SMI_1_MASK_W {
        SMI_1_MASK_W { w: self }
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2_mask(&mut self) -> SMI_2_MASK_W {
        SMI_2_MASK_W { w: self }
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3_mask(&mut self) -> SMI_3_MASK_W {
        SMI_3_MASK_W { w: self }
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4_mask(&mut self) -> SMI_4_MASK_W {
        SMI_4_MASK_W { w: self }
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5_mask(&mut self) -> SMI_5_MASK_W {
        SMI_5_MASK_W { w: self }
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6_mask(&mut self) -> SMI_6_MASK_W {
        SMI_6_MASK_W { w: self }
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7_mask(&mut self) -> SMI_7_MASK_W {
        SMI_7_MASK_W { w: self }
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8_mask(&mut self) -> SMI_8_MASK_W {
        SMI_8_MASK_W { w: self }
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9_mask(&mut self) -> SMI_9_MASK_W {
        SMI_9_MASK_W { w: self }
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10_mask(&mut self) -> SMI_10_MASK_W {
        SMI_10_MASK_W { w: self }
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11_mask(&mut self) -> SMI_11_MASK_W {
        SMI_11_MASK_W { w: self }
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12_mask(&mut self) -> SMI_12_MASK_W {
        SMI_12_MASK_W { w: self }
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13_mask(&mut self) -> SMI_13_MASK_W {
        SMI_13_MASK_W { w: self }
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14_mask(&mut self) -> SMI_14_MASK_W {
        SMI_14_MASK_W { w: self }
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_15_mask(&mut self) -> SMI_15_MASK_W {
        SMI_15_MASK_W { w: self }
    }
}
