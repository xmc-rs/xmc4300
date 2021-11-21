#[doc = "Register `ASMR` reader"]
pub struct R(crate::R<ASMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASMR` writer"]
pub struct W(crate::W<ASMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASMR_SPEC>;
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
impl From<crate::W<ASMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENGT_A {
    #[doc = "0: No conversion requests are issued"]
    VALUE1 = 0,
    #[doc = "1: Conversion requests are issued if at least one pending bit is set"]
    VALUE2 = 1,
    #[doc = "2: Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    VALUE3 = 2,
    #[doc = "3: Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    VALUE4 = 3,
}
impl From<ENGT_A> for u8 {
    #[inline(always)]
    fn from(variant: ENGT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENGT` reader - Enable Gate"]
pub struct ENGT_R(crate::FieldReader<u8, ENGT_A>);
impl ENGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENGT_A {
        match self.bits {
            0 => ENGT_A::VALUE1,
            1 => ENGT_A::VALUE2,
            2 => ENGT_A::VALUE3,
            3 => ENGT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENGT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ENGT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ENGT_A::VALUE4
    }
}
impl core::ops::Deref for ENGT_R {
    type Target = crate::FieldReader<u8, ENGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENGT` writer - Enable Gate"]
pub struct ENGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENGT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE1)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE2)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE3)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Enable External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTR_A {
    #[doc = "0: External trigger disabled"]
    VALUE1 = 0,
    #[doc = "1: The selected edge at the selected trigger input signal REQTR generates the load event"]
    VALUE2 = 1,
}
impl From<ENTR_A> for bool {
    #[inline(always)]
    fn from(variant: ENTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENTR` reader - Enable External Trigger"]
pub struct ENTR_R(crate::FieldReader<bool, ENTR_A>);
impl ENTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENTR_A {
        match self.bits {
            false => ENTR_A::VALUE1,
            true => ENTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENTR_A::VALUE2
    }
}
impl core::ops::Deref for ENTR_R {
    type Target = crate::FieldReader<bool, ENTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENTR` writer - Enable External Trigger"]
pub struct ENTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENTR_A::VALUE1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENTR_A::VALUE2)
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
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSI_A {
    #[doc = "0: No request source interrupt"]
    VALUE1 = 0,
    #[doc = "1: A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    VALUE2 = 1,
}
impl From<ENSI_A> for bool {
    #[inline(always)]
    fn from(variant: ENSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub struct ENSI_R(crate::FieldReader<bool, ENSI_A>);
impl ENSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENSI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSI_A {
        match self.bits {
            false => ENSI_A::VALUE1,
            true => ENSI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENSI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENSI_A::VALUE2
    }
}
impl core::ops::Deref for ENSI_R {
    type Target = crate::FieldReader<bool, ENSI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENSI` writer - Enable Source Interrupt"]
pub struct ENSI_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENSI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENSI_A::VALUE1)
    }
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENSI_A::VALUE2)
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
#[doc = "Autoscan Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCAN_A {
    #[doc = "0: No autoscan"]
    VALUE1 = 0,
    #[doc = "1: Autoscan functionality enabled: a request source event automatically generates a load event"]
    VALUE2 = 1,
}
impl From<SCAN_A> for bool {
    #[inline(always)]
    fn from(variant: SCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCAN` reader - Autoscan Enable"]
pub struct SCAN_R(crate::FieldReader<bool, SCAN_A>);
impl SCAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCAN_A {
        match self.bits {
            false => SCAN_A::VALUE1,
            true => SCAN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SCAN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SCAN_A::VALUE2
    }
}
impl core::ops::Deref for SCAN_R {
    type Target = crate::FieldReader<bool, SCAN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN` writer - Autoscan Enable"]
pub struct SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCAN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No autoscan"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCAN_A::VALUE1)
    }
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCAN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Autoscan Source Load Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDM_A {
    #[doc = "0: Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    VALUE1 = 0,
    #[doc = "1: Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    VALUE2 = 1,
}
impl From<LDM_A> for bool {
    #[inline(always)]
    fn from(variant: LDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDM` reader - Autoscan Source Load Event Mode"]
pub struct LDM_R(crate::FieldReader<bool, LDM_A>);
impl LDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDM_A {
        match self.bits {
            false => LDM_A::VALUE1,
            true => LDM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LDM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LDM_A::VALUE2
    }
}
impl core::ops::Deref for LDM_R {
    type Target = crate::FieldReader<bool, LDM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDM` writer - Autoscan Source Load Event Mode"]
pub struct LDM_W<'a> {
    w: &'a mut W,
}
impl<'a> LDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LDM_A::VALUE1)
    }
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LDM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQGT_A {
    #[doc = "0: The gate input is low"]
    VALUE1 = 0,
    #[doc = "1: The gate input is high"]
    VALUE2 = 1,
}
impl From<REQGT_A> for bool {
    #[inline(always)]
    fn from(variant: REQGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQGT` reader - Request Gate Level"]
pub struct REQGT_R(crate::FieldReader<bool, REQGT_A>);
impl REQGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQGT_A {
        match self.bits {
            false => REQGT_A::VALUE1,
            true => REQGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REQGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REQGT_A::VALUE2
    }
}
impl core::ops::Deref for REQGT_R {
    type Target = crate::FieldReader<bool, REQGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Clear Pending Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRPND_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: The bits in register GxASPNDx are cleared"]
    VALUE2 = 1,
}
impl From<CLRPND_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRPND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRPND` writer - Clear Pending Bits"]
pub struct CLRPND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRPND_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRPND_AW::VALUE1)
    }
    #[doc = "The bits in register GxASPNDx are cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLRPND_AW::VALUE2)
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
#[doc = "Generate Load Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDEV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: A load event is generated"]
    VALUE2 = 1,
}
impl From<LDEV_AW> for bool {
    #[inline(always)]
    fn from(variant: LDEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDEV` writer - Generate Load Event"]
pub struct LDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> LDEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LDEV_AW::VALUE1)
    }
    #[doc = "A load event is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LDEV_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Repeat Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTDIS_A {
    #[doc = "0: A cancelled conversion is repeated"]
    VALUE1 = 0,
    #[doc = "1: A cancelled conversion is discarded"]
    VALUE2 = 1,
}
impl From<RPTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPTDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPTDIS` reader - Repeat Disable"]
pub struct RPTDIS_R(crate::FieldReader<bool, RPTDIS_A>);
impl RPTDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPTDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPTDIS_A {
        match self.bits {
            false => RPTDIS_A::VALUE1,
            true => RPTDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RPTDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RPTDIS_A::VALUE2
    }
}
impl core::ops::Deref for RPTDIS_R {
    type Target = crate::FieldReader<bool, RPTDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPTDIS` writer - Repeat Disable"]
pub struct RPTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPTDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPTDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RPTDIS_A::VALUE1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RPTDIS_A::VALUE2)
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
impl R {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&self) -> ENGT_R {
        ENGT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&self) -> ENTR_R {
        ENTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> ENSI_R {
        ENSI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    pub fn ldm(&self) -> LDM_R {
        LDM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> REQGT_R {
        REQGT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&self) -> RPTDIS_R {
        RPTDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&mut self) -> ENGT_W {
        ENGT_W { w: self }
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&mut self) -> ENTR_W {
        ENTR_W { w: self }
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&mut self) -> ENSI_W {
        ENSI_W { w: self }
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W { w: self }
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    pub fn ldm(&mut self) -> LDM_W {
        LDM_W { w: self }
    }
    #[doc = "Bit 8 - Clear Pending Bits"]
    #[inline(always)]
    pub fn clrpnd(&mut self) -> CLRPND_W {
        CLRPND_W { w: self }
    }
    #[doc = "Bit 9 - Generate Load Event"]
    #[inline(always)]
    pub fn ldev(&mut self) -> LDEV_W {
        LDEV_W { w: self }
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&mut self) -> RPTDIS_W {
        RPTDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autoscan Source Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asmr](index.html) module"]
pub struct ASMR_SPEC;
impl crate::RegisterSpec for ASMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asmr::R](R) reader structure"]
impl crate::Readable for ASMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asmr::W](W) writer structure"]
impl crate::Writable for ASMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASMR to value 0"]
impl crate::Resettable for ASMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
