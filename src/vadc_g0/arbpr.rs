#[doc = "Register `ARBPR` reader"]
pub struct R(crate::R<ARBPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARBPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARBPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARBPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARBPR` writer"]
pub struct W(crate::W<ARBPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARBPR_SPEC>;
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
impl From<crate::W<ARBPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARBPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRIO0_A {
    #[doc = "0: Lowest priority is selected."]
    VALUE1 = 0,
    #[doc = "3: Highest priority is selected."]
    VALUE2 = 3,
}
impl From<PRIO0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRIO0` reader - Priority of Request Source x"]
pub struct PRIO0_R(crate::FieldReader<u8, PRIO0_A>);
impl PRIO0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRIO0_A> {
        match self.bits {
            0 => Some(PRIO0_A::VALUE1),
            3 => Some(PRIO0_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRIO0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRIO0_A::VALUE2
    }
}
impl core::ops::Deref for PRIO0_R {
    type Target = crate::FieldReader<u8, PRIO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIO0` writer - Priority of Request Source x"]
pub struct PRIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIO0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIO0_A::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIO0_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRIO1_A {
    #[doc = "0: Lowest priority is selected."]
    VALUE1 = 0,
    #[doc = "3: Highest priority is selected."]
    VALUE2 = 3,
}
impl From<PRIO1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRIO1` reader - Priority of Request Source x"]
pub struct PRIO1_R(crate::FieldReader<u8, PRIO1_A>);
impl PRIO1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRIO1_A> {
        match self.bits {
            0 => Some(PRIO1_A::VALUE1),
            3 => Some(PRIO1_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRIO1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRIO1_A::VALUE2
    }
}
impl core::ops::Deref for PRIO1_R {
    type Target = crate::FieldReader<u8, PRIO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIO1` writer - Priority of Request Source x"]
pub struct PRIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIO1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIO1_A::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIO1_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRIO2_A {
    #[doc = "0: Lowest priority is selected."]
    VALUE1 = 0,
    #[doc = "3: Highest priority is selected."]
    VALUE2 = 3,
}
impl From<PRIO2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRIO2` reader - Priority of Request Source x"]
pub struct PRIO2_R(crate::FieldReader<u8, PRIO2_A>);
impl PRIO2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIO2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRIO2_A> {
        match self.bits {
            0 => Some(PRIO2_A::VALUE1),
            3 => Some(PRIO2_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRIO2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRIO2_A::VALUE2
    }
}
impl core::ops::Deref for PRIO2_R {
    type Target = crate::FieldReader<u8, PRIO2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIO2` writer - Priority of Request Source x"]
pub struct PRIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIO2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIO2_A::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIO2_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSM0_A {
    #[doc = "0: Wait-for-start mode"]
    VALUE1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2 = 1,
}
impl From<CSM0_A> for bool {
    #[inline(always)]
    fn from(variant: CSM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSM0` reader - Conversion Start Mode of Request Source x"]
pub struct CSM0_R(crate::FieldReader<bool, CSM0_A>);
impl CSM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSM0_A {
        match self.bits {
            false => CSM0_A::VALUE1,
            true => CSM0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CSM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CSM0_A::VALUE2
    }
}
impl core::ops::Deref for CSM0_R {
    type Target = crate::FieldReader<bool, CSM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSM0` writer - Conversion Start Mode of Request Source x"]
pub struct CSM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSM0_A::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSM0_A::VALUE2)
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
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSM1_A {
    #[doc = "0: Wait-for-start mode"]
    VALUE1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2 = 1,
}
impl From<CSM1_A> for bool {
    #[inline(always)]
    fn from(variant: CSM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSM1` reader - Conversion Start Mode of Request Source x"]
pub struct CSM1_R(crate::FieldReader<bool, CSM1_A>);
impl CSM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSM1_A {
        match self.bits {
            false => CSM1_A::VALUE1,
            true => CSM1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CSM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CSM1_A::VALUE2
    }
}
impl core::ops::Deref for CSM1_R {
    type Target = crate::FieldReader<bool, CSM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSM1` writer - Conversion Start Mode of Request Source x"]
pub struct CSM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSM1_A::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSM1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSM2_A {
    #[doc = "0: Wait-for-start mode"]
    VALUE1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2 = 1,
}
impl From<CSM2_A> for bool {
    #[inline(always)]
    fn from(variant: CSM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSM2` reader - Conversion Start Mode of Request Source x"]
pub struct CSM2_R(crate::FieldReader<bool, CSM2_A>);
impl CSM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSM2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSM2_A {
        match self.bits {
            false => CSM2_A::VALUE1,
            true => CSM2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CSM2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CSM2_A::VALUE2
    }
}
impl core::ops::Deref for CSM2_R {
    type Target = crate::FieldReader<bool, CSM2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSM2` writer - Conversion Start Mode of Request Source x"]
pub struct CSM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSM2_A::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSM2_A::VALUE2)
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
#[doc = "Arbitration Slot 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASEN0_A {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2 = 1,
}
impl From<ASEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEN0` reader - Arbitration Slot 0 Enable"]
pub struct ASEN0_R(crate::FieldReader<bool, ASEN0_A>);
impl ASEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASEN0_A {
        match self.bits {
            false => ASEN0_A::VALUE1,
            true => ASEN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASEN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASEN0_A::VALUE2
    }
}
impl core::ops::Deref for ASEN0_R {
    type Target = crate::FieldReader<bool, ASEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASEN0` writer - Arbitration Slot 0 Enable"]
pub struct ASEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASEN0_A::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASEN0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Arbitration Slot 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASEN1_A {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2 = 1,
}
impl From<ASEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEN1` reader - Arbitration Slot 1 Enable"]
pub struct ASEN1_R(crate::FieldReader<bool, ASEN1_A>);
impl ASEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASEN1_A {
        match self.bits {
            false => ASEN1_A::VALUE1,
            true => ASEN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASEN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASEN1_A::VALUE2
    }
}
impl core::ops::Deref for ASEN1_R {
    type Target = crate::FieldReader<bool, ASEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASEN1` writer - Arbitration Slot 1 Enable"]
pub struct ASEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASEN1_A::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASEN1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Arbitration Slot 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASEN2_A {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2 = 1,
}
impl From<ASEN2_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEN2` reader - Arbitration Slot 2 Enable"]
pub struct ASEN2_R(crate::FieldReader<bool, ASEN2_A>);
impl ASEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASEN2_A {
        match self.bits {
            false => ASEN2_A::VALUE1,
            true => ASEN2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASEN2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASEN2_A::VALUE2
    }
}
impl core::ops::Deref for ASEN2_R {
    type Target = crate::FieldReader<bool, ASEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASEN2` writer - Arbitration Slot 2 Enable"]
pub struct ASEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASEN2_A::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASEN2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio0(&self) -> PRIO0_R {
        PRIO0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio1(&self) -> PRIO1_R {
        PRIO1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio2(&self) -> PRIO2_R {
        PRIO2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm0(&self) -> CSM0_R {
        CSM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm1(&self) -> CSM1_R {
        CSM1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm2(&self) -> CSM2_R {
        CSM2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline(always)]
    pub fn asen0(&self) -> ASEN0_R {
        ASEN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline(always)]
    pub fn asen1(&self) -> ASEN1_R {
        ASEN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline(always)]
    pub fn asen2(&self) -> ASEN2_R {
        ASEN2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio0(&mut self) -> PRIO0_W {
        PRIO0_W { w: self }
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio1(&mut self) -> PRIO1_W {
        PRIO1_W { w: self }
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio2(&mut self) -> PRIO2_W {
        PRIO2_W { w: self }
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm0(&mut self) -> CSM0_W {
        CSM0_W { w: self }
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm1(&mut self) -> CSM1_W {
        CSM1_W { w: self }
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm2(&mut self) -> CSM2_W {
        CSM2_W { w: self }
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline(always)]
    pub fn asen0(&mut self) -> ASEN0_W {
        ASEN0_W { w: self }
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline(always)]
    pub fn asen1(&mut self) -> ASEN1_W {
        ASEN1_W { w: self }
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline(always)]
    pub fn asen2(&mut self) -> ASEN2_W {
        ASEN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arbitration Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arbpr](index.html) module"]
pub struct ARBPR_SPEC;
impl crate::RegisterSpec for ARBPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arbpr::R](R) reader structure"]
impl crate::Readable for ARBPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arbpr::W](W) writer structure"]
impl crate::Writable for ARBPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARBPR to value 0"]
impl crate::Resettable for ARBPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
