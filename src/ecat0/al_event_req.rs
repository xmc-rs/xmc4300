#[doc = "Reader of register AL_EVENT_REQ"]
pub type R = crate::R<u32, super::AL_EVENT_REQ>;
#[doc = "Writer for register AL_EVENT_REQ"]
pub type W = crate::W<u32, super::AL_EVENT_REQ>;
#[doc = "Register AL_EVENT_REQ `reset()`'s with value 0x20"]
impl crate::ResetValue for super::AL_EVENT_REQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "AL Control event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_CE_A {
    #[doc = "0: No AL Control Register change"]
    VALUE1,
    #[doc = "1: AL Control Register has been written"]
    VALUE2,
}
impl From<AL_CE_A> for bool {
    #[inline(always)]
    fn from(variant: AL_CE_A) -> Self {
        match variant {
            AL_CE_A::VALUE1 => false,
            AL_CE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AL_CE`"]
pub type AL_CE_R = crate::R<bool, AL_CE_A>;
impl AL_CE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AL_CE_A {
        match self.bits {
            false => AL_CE_A::VALUE1,
            true => AL_CE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AL_CE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AL_CE_A::VALUE2
    }
}
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LE_A {
    #[doc = "0: No change on DC Latch Inputs"]
    VALUE1,
    #[doc = "1: At least one change on DC Latch Inputs"]
    VALUE2,
}
impl From<DC_LE_A> for bool {
    #[inline(always)]
    fn from(variant: DC_LE_A) -> Self {
        match variant {
            DC_LE_A::VALUE1 => false,
            DC_LE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DC_LE`"]
pub type DC_LE_R = crate::R<bool, DC_LE_A>;
impl DC_LE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_LE_A {
        match self.bits {
            false => DC_LE_A::VALUE1,
            true => DC_LE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_A::VALUE2
    }
}
#[doc = "Reader of field `ST_S0`"]
pub type ST_S0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ST_S1`"]
pub type ST_S1_R = crate::R<bool, bool>;
#[doc = "SyncManager activation register changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_A_A {
    #[doc = "0: No change in any SyncManager"]
    VALUE1,
    #[doc = "1: At least one change on DC Latch Inputs"]
    VALUE2,
}
impl From<SM_A_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A_A) -> Self {
        match variant {
            SM_A_A::VALUE1 => false,
            SM_A_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SM_A`"]
pub type SM_A_R = crate::R<bool, SM_A_A>;
impl SM_A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A_A {
        match self.bits {
            false => SM_A_A::VALUE1,
            true => SM_A_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SM_A_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SM_A_A::VALUE2
    }
}
#[doc = "EEPROM Emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEP_E_A {
    #[doc = "0: No command pending"]
    VALUE1,
    #[doc = "1: EEPROM command pending"]
    VALUE2,
}
impl From<EEP_E_A> for bool {
    #[inline(always)]
    fn from(variant: EEP_E_A) -> Self {
        match variant {
            EEP_E_A::VALUE1 => false,
            EEP_E_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EEP_E`"]
pub type EEP_E_R = crate::R<bool, EEP_E_A>;
impl EEP_E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEP_E_A {
        match self.bits {
            false => EEP_E_A::VALUE1,
            true => EEP_E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EEP_E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EEP_E_A::VALUE2
    }
}
#[doc = "Watchdog Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_D_A {
    #[doc = "0: Has not expired"]
    VALUE1,
    #[doc = "1: Has expired"]
    VALUE2,
}
impl From<WP_D_A> for bool {
    #[inline(always)]
    fn from(variant: WP_D_A) -> Self {
        match variant {
            WP_D_A::VALUE1 => false,
            WP_D_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WP_D`"]
pub type WP_D_R = crate::R<bool, WP_D_A>;
impl WP_D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_D_A {
        match self.bits {
            false => WP_D_A::VALUE1,
            true => WP_D_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WP_D_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WP_D_A::VALUE2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_0_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_0_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_0_A) -> Self {
        match variant {
            SMI_0_A::VALUE1 => false,
            SMI_0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_0`"]
pub type SMI_0_R = crate::R<bool, SMI_0_A>;
impl SMI_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_0_A {
        match self.bits {
            false => SMI_0_A::VALUE1,
            true => SMI_0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_0_A::VALUE2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_1_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_1_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_1_A) -> Self {
        match variant {
            SMI_1_A::VALUE1 => false,
            SMI_1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_1`"]
pub type SMI_1_R = crate::R<bool, SMI_1_A>;
impl SMI_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_1_A {
        match self.bits {
            false => SMI_1_A::VALUE1,
            true => SMI_1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_1_A::VALUE2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_2_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_2_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_2_A) -> Self {
        match variant {
            SMI_2_A::VALUE1 => false,
            SMI_2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_2`"]
pub type SMI_2_R = crate::R<bool, SMI_2_A>;
impl SMI_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_2_A {
        match self.bits {
            false => SMI_2_A::VALUE1,
            true => SMI_2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_2_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_2`"]
pub struct SMI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_2_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_2_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_3_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_3_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_3_A) -> Self {
        match variant {
            SMI_3_A::VALUE1 => false,
            SMI_3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_3`"]
pub type SMI_3_R = crate::R<bool, SMI_3_A>;
impl SMI_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_3_A {
        match self.bits {
            false => SMI_3_A::VALUE1,
            true => SMI_3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_3_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_3`"]
pub struct SMI_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_3_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_3_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_4_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_4_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_4_A) -> Self {
        match variant {
            SMI_4_A::VALUE1 => false,
            SMI_4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_4`"]
pub type SMI_4_R = crate::R<bool, SMI_4_A>;
impl SMI_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_4_A {
        match self.bits {
            false => SMI_4_A::VALUE1,
            true => SMI_4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_4_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_4`"]
pub struct SMI_4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_4_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_4_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_5_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_5_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_5_A) -> Self {
        match variant {
            SMI_5_A::VALUE1 => false,
            SMI_5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_5`"]
pub type SMI_5_R = crate::R<bool, SMI_5_A>;
impl SMI_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_5_A {
        match self.bits {
            false => SMI_5_A::VALUE1,
            true => SMI_5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_5_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_5`"]
pub struct SMI_5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_5_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_5_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_6_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_6_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_6_A) -> Self {
        match variant {
            SMI_6_A::VALUE1 => false,
            SMI_6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_6`"]
pub type SMI_6_R = crate::R<bool, SMI_6_A>;
impl SMI_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_6_A {
        match self.bits {
            false => SMI_6_A::VALUE1,
            true => SMI_6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_6_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_6`"]
pub struct SMI_6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_6_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_6_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_7_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_7_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_7_A) -> Self {
        match variant {
            SMI_7_A::VALUE1 => false,
            SMI_7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_7`"]
pub type SMI_7_R = crate::R<bool, SMI_7_A>;
impl SMI_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_7_A {
        match self.bits {
            false => SMI_7_A::VALUE1,
            true => SMI_7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_7_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_7`"]
pub struct SMI_7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_7_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_7_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_8_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_8_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_8_A) -> Self {
        match variant {
            SMI_8_A::VALUE1 => false,
            SMI_8_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_8`"]
pub type SMI_8_R = crate::R<bool, SMI_8_A>;
impl SMI_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_8_A {
        match self.bits {
            false => SMI_8_A::VALUE1,
            true => SMI_8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_8_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_8`"]
pub struct SMI_8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_8_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_8_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_9_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_9_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_9_A) -> Self {
        match variant {
            SMI_9_A::VALUE1 => false,
            SMI_9_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_9`"]
pub type SMI_9_R = crate::R<bool, SMI_9_A>;
impl SMI_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_9_A {
        match self.bits {
            false => SMI_9_A::VALUE1,
            true => SMI_9_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_9_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_9`"]
pub struct SMI_9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_9_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_9_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_10_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_10_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_10_A) -> Self {
        match variant {
            SMI_10_A::VALUE1 => false,
            SMI_10_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_10`"]
pub type SMI_10_R = crate::R<bool, SMI_10_A>;
impl SMI_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_10_A {
        match self.bits {
            false => SMI_10_A::VALUE1,
            true => SMI_10_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_10_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_10`"]
pub struct SMI_10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_10_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_10_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_11_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_11_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_11_A) -> Self {
        match variant {
            SMI_11_A::VALUE1 => false,
            SMI_11_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_11`"]
pub type SMI_11_R = crate::R<bool, SMI_11_A>;
impl SMI_11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_11_A {
        match self.bits {
            false => SMI_11_A::VALUE1,
            true => SMI_11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_11_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_11`"]
pub struct SMI_11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_11_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_11_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_12_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_12_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_12_A) -> Self {
        match variant {
            SMI_12_A::VALUE1 => false,
            SMI_12_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_12`"]
pub type SMI_12_R = crate::R<bool, SMI_12_A>;
impl SMI_12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_12_A {
        match self.bits {
            false => SMI_12_A::VALUE1,
            true => SMI_12_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_12_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_12`"]
pub struct SMI_12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_12_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_12_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_13_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_13_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_13_A) -> Self {
        match variant {
            SMI_13_A::VALUE1 => false,
            SMI_13_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_13`"]
pub type SMI_13_R = crate::R<bool, SMI_13_A>;
impl SMI_13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_13_A {
        match self.bits {
            false => SMI_13_A::VALUE1,
            true => SMI_13_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_13_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_13_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_13`"]
pub struct SMI_13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_13_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_13_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_14_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_14_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_14_A) -> Self {
        match variant {
            SMI_14_A::VALUE1 => false,
            SMI_14_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_14`"]
pub type SMI_14_R = crate::R<bool, SMI_14_A>;
impl SMI_14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_14_A {
        match self.bits {
            false => SMI_14_A::VALUE1,
            true => SMI_14_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_14_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_14_A::VALUE2
    }
}
#[doc = "Write proxy for field `SMI_14`"]
pub struct SMI_14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_14_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_14_A::VALUE2)
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
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_15_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2,
}
impl From<SMI_15_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_15_A) -> Self {
        match variant {
            SMI_15_A::VALUE1 => false,
            SMI_15_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SMI_15`"]
pub type SMI_15_R = crate::R<bool, SMI_15_A>;
impl SMI_15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_15_A {
        match self.bits {
            false => SMI_15_A::VALUE1,
            true => SMI_15_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_15_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_15_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    pub fn al_ce(&self) -> AL_CE_R {
        AL_CE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le(&self) -> DC_LE_R {
        DC_LE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    pub fn st_s0(&self) -> ST_S0_R {
        ST_S0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    pub fn st_s1(&self) -> ST_S1_R {
        ST_S1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    pub fn sm_a(&self) -> SM_A_R {
        SM_A_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    pub fn eep_e(&self) -> EEP_E_R {
        EEP_E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    pub fn wp_d(&self) -> WP_D_R {
        WP_D_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_0(&self) -> SMI_0_R {
        SMI_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_1(&self) -> SMI_1_R {
        SMI_1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2(&self) -> SMI_2_R {
        SMI_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3(&self) -> SMI_3_R {
        SMI_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4(&self) -> SMI_4_R {
        SMI_4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5(&self) -> SMI_5_R {
        SMI_5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6(&self) -> SMI_6_R {
        SMI_6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7(&self) -> SMI_7_R {
        SMI_7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8(&self) -> SMI_8_R {
        SMI_8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9(&self) -> SMI_9_R {
        SMI_9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10(&self) -> SMI_10_R {
        SMI_10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11(&self) -> SMI_11_R {
        SMI_11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12(&self) -> SMI_12_R {
        SMI_12_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13(&self) -> SMI_13_R {
        SMI_13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14(&self) -> SMI_14_R {
        SMI_14_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_15(&self) -> SMI_15_R {
        SMI_15_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2(&mut self) -> SMI_2_W {
        SMI_2_W { w: self }
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3(&mut self) -> SMI_3_W {
        SMI_3_W { w: self }
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4(&mut self) -> SMI_4_W {
        SMI_4_W { w: self }
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5(&mut self) -> SMI_5_W {
        SMI_5_W { w: self }
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6(&mut self) -> SMI_6_W {
        SMI_6_W { w: self }
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7(&mut self) -> SMI_7_W {
        SMI_7_W { w: self }
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8(&mut self) -> SMI_8_W {
        SMI_8_W { w: self }
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9(&mut self) -> SMI_9_W {
        SMI_9_W { w: self }
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10(&mut self) -> SMI_10_W {
        SMI_10_W { w: self }
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11(&mut self) -> SMI_11_W {
        SMI_11_W { w: self }
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12(&mut self) -> SMI_12_W {
        SMI_12_W { w: self }
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13(&mut self) -> SMI_13_W {
        SMI_13_W { w: self }
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14(&mut self) -> SMI_14_W {
        SMI_14_W { w: self }
    }
}
