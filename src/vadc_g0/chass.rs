#[doc = "Register `CHASS` reader"]
pub struct R(crate::R<CHASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHASS` writer"]
pub struct W(crate::W<CHASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHASS_SPEC>;
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
impl From<crate::W<CHASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Assignment for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH0_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH0_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH0` reader - Assignment for Channel 0"]
pub struct ASSCH0_R(crate::FieldReader<bool, ASSCH0_A>);
impl ASSCH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH0_A {
        match self.bits {
            false => ASSCH0_A::VALUE1,
            true => ASSCH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH0_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH0_R {
    type Target = crate::FieldReader<bool, ASSCH0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH0` writer - Assignment for Channel 0"]
pub struct ASSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH0_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Assignment for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH1_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH1_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH1` reader - Assignment for Channel 1"]
pub struct ASSCH1_R(crate::FieldReader<bool, ASSCH1_A>);
impl ASSCH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH1_A {
        match self.bits {
            false => ASSCH1_A::VALUE1,
            true => ASSCH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH1_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH1_R {
    type Target = crate::FieldReader<bool, ASSCH1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH1` writer - Assignment for Channel 1"]
pub struct ASSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH1_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Assignment for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH2_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH2_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH2` reader - Assignment for Channel 2"]
pub struct ASSCH2_R(crate::FieldReader<bool, ASSCH2_A>);
impl ASSCH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH2_A {
        match self.bits {
            false => ASSCH2_A::VALUE1,
            true => ASSCH2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH2_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH2_R {
    type Target = crate::FieldReader<bool, ASSCH2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH2` writer - Assignment for Channel 2"]
pub struct ASSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH2_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH2_A::VALUE2)
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
#[doc = "Assignment for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH3_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH3_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH3` reader - Assignment for Channel 3"]
pub struct ASSCH3_R(crate::FieldReader<bool, ASSCH3_A>);
impl ASSCH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH3_A {
        match self.bits {
            false => ASSCH3_A::VALUE1,
            true => ASSCH3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH3_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH3_R {
    type Target = crate::FieldReader<bool, ASSCH3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH3` writer - Assignment for Channel 3"]
pub struct ASSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH3_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH3_A::VALUE2)
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
#[doc = "Assignment for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH4_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH4_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH4` reader - Assignment for Channel 4"]
pub struct ASSCH4_R(crate::FieldReader<bool, ASSCH4_A>);
impl ASSCH4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH4_A {
        match self.bits {
            false => ASSCH4_A::VALUE1,
            true => ASSCH4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH4_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH4_R {
    type Target = crate::FieldReader<bool, ASSCH4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH4` writer - Assignment for Channel 4"]
pub struct ASSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH4_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH4_A::VALUE2)
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
#[doc = "Assignment for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH5_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH5_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH5` reader - Assignment for Channel 5"]
pub struct ASSCH5_R(crate::FieldReader<bool, ASSCH5_A>);
impl ASSCH5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH5_A {
        match self.bits {
            false => ASSCH5_A::VALUE1,
            true => ASSCH5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH5_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH5_R {
    type Target = crate::FieldReader<bool, ASSCH5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH5` writer - Assignment for Channel 5"]
pub struct ASSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH5_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH5_A::VALUE2)
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
#[doc = "Assignment for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH6_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH6_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH6` reader - Assignment for Channel 6"]
pub struct ASSCH6_R(crate::FieldReader<bool, ASSCH6_A>);
impl ASSCH6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH6_A {
        match self.bits {
            false => ASSCH6_A::VALUE1,
            true => ASSCH6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH6_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH6_R {
    type Target = crate::FieldReader<bool, ASSCH6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH6` writer - Assignment for Channel 6"]
pub struct ASSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH6_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH6_A::VALUE2)
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
#[doc = "Assignment for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSCH7_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH7_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH7` reader - Assignment for Channel 7"]
pub struct ASSCH7_R(crate::FieldReader<bool, ASSCH7_A>);
impl ASSCH7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASSCH7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH7_A {
        match self.bits {
            false => ASSCH7_A::VALUE1,
            true => ASSCH7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASSCH7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ASSCH7_A::VALUE2
    }
}
impl core::ops::Deref for ASSCH7_R {
    type Target = crate::FieldReader<bool, ASSCH7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASSCH7` writer - Assignment for Channel 7"]
pub struct ASSCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSCH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSCH7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH7_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH7_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    pub fn assch0(&self) -> ASSCH0_R {
        ASSCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    pub fn assch1(&self) -> ASSCH1_R {
        ASSCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    pub fn assch2(&self) -> ASSCH2_R {
        ASSCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    pub fn assch3(&self) -> ASSCH3_R {
        ASSCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    pub fn assch4(&self) -> ASSCH4_R {
        ASSCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    pub fn assch5(&self) -> ASSCH5_R {
        ASSCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    pub fn assch6(&self) -> ASSCH6_R {
        ASSCH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    pub fn assch7(&self) -> ASSCH7_R {
        ASSCH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    pub fn assch0(&mut self) -> ASSCH0_W {
        ASSCH0_W { w: self }
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    pub fn assch1(&mut self) -> ASSCH1_W {
        ASSCH1_W { w: self }
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    pub fn assch2(&mut self) -> ASSCH2_W {
        ASSCH2_W { w: self }
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    pub fn assch3(&mut self) -> ASSCH3_W {
        ASSCH3_W { w: self }
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    pub fn assch4(&mut self) -> ASSCH4_W {
        ASSCH4_W { w: self }
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    pub fn assch5(&mut self) -> ASSCH5_W {
        ASSCH5_W { w: self }
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    pub fn assch6(&mut self) -> ASSCH6_W {
        ASSCH6_W { w: self }
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    pub fn assch7(&mut self) -> ASSCH7_W {
        ASSCH7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Assignment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chass](index.html) module"]
pub struct CHASS_SPEC;
impl crate::RegisterSpec for CHASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chass::R](R) reader structure"]
impl crate::Readable for CHASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chass::W](W) writer structure"]
impl crate::Writable for CHASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHASS to value 0"]
impl crate::Resettable for CHASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
