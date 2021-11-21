#[doc = "Register `AL_EVENT_REQ` reader"]
pub struct R(crate::R<AL_EVENT_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AL_EVENT_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AL_EVENT_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AL_EVENT_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AL_EVENT_REQ` writer"]
pub struct W(crate::W<AL_EVENT_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AL_EVENT_REQ_SPEC>;
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
impl From<crate::W<AL_EVENT_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AL_EVENT_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AL Control event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_CE_A {
    #[doc = "0: No AL Control Register change"]
    VALUE1 = 0,
    #[doc = "1: AL Control Register has been written"]
    VALUE2 = 1,
}
impl From<AL_CE_A> for bool {
    #[inline(always)]
    fn from(variant: AL_CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_CE` reader - AL Control event"]
pub struct AL_CE_R(crate::FieldReader<bool, AL_CE_A>);
impl AL_CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AL_CE_R(crate::FieldReader::new(bits))
    }
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
        **self == AL_CE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AL_CE_A::VALUE2
    }
}
impl core::ops::Deref for AL_CE_R {
    type Target = crate::FieldReader<bool, AL_CE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LE_A {
    #[doc = "0: No change on DC Latch Inputs"]
    VALUE1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    VALUE2 = 1,
}
impl From<DC_LE_A> for bool {
    #[inline(always)]
    fn from(variant: DC_LE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE` reader - DC Latch event"]
pub struct DC_LE_R(crate::FieldReader<bool, DC_LE_A>);
impl DC_LE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_LE_R(crate::FieldReader::new(bits))
    }
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
        **self == DC_LE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DC_LE_A::VALUE2
    }
}
impl core::ops::Deref for DC_LE_R {
    type Target = crate::FieldReader<bool, DC_LE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST_S0` reader - State of DC SYNC0"]
pub struct ST_S0_R(crate::FieldReader<bool, bool>);
impl ST_S0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ST_S0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_S0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST_S1` reader - State of DC SYNC1"]
pub struct ST_S1_R(crate::FieldReader<bool, bool>);
impl ST_S1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ST_S1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_S1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SyncManager activation register changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_A_A {
    #[doc = "0: No change in any SyncManager"]
    VALUE1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    VALUE2 = 1,
}
impl From<SM_A_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM_A` reader - SyncManager activation register changed"]
pub struct SM_A_R(crate::FieldReader<bool, SM_A_A>);
impl SM_A_R {
    pub(crate) fn new(bits: bool) -> Self {
        SM_A_R(crate::FieldReader::new(bits))
    }
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
        **self == SM_A_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SM_A_A::VALUE2
    }
}
impl core::ops::Deref for SM_A_R {
    type Target = crate::FieldReader<bool, SM_A_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "EEPROM Emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEP_E_A {
    #[doc = "0: No command pending"]
    VALUE1 = 0,
    #[doc = "1: EEPROM command pending"]
    VALUE2 = 1,
}
impl From<EEP_E_A> for bool {
    #[inline(always)]
    fn from(variant: EEP_E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEP_E` reader - EEPROM Emulation"]
pub struct EEP_E_R(crate::FieldReader<bool, EEP_E_A>);
impl EEP_E_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEP_E_R(crate::FieldReader::new(bits))
    }
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
        **self == EEP_E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EEP_E_A::VALUE2
    }
}
impl core::ops::Deref for EEP_E_R {
    type Target = crate::FieldReader<bool, EEP_E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_D_A {
    #[doc = "0: Has not expired"]
    VALUE1 = 0,
    #[doc = "1: Has expired"]
    VALUE2 = 1,
}
impl From<WP_D_A> for bool {
    #[inline(always)]
    fn from(variant: WP_D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_D` reader - Watchdog Process Data"]
pub struct WP_D_R(crate::FieldReader<bool, WP_D_A>);
impl WP_D_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP_D_R(crate::FieldReader::new(bits))
    }
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
        **self == WP_D_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WP_D_A::VALUE2
    }
}
impl core::ops::Deref for WP_D_R {
    type Target = crate::FieldReader<bool, WP_D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_0_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_0_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_0` reader - SyncManager interrupt"]
pub struct SMI_0_R(crate::FieldReader<bool, SMI_0_A>);
impl SMI_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_0_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_0_A::VALUE2
    }
}
impl core::ops::Deref for SMI_0_R {
    type Target = crate::FieldReader<bool, SMI_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_1_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_1_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_1` reader - SyncManager interrupt"]
pub struct SMI_1_R(crate::FieldReader<bool, SMI_1_A>);
impl SMI_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_1_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_1_A::VALUE2
    }
}
impl core::ops::Deref for SMI_1_R {
    type Target = crate::FieldReader<bool, SMI_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_2_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_2_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_2` reader - SyncManager interrupt"]
pub struct SMI_2_R(crate::FieldReader<bool, SMI_2_A>);
impl SMI_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_2_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_2_A::VALUE2
    }
}
impl core::ops::Deref for SMI_2_R {
    type Target = crate::FieldReader<bool, SMI_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_2` writer - SyncManager interrupt"]
pub struct SMI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_2_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_3_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_3_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_3` reader - SyncManager interrupt"]
pub struct SMI_3_R(crate::FieldReader<bool, SMI_3_A>);
impl SMI_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_3_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_3_A::VALUE2
    }
}
impl core::ops::Deref for SMI_3_R {
    type Target = crate::FieldReader<bool, SMI_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_3` writer - SyncManager interrupt"]
pub struct SMI_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_3_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_4_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_4_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_4` reader - SyncManager interrupt"]
pub struct SMI_4_R(crate::FieldReader<bool, SMI_4_A>);
impl SMI_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_4_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_4_A::VALUE2
    }
}
impl core::ops::Deref for SMI_4_R {
    type Target = crate::FieldReader<bool, SMI_4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_4` writer - SyncManager interrupt"]
pub struct SMI_4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_4_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_5_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_5_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_5` reader - SyncManager interrupt"]
pub struct SMI_5_R(crate::FieldReader<bool, SMI_5_A>);
impl SMI_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_5_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_5_A::VALUE2
    }
}
impl core::ops::Deref for SMI_5_R {
    type Target = crate::FieldReader<bool, SMI_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_5` writer - SyncManager interrupt"]
pub struct SMI_5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_5_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_6_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_6_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_6` reader - SyncManager interrupt"]
pub struct SMI_6_R(crate::FieldReader<bool, SMI_6_A>);
impl SMI_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_6_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_6_A::VALUE2
    }
}
impl core::ops::Deref for SMI_6_R {
    type Target = crate::FieldReader<bool, SMI_6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_6` writer - SyncManager interrupt"]
pub struct SMI_6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_6_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_7_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_7_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_7` reader - SyncManager interrupt"]
pub struct SMI_7_R(crate::FieldReader<bool, SMI_7_A>);
impl SMI_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_7_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_7_A::VALUE2
    }
}
impl core::ops::Deref for SMI_7_R {
    type Target = crate::FieldReader<bool, SMI_7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_7` writer - SyncManager interrupt"]
pub struct SMI_7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_7_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_8_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_8_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_8` reader - SyncManager interrupt"]
pub struct SMI_8_R(crate::FieldReader<bool, SMI_8_A>);
impl SMI_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_8_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_8_A::VALUE2
    }
}
impl core::ops::Deref for SMI_8_R {
    type Target = crate::FieldReader<bool, SMI_8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_8` writer - SyncManager interrupt"]
pub struct SMI_8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_8_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_9_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_9_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_9` reader - SyncManager interrupt"]
pub struct SMI_9_R(crate::FieldReader<bool, SMI_9_A>);
impl SMI_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_9_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_9_A::VALUE2
    }
}
impl core::ops::Deref for SMI_9_R {
    type Target = crate::FieldReader<bool, SMI_9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_9` writer - SyncManager interrupt"]
pub struct SMI_9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_9_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_10_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_10_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_10` reader - SyncManager interrupt"]
pub struct SMI_10_R(crate::FieldReader<bool, SMI_10_A>);
impl SMI_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_10_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_10_A::VALUE2
    }
}
impl core::ops::Deref for SMI_10_R {
    type Target = crate::FieldReader<bool, SMI_10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_10` writer - SyncManager interrupt"]
pub struct SMI_10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_10_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_11_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_11_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_11` reader - SyncManager interrupt"]
pub struct SMI_11_R(crate::FieldReader<bool, SMI_11_A>);
impl SMI_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_11_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_11_A::VALUE2
    }
}
impl core::ops::Deref for SMI_11_R {
    type Target = crate::FieldReader<bool, SMI_11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_11` writer - SyncManager interrupt"]
pub struct SMI_11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_11_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_12_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_12_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_12` reader - SyncManager interrupt"]
pub struct SMI_12_R(crate::FieldReader<bool, SMI_12_A>);
impl SMI_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_12_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_12_A::VALUE2
    }
}
impl core::ops::Deref for SMI_12_R {
    type Target = crate::FieldReader<bool, SMI_12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_12` writer - SyncManager interrupt"]
pub struct SMI_12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_12_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_13_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_13_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_13` reader - SyncManager interrupt"]
pub struct SMI_13_R(crate::FieldReader<bool, SMI_13_A>);
impl SMI_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_13_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_13_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_13_A::VALUE2
    }
}
impl core::ops::Deref for SMI_13_R {
    type Target = crate::FieldReader<bool, SMI_13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_13` writer - SyncManager interrupt"]
pub struct SMI_13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_13_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_14_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_14_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_14` reader - SyncManager interrupt"]
pub struct SMI_14_R(crate::FieldReader<bool, SMI_14_A>);
impl SMI_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_14_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_14_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_14_A::VALUE2
    }
}
impl core::ops::Deref for SMI_14_R {
    type Target = crate::FieldReader<bool, SMI_14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMI_14` writer - SyncManager interrupt"]
pub struct SMI_14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMI_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMI_14_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_15_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_15_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_15` reader - SyncManager interrupt"]
pub struct SMI_15_R(crate::FieldReader<bool, SMI_15_A>);
impl SMI_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMI_15_R(crate::FieldReader::new(bits))
    }
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
        **self == SMI_15_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SMI_15_A::VALUE2
    }
}
impl core::ops::Deref for SMI_15_R {
    type Target = crate::FieldReader<bool, SMI_15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AL Event Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_event_req](index.html) module"]
pub struct AL_EVENT_REQ_SPEC;
impl crate::RegisterSpec for AL_EVENT_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [al_event_req::R](R) reader structure"]
impl crate::Readable for AL_EVENT_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [al_event_req::W](W) writer structure"]
impl crate::Writable for AL_EVENT_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AL_EVENT_REQ to value 0x20"]
impl crate::Resettable for AL_EVENT_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
