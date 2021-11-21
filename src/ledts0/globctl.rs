#[doc = "Register `GLOBCTL` reader"]
pub struct R(crate::R<GLOBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBCTL` writer"]
pub struct W(crate::W<GLOBCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBCTL_SPEC>;
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
impl From<crate::W<GLOBCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_EN` reader - Touch-Sense Function Enable"]
pub struct TS_EN_R(crate::FieldReader<bool, bool>);
impl TS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_EN` writer - Touch-Sense Function Enable"]
pub struct TS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_EN_W<'a> {
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
#[doc = "Field `LD_EN` reader - LED Function Enable"]
pub struct LD_EN_R(crate::FieldReader<bool, bool>);
impl LD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LD_EN` writer - LED Function Enable"]
pub struct LD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LD_EN_W<'a> {
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
#[doc = "Clock Master Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTR_A {
    #[doc = "0: Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    VALUE1 = 0,
    #[doc = "1: LEDTS-counter takes its clock from another master kernel"]
    VALUE2 = 1,
}
impl From<CMTR_A> for bool {
    #[inline(always)]
    fn from(variant: CMTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTR` reader - Clock Master Disable"]
pub struct CMTR_R(crate::FieldReader<bool, CMTR_A>);
impl CMTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTR_A {
        match self.bits {
            false => CMTR_A::VALUE1,
            true => CMTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMTR_A::VALUE2
    }
}
impl core::ops::Deref for CMTR_R {
    type Target = crate::FieldReader<bool, CMTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMTR` writer - Clock Master Disable"]
pub struct CMTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMTR_A::VALUE1)
    }
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMTR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable Autoscan Time Period Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSYNC_A {
    #[doc = "0: No synchronization"]
    VALUE1 = 0,
    #[doc = "1: Synchronization enabled on Kernel0 autoscan time period"]
    VALUE2 = 1,
}
impl From<ENSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: ENSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSYNC` reader - Enable Autoscan Time Period Synchronization"]
pub struct ENSYNC_R(crate::FieldReader<bool, ENSYNC_A>);
impl ENSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSYNC_A {
        match self.bits {
            false => ENSYNC_A::VALUE1,
            true => ENSYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENSYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENSYNC_A::VALUE2
    }
}
impl core::ops::Deref for ENSYNC_R {
    type Target = crate::FieldReader<bool, ENSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENSYNC` writer - Enable Autoscan Time Period Synchronization"]
pub struct ENSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No synchronization"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENSYNC_A::VALUE1)
    }
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENSYNC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Suspend Request Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSCFG_A {
    #[doc = "0: Ignore suspend request"]
    VALUE1 = 0,
    #[doc = "1: Enable suspend according to request"]
    VALUE2 = 1,
}
impl From<SUSCFG_A> for bool {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSCFG` reader - Suspend Request Configuration"]
pub struct SUSCFG_R(crate::FieldReader<bool, SUSCFG_A>);
impl SUSCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSCFG_A {
        match self.bits {
            false => SUSCFG_A::VALUE1,
            true => SUSCFG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SUSCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SUSCFG_A::VALUE2
    }
}
impl core::ops::Deref for SUSCFG_R {
    type Target = crate::FieldReader<bool, SUSCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSCFG` writer - Suspend Request Configuration"]
pub struct SUSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignore suspend request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "Enable suspend according to request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE2)
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
#[doc = "Mask Number of LSB Bits for Event Validation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASKVAL_A {
    #[doc = "0: Mask LSB bit"]
    VALUE1 = 0,
    #[doc = "1: Mask 2 LSB bits"]
    VALUE2 = 1,
    #[doc = "7: Mask 8 LSB bits"]
    VALUE3 = 7,
}
impl From<MASKVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MASKVAL` reader - Mask Number of LSB Bits for Event Validation"]
pub struct MASKVAL_R(crate::FieldReader<u8, MASKVAL_A>);
impl MASKVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASKVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASKVAL_A> {
        match self.bits {
            0 => Some(MASKVAL_A::VALUE1),
            1 => Some(MASKVAL_A::VALUE2),
            7 => Some(MASKVAL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MASKVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MASKVAL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MASKVAL_A::VALUE3
    }
}
impl core::ops::Deref for MASKVAL_R {
    type Target = crate::FieldReader<u8, MASKVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASKVAL` writer - Mask Number of LSB Bits for Event Validation"]
pub struct MASKVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASKVAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Mask LSB bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MASKVAL_A::VALUE1)
    }
    #[doc = "Mask 2 LSB bits"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MASKVAL_A::VALUE2)
    }
    #[doc = "Mask 8 LSB bits"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MASKVAL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Enable (Extended) Time Frame Validation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FENVAL_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<FENVAL_A> for bool {
    #[inline(always)]
    fn from(variant: FENVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FENVAL` reader - Enable (Extended) Time Frame Validation"]
pub struct FENVAL_R(crate::FieldReader<bool, FENVAL_A>);
impl FENVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FENVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FENVAL_A {
        match self.bits {
            false => FENVAL_A::VALUE1,
            true => FENVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FENVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FENVAL_A::VALUE2
    }
}
impl core::ops::Deref for FENVAL_R {
    type Target = crate::FieldReader<bool, FENVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FENVAL` writer - Enable (Extended) Time Frame Validation"]
pub struct FENVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FENVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FENVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FENVAL_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FENVAL_A::VALUE2)
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
#[doc = "Enable Time Slice Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITS_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ITS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ITS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITS_EN` reader - Enable Time Slice Interrupt"]
pub struct ITS_EN_R(crate::FieldReader<bool, ITS_EN_A>);
impl ITS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITS_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITS_EN_A {
        match self.bits {
            false => ITS_EN_A::VALUE1,
            true => ITS_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ITS_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ITS_EN_A::VALUE2
    }
}
impl core::ops::Deref for ITS_EN_R {
    type Target = crate::FieldReader<bool, ITS_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS_EN` writer - Enable Time Slice Interrupt"]
pub struct ITS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITS_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITS_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITS_EN_A::VALUE2)
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
#[doc = "Enable (Extended) Time Frame Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITF_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ITF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ITF_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITF_EN` reader - Enable (Extended) Time Frame Interrupt"]
pub struct ITF_EN_R(crate::FieldReader<bool, ITF_EN_A>);
impl ITF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITF_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITF_EN_A {
        match self.bits {
            false => ITF_EN_A::VALUE1,
            true => ITF_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ITF_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ITF_EN_A::VALUE2
    }
}
impl core::ops::Deref for ITF_EN_R {
    type Target = crate::FieldReader<bool, ITF_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITF_EN` writer - Enable (Extended) Time Frame Interrupt"]
pub struct ITF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITF_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITF_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITF_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITF_EN_A::VALUE2)
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
#[doc = "Enable Autoscan Time Period Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITP_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable (valid only for case of hardware-enabled pad turn control)"]
    VALUE2 = 1,
}
impl From<ITP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ITP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITP_EN` reader - Enable Autoscan Time Period Interrupt"]
pub struct ITP_EN_R(crate::FieldReader<bool, ITP_EN_A>);
impl ITP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITP_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITP_EN_A {
        match self.bits {
            false => ITP_EN_A::VALUE1,
            true => ITP_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ITP_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ITP_EN_A::VALUE2
    }
}
impl core::ops::Deref for ITP_EN_R {
    type Target = crate::FieldReader<bool, ITP_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITP_EN` writer - Enable Autoscan Time Period Interrupt"]
pub struct ITP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITP_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITP_EN_A::VALUE1)
    }
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITP_EN_A::VALUE2)
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
#[doc = "Field `CLK_PS` reader - LEDTS-Counter Clock Pre-Scale Factor"]
pub struct CLK_PS_R(crate::FieldReader<u16, u16>);
impl CLK_PS_R {
    pub(crate) fn new(bits: u16) -> Self {
        CLK_PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_PS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_PS` writer - LEDTS-Counter Clock Pre-Scale Factor"]
pub struct CLK_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline(always)]
    pub fn ts_en(&self) -> TS_EN_R {
        TS_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline(always)]
    pub fn ld_en(&self) -> LD_EN_R {
        LD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline(always)]
    pub fn cmtr(&self) -> CMTR_R {
        CMTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline(always)]
    pub fn ensync(&self) -> ENSYNC_R {
        ENSYNC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline(always)]
    pub fn maskval(&self) -> MASKVAL_R {
        MASKVAL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline(always)]
    pub fn fenval(&self) -> FENVAL_R {
        FENVAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline(always)]
    pub fn its_en(&self) -> ITS_EN_R {
        ITS_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline(always)]
    pub fn itf_en(&self) -> ITF_EN_R {
        ITF_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline(always)]
    pub fn itp_en(&self) -> ITP_EN_R {
        ITP_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline(always)]
    pub fn clk_ps(&self) -> CLK_PS_R {
        CLK_PS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline(always)]
    pub fn ts_en(&mut self) -> TS_EN_W {
        TS_EN_W { w: self }
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline(always)]
    pub fn ld_en(&mut self) -> LD_EN_W {
        LD_EN_W { w: self }
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline(always)]
    pub fn cmtr(&mut self) -> CMTR_W {
        CMTR_W { w: self }
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline(always)]
    pub fn ensync(&mut self) -> ENSYNC_W {
        ENSYNC_W { w: self }
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline(always)]
    pub fn suscfg(&mut self) -> SUSCFG_W {
        SUSCFG_W { w: self }
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline(always)]
    pub fn maskval(&mut self) -> MASKVAL_W {
        MASKVAL_W { w: self }
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline(always)]
    pub fn fenval(&mut self) -> FENVAL_W {
        FENVAL_W { w: self }
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline(always)]
    pub fn its_en(&mut self) -> ITS_EN_W {
        ITS_EN_W { w: self }
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline(always)]
    pub fn itf_en(&mut self) -> ITF_EN_W {
        ITF_EN_W { w: self }
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline(always)]
    pub fn itp_en(&mut self) -> ITP_EN_W {
        ITP_EN_W { w: self }
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline(always)]
    pub fn clk_ps(&mut self) -> CLK_PS_W {
        CLK_PS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globctl](index.html) module"]
pub struct GLOBCTL_SPEC;
impl crate::RegisterSpec for GLOBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globctl::R](R) reader structure"]
impl crate::Readable for GLOBCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globctl::W](W) writer structure"]
impl crate::Writable for GLOBCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOBCTL to value 0"]
impl crate::Resettable for GLOBCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
