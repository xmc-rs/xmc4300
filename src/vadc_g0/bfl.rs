#[doc = "Register `BFL` reader"]
pub struct R(crate::R<BFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFL` writer"]
pub struct W(crate::W<BFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFL_SPEC>;
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
impl From<crate::W<BFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Boundary Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL0_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL0_A> for bool {
    #[inline(always)]
    fn from(variant: BFL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL0` reader - Boundary Flag 0"]
pub struct BFL0_R(crate::FieldReader<bool, BFL0_A>);
impl BFL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL0_A {
        match self.bits {
            false => BFL0_A::VALUE1,
            true => BFL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL0_A::VALUE2
    }
}
impl core::ops::Deref for BFL0_R {
    type Target = crate::FieldReader<bool, BFL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boundary Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL1_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL1_A> for bool {
    #[inline(always)]
    fn from(variant: BFL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL1` reader - Boundary Flag 1"]
pub struct BFL1_R(crate::FieldReader<bool, BFL1_A>);
impl BFL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL1_A {
        match self.bits {
            false => BFL1_A::VALUE1,
            true => BFL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL1_A::VALUE2
    }
}
impl core::ops::Deref for BFL1_R {
    type Target = crate::FieldReader<bool, BFL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boundary Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL2_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL2_A> for bool {
    #[inline(always)]
    fn from(variant: BFL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL2` reader - Boundary Flag 2"]
pub struct BFL2_R(crate::FieldReader<bool, BFL2_A>);
impl BFL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL2_A {
        match self.bits {
            false => BFL2_A::VALUE1,
            true => BFL2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL2_A::VALUE2
    }
}
impl core::ops::Deref for BFL2_R {
    type Target = crate::FieldReader<bool, BFL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boundary Flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL3_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary (see bitfield BFAy), or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL3_A> for bool {
    #[inline(always)]
    fn from(variant: BFL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL3` reader - Boundary Flag 3"]
pub struct BFL3_R(crate::FieldReader<bool, BFL3_A>);
impl BFL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL3_A {
        match self.bits {
            false => BFL3_A::VALUE1,
            true => BFL3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL3_A::VALUE2
    }
}
impl core::ops::Deref for BFL3_R {
    type Target = crate::FieldReader<bool, BFL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boundary Flag 0 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA0_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA0_A> for bool {
    #[inline(always)]
    fn from(variant: BFA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA0` reader - Boundary Flag 0 Activation Select"]
pub struct BFA0_R(crate::FieldReader<bool, BFA0_A>);
impl BFA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFA0_A {
        match self.bits {
            false => BFA0_A::VALUE1,
            true => BFA0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFA0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFA0_A::VALUE2
    }
}
impl core::ops::Deref for BFA0_R {
    type Target = crate::FieldReader<bool, BFA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFA0` writer - Boundary Flag 0 Activation Select"]
pub struct BFA0_W<'a> {
    w: &'a mut W,
}
impl<'a> BFA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA0_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA0_A::VALUE2)
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
#[doc = "Boundary Flag 1 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA1_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA1_A> for bool {
    #[inline(always)]
    fn from(variant: BFA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA1` reader - Boundary Flag 1 Activation Select"]
pub struct BFA1_R(crate::FieldReader<bool, BFA1_A>);
impl BFA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFA1_A {
        match self.bits {
            false => BFA1_A::VALUE1,
            true => BFA1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFA1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFA1_A::VALUE2
    }
}
impl core::ops::Deref for BFA1_R {
    type Target = crate::FieldReader<bool, BFA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFA1` writer - Boundary Flag 1 Activation Select"]
pub struct BFA1_W<'a> {
    w: &'a mut W,
}
impl<'a> BFA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA1_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA1_A::VALUE2)
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
#[doc = "Boundary Flag 2 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA2_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA2_A> for bool {
    #[inline(always)]
    fn from(variant: BFA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA2` reader - Boundary Flag 2 Activation Select"]
pub struct BFA2_R(crate::FieldReader<bool, BFA2_A>);
impl BFA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFA2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFA2_A {
        match self.bits {
            false => BFA2_A::VALUE1,
            true => BFA2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFA2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFA2_A::VALUE2
    }
}
impl core::ops::Deref for BFA2_R {
    type Target = crate::FieldReader<bool, BFA2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFA2` writer - Boundary Flag 2 Activation Select"]
pub struct BFA2_W<'a> {
    w: &'a mut W,
}
impl<'a> BFA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFA2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA2_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA2_A::VALUE2)
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
#[doc = "Boundary Flag 3 Activation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFA3_A {
    #[doc = "0: Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    VALUE1 = 0,
    #[doc = "1: Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    VALUE2 = 1,
}
impl From<BFA3_A> for bool {
    #[inline(always)]
    fn from(variant: BFA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFA3` reader - Boundary Flag 3 Activation Select"]
pub struct BFA3_R(crate::FieldReader<bool, BFA3_A>);
impl BFA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFA3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFA3_A {
        match self.bits {
            false => BFA3_A::VALUE1,
            true => BFA3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFA3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFA3_A::VALUE2
    }
}
impl core::ops::Deref for BFA3_R {
    type Target = crate::FieldReader<bool, BFA3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFA3` writer - Boundary Flag 3 Activation Select"]
pub struct BFA3_W<'a> {
    w: &'a mut W,
}
impl<'a> BFA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFA3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set boundary flag BFLy if result is above the defined band or compare value, clear if below"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFA3_A::VALUE1)
    }
    #[doc = "Set boundary flag BFLy if result is below the defined band or compare value, clear if above"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFA3_A::VALUE2)
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
#[doc = "Boundary Flag 0 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI0_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI0_A> for bool {
    #[inline(always)]
    fn from(variant: BFI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI0` reader - Boundary Flag 0 Inversion Control"]
pub struct BFI0_R(crate::FieldReader<bool, BFI0_A>);
impl BFI0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFI0_A {
        match self.bits {
            false => BFI0_A::VALUE1,
            true => BFI0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFI0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFI0_A::VALUE2
    }
}
impl core::ops::Deref for BFI0_R {
    type Target = crate::FieldReader<bool, BFI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFI0` writer - Boundary Flag 0 Inversion Control"]
pub struct BFI0_W<'a> {
    w: &'a mut W,
}
impl<'a> BFI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI0_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI0_A::VALUE2)
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
#[doc = "Boundary Flag 1 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI1_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI1_A> for bool {
    #[inline(always)]
    fn from(variant: BFI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI1` reader - Boundary Flag 1 Inversion Control"]
pub struct BFI1_R(crate::FieldReader<bool, BFI1_A>);
impl BFI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFI1_A {
        match self.bits {
            false => BFI1_A::VALUE1,
            true => BFI1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFI1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFI1_A::VALUE2
    }
}
impl core::ops::Deref for BFI1_R {
    type Target = crate::FieldReader<bool, BFI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFI1` writer - Boundary Flag 1 Inversion Control"]
pub struct BFI1_W<'a> {
    w: &'a mut W,
}
impl<'a> BFI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI1_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI1_A::VALUE2)
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
#[doc = "Boundary Flag 2 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI2_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI2_A> for bool {
    #[inline(always)]
    fn from(variant: BFI2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI2` reader - Boundary Flag 2 Inversion Control"]
pub struct BFI2_R(crate::FieldReader<bool, BFI2_A>);
impl BFI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFI2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFI2_A {
        match self.bits {
            false => BFI2_A::VALUE1,
            true => BFI2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFI2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFI2_A::VALUE2
    }
}
impl core::ops::Deref for BFI2_R {
    type Target = crate::FieldReader<bool, BFI2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFI2` writer - Boundary Flag 2 Inversion Control"]
pub struct BFI2_W<'a> {
    w: &'a mut W,
}
impl<'a> BFI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFI2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI2_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI2_A::VALUE2)
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
#[doc = "Boundary Flag 3 Inversion Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFI3_A {
    #[doc = "0: Use BFLy directly"]
    VALUE1 = 0,
    #[doc = "1: Invert value and use BFLy"]
    VALUE2 = 1,
}
impl From<BFI3_A> for bool {
    #[inline(always)]
    fn from(variant: BFI3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFI3` reader - Boundary Flag 3 Inversion Control"]
pub struct BFI3_R(crate::FieldReader<bool, BFI3_A>);
impl BFI3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFI3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFI3_A {
        match self.bits {
            false => BFI3_A::VALUE1,
            true => BFI3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFI3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFI3_A::VALUE2
    }
}
impl core::ops::Deref for BFI3_R {
    type Target = crate::FieldReader<bool, BFI3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFI3` writer - Boundary Flag 3 Inversion Control"]
pub struct BFI3_W<'a> {
    w: &'a mut W,
}
impl<'a> BFI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFI3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use BFLy directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFI3_A::VALUE1)
    }
    #[doc = "Invert value and use BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFI3_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Boundary Flag 0"]
    #[inline(always)]
    pub fn bfl0(&self) -> BFL0_R {
        BFL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Boundary Flag 1"]
    #[inline(always)]
    pub fn bfl1(&self) -> BFL1_R {
        BFL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Boundary Flag 2"]
    #[inline(always)]
    pub fn bfl2(&self) -> BFL2_R {
        BFL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Boundary Flag 3"]
    #[inline(always)]
    pub fn bfl3(&self) -> BFL3_R {
        BFL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline(always)]
    pub fn bfa0(&self) -> BFA0_R {
        BFA0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline(always)]
    pub fn bfa1(&self) -> BFA1_R {
        BFA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline(always)]
    pub fn bfa2(&self) -> BFA2_R {
        BFA2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline(always)]
    pub fn bfa3(&self) -> BFA3_R {
        BFA3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline(always)]
    pub fn bfi0(&self) -> BFI0_R {
        BFI0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline(always)]
    pub fn bfi1(&self) -> BFI1_R {
        BFI1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline(always)]
    pub fn bfi2(&self) -> BFI2_R {
        BFI2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline(always)]
    pub fn bfi3(&self) -> BFI3_R {
        BFI3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Boundary Flag 0 Activation Select"]
    #[inline(always)]
    pub fn bfa0(&mut self) -> BFA0_W {
        BFA0_W { w: self }
    }
    #[doc = "Bit 9 - Boundary Flag 1 Activation Select"]
    #[inline(always)]
    pub fn bfa1(&mut self) -> BFA1_W {
        BFA1_W { w: self }
    }
    #[doc = "Bit 10 - Boundary Flag 2 Activation Select"]
    #[inline(always)]
    pub fn bfa2(&mut self) -> BFA2_W {
        BFA2_W { w: self }
    }
    #[doc = "Bit 11 - Boundary Flag 3 Activation Select"]
    #[inline(always)]
    pub fn bfa3(&mut self) -> BFA3_W {
        BFA3_W { w: self }
    }
    #[doc = "Bit 16 - Boundary Flag 0 Inversion Control"]
    #[inline(always)]
    pub fn bfi0(&mut self) -> BFI0_W {
        BFI0_W { w: self }
    }
    #[doc = "Bit 17 - Boundary Flag 1 Inversion Control"]
    #[inline(always)]
    pub fn bfi1(&mut self) -> BFI1_W {
        BFI1_W { w: self }
    }
    #[doc = "Bit 18 - Boundary Flag 2 Inversion Control"]
    #[inline(always)]
    pub fn bfi2(&mut self) -> BFI2_W {
        BFI2_W { w: self }
    }
    #[doc = "Bit 19 - Boundary Flag 3 Inversion Control"]
    #[inline(always)]
    pub fn bfi3(&mut self) -> BFI3_W {
        BFI3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boundary Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfl](index.html) module"]
pub struct BFL_SPEC;
impl crate::RegisterSpec for BFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfl::R](R) reader structure"]
impl crate::Readable for BFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfl::W](W) writer structure"]
impl crate::Writable for BFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BFL to value 0"]
impl crate::Resettable for BFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
