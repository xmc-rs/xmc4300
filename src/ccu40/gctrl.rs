#[doc = "Register `GCTRL` reader"]
pub struct R(crate::R<GCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCTRL` writer"]
pub struct W(crate::W<GCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCTRL_SPEC>;
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
impl From<crate::W<GCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescaler Clear Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRBC_A {
    #[doc = "0: SW only"]
    VALUE1 = 0,
    #[doc = "1: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC40 is cleared."]
    VALUE2 = 1,
    #[doc = "2: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC41 is cleared."]
    VALUE3 = 2,
    #[doc = "3: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC42 is cleared."]
    VALUE4 = 3,
    #[doc = "4: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC43 is cleared."]
    VALUE5 = 4,
}
impl From<PRBC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRBC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRBC` reader - Prescaler Clear Configuration"]
pub struct PRBC_R(crate::FieldReader<u8, PRBC_A>);
impl PRBC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRBC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRBC_A> {
        match self.bits {
            0 => Some(PRBC_A::VALUE1),
            1 => Some(PRBC_A::VALUE2),
            2 => Some(PRBC_A::VALUE3),
            3 => Some(PRBC_A::VALUE4),
            4 => Some(PRBC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRBC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRBC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PRBC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PRBC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == PRBC_A::VALUE5
    }
}
impl core::ops::Deref for PRBC_R {
    type Target = crate::FieldReader<u8, PRBC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRBC` writer - Prescaler Clear Configuration"]
pub struct PRBC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRBC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SW only"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE1)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC40 is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE2)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC41 is cleared."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE3)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC42 is cleared."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE4)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC43 is cleared."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PRBC_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Prescaler Input Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCIS_A {
    #[doc = "0: Module clock"]
    VALUE1 = 0,
    #[doc = "1: CCU4x.ECLKA"]
    VALUE2 = 1,
    #[doc = "2: CCU4x.ECLKB"]
    VALUE3 = 2,
    #[doc = "3: CCU4x.ECLKC"]
    VALUE4 = 3,
}
impl From<PCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCIS` reader - Prescaler Input Clock Selection"]
pub struct PCIS_R(crate::FieldReader<u8, PCIS_A>);
impl PCIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCIS_A {
        match self.bits {
            0 => PCIS_A::VALUE1,
            1 => PCIS_A::VALUE2,
            2 => PCIS_A::VALUE3,
            3 => PCIS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PCIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PCIS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PCIS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PCIS_A::VALUE4
    }
}
impl core::ops::Deref for PCIS_R {
    type Target = crate::FieldReader<u8, PCIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIS` writer - Prescaler Input Clock Selection"]
pub struct PCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCIS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Module clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE1)
    }
    #[doc = "CCU4x.ECLKA"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE2)
    }
    #[doc = "CCU4x.ECLKB"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE3)
    }
    #[doc = "CCU4x.ECLKC"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PCIS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Suspend Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUSCFG_A {
    #[doc = "0: Suspend request ignored. The module never enters in suspend"]
    VALUE1 = 0,
    #[doc = "1: Stops all the running slices immediately. Safe stop is not applied."]
    VALUE2 = 1,
    #[doc = "2: Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    VALUE3 = 2,
    #[doc = "3: Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    VALUE4 = 3,
}
impl From<SUSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUSCFG` reader - Suspend Mode Configuration"]
pub struct SUSCFG_R(crate::FieldReader<u8, SUSCFG_A>);
impl SUSCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUSCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSCFG_A {
        match self.bits {
            0 => SUSCFG_A::VALUE1,
            1 => SUSCFG_A::VALUE2,
            2 => SUSCFG_A::VALUE3,
            3 => SUSCFG_A::VALUE4,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SUSCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SUSCFG_A::VALUE4
    }
}
impl core::ops::Deref for SUSCFG_R {
    type Target = crate::FieldReader<u8, SUSCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSCFG` writer - Suspend Mode Configuration"]
pub struct SUSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE2)
    }
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE3)
    }
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Slice 0 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE0_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE0_A> for bool {
    #[inline(always)]
    fn from(variant: MSE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE0` reader - Slice 0 Multi Channel shadow transfer enable"]
pub struct MSE0_R(crate::FieldReader<bool, MSE0_A>);
impl MSE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE0_A {
        match self.bits {
            false => MSE0_A::VALUE1,
            true => MSE0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSE0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSE0_A::VALUE2
    }
}
impl core::ops::Deref for MSE0_R {
    type Target = crate::FieldReader<bool, MSE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSE0` writer - Slice 0 Multi Channel shadow transfer enable"]
pub struct MSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE0_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE0_A::VALUE2)
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
#[doc = "Slice 1 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE1_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE1_A> for bool {
    #[inline(always)]
    fn from(variant: MSE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE1` reader - Slice 1 Multi Channel shadow transfer enable"]
pub struct MSE1_R(crate::FieldReader<bool, MSE1_A>);
impl MSE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE1_A {
        match self.bits {
            false => MSE1_A::VALUE1,
            true => MSE1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSE1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSE1_A::VALUE2
    }
}
impl core::ops::Deref for MSE1_R {
    type Target = crate::FieldReader<bool, MSE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSE1` writer - Slice 1 Multi Channel shadow transfer enable"]
pub struct MSE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE1_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE1_A::VALUE2)
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
#[doc = "Slice 2 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE2_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE2_A> for bool {
    #[inline(always)]
    fn from(variant: MSE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE2` reader - Slice 2 Multi Channel shadow transfer enable"]
pub struct MSE2_R(crate::FieldReader<bool, MSE2_A>);
impl MSE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE2_A {
        match self.bits {
            false => MSE2_A::VALUE1,
            true => MSE2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSE2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSE2_A::VALUE2
    }
}
impl core::ops::Deref for MSE2_R {
    type Target = crate::FieldReader<bool, MSE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSE2` writer - Slice 2 Multi Channel shadow transfer enable"]
pub struct MSE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE2_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE2_A::VALUE2)
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
#[doc = "Slice 3 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE3_A {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    VALUE2 = 1,
}
impl From<MSE3_A> for bool {
    #[inline(always)]
    fn from(variant: MSE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE3` reader - Slice 3 Multi Channel shadow transfer enable"]
pub struct MSE3_R(crate::FieldReader<bool, MSE3_A>);
impl MSE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE3_A {
        match self.bits {
            false => MSE3_A::VALUE1,
            true => MSE3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSE3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSE3_A::VALUE2
    }
}
impl core::ops::Deref for MSE3_R {
    type Target = crate::FieldReader<bool, MSE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSE3` writer - Slice 3 Multi Channel shadow transfer enable"]
pub struct MSE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE3_A::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE3_A::VALUE2)
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
#[doc = "Multi Channel shadow transfer request configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSDE_A {
    #[doc = "0: Only the shadow transfer for period and compare values is requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer for the compare, period and prescaler compare values is requested"]
    VALUE2 = 1,
    #[doc = "3: Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    VALUE4 = 3,
}
impl From<MSDE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSDE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSDE` reader - Multi Channel shadow transfer request configuration"]
pub struct MSDE_R(crate::FieldReader<u8, MSDE_A>);
impl MSDE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSDE_A> {
        match self.bits {
            0 => Some(MSDE_A::VALUE1),
            1 => Some(MSDE_A::VALUE2),
            3 => Some(MSDE_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSDE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MSDE_A::VALUE4
    }
}
impl core::ops::Deref for MSDE_R {
    type Target = crate::FieldReader<u8, MSDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSDE` writer - Multi Channel shadow transfer request configuration"]
pub struct MSDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSDE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE1)
    }
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE2)
    }
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSDE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    pub fn prbc(&self) -> PRBC_R {
        PRBC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    pub fn pcis(&self) -> PCIS_R {
        PCIS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse0(&self) -> MSE0_R {
        MSE0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse1(&self) -> MSE1_R {
        MSE1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse2(&self) -> MSE2_R {
        MSE2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse3(&self) -> MSE3_R {
        MSE3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    pub fn msde(&self) -> MSDE_R {
        MSDE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    pub fn prbc(&mut self) -> PRBC_W {
        PRBC_W { w: self }
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    pub fn pcis(&mut self) -> PCIS_W {
        PCIS_W { w: self }
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn suscfg(&mut self) -> SUSCFG_W {
        SUSCFG_W { w: self }
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse0(&mut self) -> MSE0_W {
        MSE0_W { w: self }
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse1(&mut self) -> MSE1_W {
        MSE1_W { w: self }
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse2(&mut self) -> MSE2_W {
        MSE2_W { w: self }
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse3(&mut self) -> MSE3_W {
        MSE3_W { w: self }
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    pub fn msde(&mut self) -> MSDE_W {
        MSDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gctrl](index.html) module"]
pub struct GCTRL_SPEC;
impl crate::RegisterSpec for GCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gctrl::R](R) reader structure"]
impl crate::Readable for GCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gctrl::W](W) writer structure"]
impl crate::Writable for GCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCTRL to value 0"]
impl crate::Resettable for GCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
