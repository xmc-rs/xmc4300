#[doc = "Register `DX0CR` reader"]
pub struct R(crate::R<DX0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DX0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DX0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DX0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DX0CR` writer"]
pub struct W(crate::W<DX0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DX0CR_SPEC>;
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
impl From<crate::W<DX0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DX0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data Selection for Input Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSEL_A {
    #[doc = "0: The data input DXnA is selected."]
    VALUE1 = 0,
    #[doc = "1: The data input DXnB is selected."]
    VALUE2 = 1,
    #[doc = "2: The data input DXnC is selected."]
    VALUE3 = 2,
    #[doc = "3: The data input DXnD is selected."]
    VALUE4 = 3,
    #[doc = "4: The data input DXnE is selected."]
    VALUE5 = 4,
    #[doc = "5: The data input DXnF is selected."]
    VALUE6 = 5,
    #[doc = "6: The data input DXnG is selected."]
    VALUE7 = 6,
    #[doc = "7: The data input is always 1."]
    VALUE8 = 7,
}
impl From<DSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DSEL` reader - Data Selection for Input Signal"]
pub struct DSEL_R(crate::FieldReader<u8, DSEL_A>);
impl DSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSEL_A {
        match self.bits {
            0 => DSEL_A::VALUE1,
            1 => DSEL_A::VALUE2,
            2 => DSEL_A::VALUE3,
            3 => DSEL_A::VALUE4,
            4 => DSEL_A::VALUE5,
            5 => DSEL_A::VALUE6,
            6 => DSEL_A::VALUE7,
            7 => DSEL_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == DSEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == DSEL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == DSEL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == DSEL_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == DSEL_A::VALUE8
    }
}
impl core::ops::Deref for DSEL_R {
    type Target = crate::FieldReader<u8, DSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSEL` writer - Data Selection for Input Signal"]
pub struct DSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The data input DXnA is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE1)
    }
    #[doc = "The data input DXnB is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE2)
    }
    #[doc = "The data input DXnC is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE3)
    }
    #[doc = "The data input DXnD is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE4)
    }
    #[doc = "The data input DXnE is selected."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE5)
    }
    #[doc = "The data input DXnF is selected."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE6)
    }
    #[doc = "The data input DXnG is selected."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE7)
    }
    #[doc = "The data input is always 1."]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Input Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSW_A {
    #[doc = "0: The input of the data shift unit is controlled by the protocol pre-processor."]
    VALUE1 = 0,
    #[doc = "1: The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    VALUE2 = 1,
}
impl From<INSW_A> for bool {
    #[inline(always)]
    fn from(variant: INSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSW` reader - Input Switch"]
pub struct INSW_R(crate::FieldReader<bool, INSW_A>);
impl INSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        INSW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSW_A {
        match self.bits {
            false => INSW_A::VALUE1,
            true => INSW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INSW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INSW_A::VALUE2
    }
}
impl core::ops::Deref for INSW_R {
    type Target = crate::FieldReader<bool, INSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSW` writer - Input Switch"]
pub struct INSW_W<'a> {
    w: &'a mut W,
}
impl<'a> INSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSW_A::VALUE1)
    }
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSW_A::VALUE2)
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
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFEN_A {
    #[doc = "0: The input signal is not digitally filtered."]
    VALUE1 = 0,
    #[doc = "1: The input signal is digitally filtered."]
    VALUE2 = 1,
}
impl From<DFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFEN` reader - Digital Filter Enable"]
pub struct DFEN_R(crate::FieldReader<bool, DFEN_A>);
impl DFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFEN_A {
        match self.bits {
            false => DFEN_A::VALUE1,
            true => DFEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DFEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DFEN_A::VALUE2
    }
}
impl core::ops::Deref for DFEN_R {
    type Target = crate::FieldReader<bool, DFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFEN` writer - Digital Filter Enable"]
pub struct DFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input signal is not digitally filtered."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DFEN_A::VALUE1)
    }
    #[doc = "The input signal is digitally filtered."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DFEN_A::VALUE2)
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
#[doc = "Data Synchronization Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSEN_A {
    #[doc = "0: The un-synchronized signal can be taken as input for the data shift unit."]
    VALUE1 = 0,
    #[doc = "1: The synchronized signal can be taken as input for the data shift unit."]
    VALUE2 = 1,
}
impl From<DSEN_A> for bool {
    #[inline(always)]
    fn from(variant: DSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSEN` reader - Data Synchronization Enable"]
pub struct DSEN_R(crate::FieldReader<bool, DSEN_A>);
impl DSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSEN_A {
        match self.bits {
            false => DSEN_A::VALUE1,
            true => DSEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DSEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DSEN_A::VALUE2
    }
}
impl core::ops::Deref for DSEN_R {
    type Target = crate::FieldReader<bool, DSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSEN` writer - Data Synchronization Enable"]
pub struct DSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSEN_A::VALUE1)
    }
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Data Polarity for DXn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPOL_A {
    #[doc = "0: The input signal is not inverted."]
    VALUE1 = 0,
    #[doc = "1: The input signal is inverted."]
    VALUE2 = 1,
}
impl From<DPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPOL` reader - Data Polarity for DXn"]
pub struct DPOL_R(crate::FieldReader<bool, DPOL_A>);
impl DPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPOL_A {
        match self.bits {
            false => DPOL_A::VALUE1,
            true => DPOL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DPOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DPOL_A::VALUE2
    }
}
impl core::ops::Deref for DPOL_R {
    type Target = crate::FieldReader<bool, DPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPOL` writer - Data Polarity for DXn"]
pub struct DPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input signal is not inverted."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPOL_A::VALUE1)
    }
    #[doc = "The input signal is inverted."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPOL_A::VALUE2)
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
#[doc = "Sampling Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFSEL_A {
    #[doc = "0: The sampling frequency is fPB."]
    VALUE1 = 0,
    #[doc = "1: The sampling frequency is fFD."]
    VALUE2 = 1,
}
impl From<SFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFSEL` reader - Sampling Frequency Selection"]
pub struct SFSEL_R(crate::FieldReader<bool, SFSEL_A>);
impl SFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFSEL_A {
        match self.bits {
            false => SFSEL_A::VALUE1,
            true => SFSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SFSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SFSEL_A::VALUE2
    }
}
impl core::ops::Deref for SFSEL_R {
    type Target = crate::FieldReader<bool, SFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFSEL` writer - Sampling Frequency Selection"]
pub struct SFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The sampling frequency is fPB."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SFSEL_A::VALUE1)
    }
    #[doc = "The sampling frequency is fFD."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SFSEL_A::VALUE2)
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
#[doc = "Combination Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: The trigger activation is disabled."]
    VALUE1 = 0,
    #[doc = "1: A rising edge activates DXnT."]
    VALUE2 = 1,
    #[doc = "2: A falling edge activates DXnT."]
    VALUE3 = 2,
    #[doc = "3: Both edges activate DXnT."]
    VALUE4 = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CM` reader - Combination Mode"]
pub struct CM_R(crate::FieldReader<u8, CM_A>);
impl CM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::VALUE1,
            1 => CM_A::VALUE2,
            2 => CM_A::VALUE3,
            3 => CM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CM_A::VALUE4
    }
}
impl core::ops::Deref for CM_R {
    type Target = crate::FieldReader<u8, CM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM` writer - Combination Mode"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The trigger activation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CM_A::VALUE1)
    }
    #[doc = "A rising edge activates DXnT."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CM_A::VALUE2)
    }
    #[doc = "A falling edge activates DXnT."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CM_A::VALUE3)
    }
    #[doc = "Both edges activate DXnT."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Synchronized Data Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DXS_A {
    #[doc = "0: The current value of DXnS is 0."]
    VALUE1 = 0,
    #[doc = "1: The current value of DXnS is 1."]
    VALUE2 = 1,
}
impl From<DXS_A> for bool {
    #[inline(always)]
    fn from(variant: DXS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DXS` reader - Synchronized Data Value"]
pub struct DXS_R(crate::FieldReader<bool, DXS_A>);
impl DXS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DXS_A {
        match self.bits {
            false => DXS_A::VALUE1,
            true => DXS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DXS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DXS_A::VALUE2
    }
}
impl core::ops::Deref for DXS_R {
    type Target = crate::FieldReader<bool, DXS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline(always)]
    pub fn dsel(&self) -> DSEL_R {
        DSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline(always)]
    pub fn insw(&self) -> INSW_R {
        INSW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline(always)]
    pub fn dpol(&self) -> DPOL_R {
        DPOL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline(always)]
    pub fn sfsel(&self) -> SFSEL_R {
        SFSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Synchronized Data Value"]
    #[inline(always)]
    pub fn dxs(&self) -> DXS_R {
        DXS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline(always)]
    pub fn dsel(&mut self) -> DSEL_W {
        DSEL_W { w: self }
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline(always)]
    pub fn insw(&mut self) -> INSW_W {
        INSW_W { w: self }
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W {
        DFEN_W { w: self }
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline(always)]
    pub fn dsen(&mut self) -> DSEN_W {
        DSEN_W { w: self }
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline(always)]
    pub fn dpol(&mut self) -> DPOL_W {
        DPOL_W { w: self }
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline(always)]
    pub fn sfsel(&mut self) -> SFSEL_W {
        SFSEL_W { w: self }
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dx0cr](index.html) module"]
pub struct DX0CR_SPEC;
impl crate::RegisterSpec for DX0CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dx0cr::R](R) reader structure"]
impl crate::Readable for DX0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dx0cr::W](W) writer structure"]
impl crate::Writable for DX0CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DX0CR to value 0"]
impl crate::Resettable for DX0CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
