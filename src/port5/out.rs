#[doc = "Register `OUT` reader"]
pub struct R(crate::R<OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT` writer"]
pub struct W(crate::W<OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SPEC>;
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
impl From<crate::W<OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port n Output Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P0_A> for bool {
    #[inline(always)]
    fn from(variant: P0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0` reader - Port n Output Bit 0"]
pub struct P0_R(crate::FieldReader<bool, P0_A>);
impl P0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_A {
        match self.bits {
            false => P0_A::CONST_0,
            true => P0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P0_A::CONST_1
    }
}
impl core::ops::Deref for P0_R {
    type Target = crate::FieldReader<bool, P0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0` writer - Port n Output Bit 0"]
pub struct P0_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P0_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P0_A::CONST_1)
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
#[doc = "Port n Output Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P1_A> for bool {
    #[inline(always)]
    fn from(variant: P1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1` reader - Port n Output Bit 1"]
pub struct P1_R(crate::FieldReader<bool, P1_A>);
impl P1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_A {
        match self.bits {
            false => P1_A::CONST_0,
            true => P1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P1_A::CONST_1
    }
}
impl core::ops::Deref for P1_R {
    type Target = crate::FieldReader<bool, P1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1` writer - Port n Output Bit 1"]
pub struct P1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P1_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P1_A::CONST_1)
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
#[doc = "Port n Output Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P2_A> for bool {
    #[inline(always)]
    fn from(variant: P2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2` reader - Port n Output Bit 2"]
pub struct P2_R(crate::FieldReader<bool, P2_A>);
impl P2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_A {
        match self.bits {
            false => P2_A::CONST_0,
            true => P2_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P2_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P2_A::CONST_1
    }
}
impl core::ops::Deref for P2_R {
    type Target = crate::FieldReader<bool, P2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2` writer - Port n Output Bit 2"]
pub struct P2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P2_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P2_A::CONST_1)
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
#[doc = "Port n Output Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P3_A> for bool {
    #[inline(always)]
    fn from(variant: P3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3` reader - Port n Output Bit 3"]
pub struct P3_R(crate::FieldReader<bool, P3_A>);
impl P3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_A {
        match self.bits {
            false => P3_A::CONST_0,
            true => P3_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P3_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P3_A::CONST_1
    }
}
impl core::ops::Deref for P3_R {
    type Target = crate::FieldReader<bool, P3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3` writer - Port n Output Bit 3"]
pub struct P3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P3_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P3_A::CONST_1)
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
#[doc = "Port n Output Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P4_A> for bool {
    #[inline(always)]
    fn from(variant: P4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P4` reader - Port n Output Bit 4"]
pub struct P4_R(crate::FieldReader<bool, P4_A>);
impl P4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_A {
        match self.bits {
            false => P4_A::CONST_0,
            true => P4_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P4_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P4_A::CONST_1
    }
}
impl core::ops::Deref for P4_R {
    type Target = crate::FieldReader<bool, P4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4` writer - Port n Output Bit 4"]
pub struct P4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P4_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P4_A::CONST_1)
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
#[doc = "Port n Output Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P5_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P5_A> for bool {
    #[inline(always)]
    fn from(variant: P5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P5` reader - Port n Output Bit 5"]
pub struct P5_R(crate::FieldReader<bool, P5_A>);
impl P5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P5_A {
        match self.bits {
            false => P5_A::CONST_0,
            true => P5_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P5_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P5_A::CONST_1
    }
}
impl core::ops::Deref for P5_R {
    type Target = crate::FieldReader<bool, P5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5` writer - Port n Output Bit 5"]
pub struct P5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P5_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P5_A::CONST_1)
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
#[doc = "Port n Output Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P6_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P6_A> for bool {
    #[inline(always)]
    fn from(variant: P6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P6` reader - Port n Output Bit 6"]
pub struct P6_R(crate::FieldReader<bool, P6_A>);
impl P6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P6_A {
        match self.bits {
            false => P6_A::CONST_0,
            true => P6_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P6_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P6_A::CONST_1
    }
}
impl core::ops::Deref for P6_R {
    type Target = crate::FieldReader<bool, P6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6` writer - Port n Output Bit 6"]
pub struct P6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P6_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P6_A::CONST_1)
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
#[doc = "Port n Output Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P7_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P7_A> for bool {
    #[inline(always)]
    fn from(variant: P7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P7` reader - Port n Output Bit 7"]
pub struct P7_R(crate::FieldReader<bool, P7_A>);
impl P7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P7_A {
        match self.bits {
            false => P7_A::CONST_0,
            true => P7_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P7_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P7_A::CONST_1
    }
}
impl core::ops::Deref for P7_R {
    type Target = crate::FieldReader<bool, P7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7` writer - Port n Output Bit 7"]
pub struct P7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P7_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P7_A::CONST_1)
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
#[doc = "Port n Output Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P8_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P8_A> for bool {
    #[inline(always)]
    fn from(variant: P8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P8` reader - Port n Output Bit 8"]
pub struct P8_R(crate::FieldReader<bool, P8_A>);
impl P8_R {
    pub(crate) fn new(bits: bool) -> Self {
        P8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P8_A {
        match self.bits {
            false => P8_A::CONST_0,
            true => P8_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P8_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P8_A::CONST_1
    }
}
impl core::ops::Deref for P8_R {
    type Target = crate::FieldReader<bool, P8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8` writer - Port n Output Bit 8"]
pub struct P8_W<'a> {
    w: &'a mut W,
}
impl<'a> P8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P8_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P8_A::CONST_1)
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
#[doc = "Port n Output Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P9_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P9_A> for bool {
    #[inline(always)]
    fn from(variant: P9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P9` reader - Port n Output Bit 9"]
pub struct P9_R(crate::FieldReader<bool, P9_A>);
impl P9_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P9_A {
        match self.bits {
            false => P9_A::CONST_0,
            true => P9_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P9_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P9_A::CONST_1
    }
}
impl core::ops::Deref for P9_R {
    type Target = crate::FieldReader<bool, P9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9` writer - Port n Output Bit 9"]
pub struct P9_W<'a> {
    w: &'a mut W,
}
impl<'a> P9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P9_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P9_A::CONST_1)
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
#[doc = "Port n Output Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P10_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P10_A> for bool {
    #[inline(always)]
    fn from(variant: P10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P10` reader - Port n Output Bit 10"]
pub struct P10_R(crate::FieldReader<bool, P10_A>);
impl P10_R {
    pub(crate) fn new(bits: bool) -> Self {
        P10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P10_A {
        match self.bits {
            false => P10_A::CONST_0,
            true => P10_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P10_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P10_A::CONST_1
    }
}
impl core::ops::Deref for P10_R {
    type Target = crate::FieldReader<bool, P10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10` writer - Port n Output Bit 10"]
pub struct P10_W<'a> {
    w: &'a mut W,
}
impl<'a> P10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P10_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P10_A::CONST_1)
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
#[doc = "Port n Output Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P11_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P11_A> for bool {
    #[inline(always)]
    fn from(variant: P11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P11` reader - Port n Output Bit 11"]
pub struct P11_R(crate::FieldReader<bool, P11_A>);
impl P11_R {
    pub(crate) fn new(bits: bool) -> Self {
        P11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P11_A {
        match self.bits {
            false => P11_A::CONST_0,
            true => P11_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P11_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P11_A::CONST_1
    }
}
impl core::ops::Deref for P11_R {
    type Target = crate::FieldReader<bool, P11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P11` writer - Port n Output Bit 11"]
pub struct P11_W<'a> {
    w: &'a mut W,
}
impl<'a> P11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P11_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P11_A::CONST_1)
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
#[doc = "Port n Output Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P12_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P12_A> for bool {
    #[inline(always)]
    fn from(variant: P12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P12` reader - Port n Output Bit 12"]
pub struct P12_R(crate::FieldReader<bool, P12_A>);
impl P12_R {
    pub(crate) fn new(bits: bool) -> Self {
        P12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P12_A {
        match self.bits {
            false => P12_A::CONST_0,
            true => P12_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P12_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P12_A::CONST_1
    }
}
impl core::ops::Deref for P12_R {
    type Target = crate::FieldReader<bool, P12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P12` writer - Port n Output Bit 12"]
pub struct P12_W<'a> {
    w: &'a mut W,
}
impl<'a> P12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P12_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P12_A::CONST_1)
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
#[doc = "Port n Output Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P13_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P13_A> for bool {
    #[inline(always)]
    fn from(variant: P13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P13` reader - Port n Output Bit 13"]
pub struct P13_R(crate::FieldReader<bool, P13_A>);
impl P13_R {
    pub(crate) fn new(bits: bool) -> Self {
        P13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P13_A {
        match self.bits {
            false => P13_A::CONST_0,
            true => P13_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P13_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P13_A::CONST_1
    }
}
impl core::ops::Deref for P13_R {
    type Target = crate::FieldReader<bool, P13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P13` writer - Port n Output Bit 13"]
pub struct P13_W<'a> {
    w: &'a mut W,
}
impl<'a> P13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P13_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P13_A::CONST_1)
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
#[doc = "Port n Output Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P14_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P14_A> for bool {
    #[inline(always)]
    fn from(variant: P14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P14` reader - Port n Output Bit 14"]
pub struct P14_R(crate::FieldReader<bool, P14_A>);
impl P14_R {
    pub(crate) fn new(bits: bool) -> Self {
        P14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P14_A {
        match self.bits {
            false => P14_A::CONST_0,
            true => P14_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P14_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P14_A::CONST_1
    }
}
impl core::ops::Deref for P14_R {
    type Target = crate::FieldReader<bool, P14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P14` writer - Port n Output Bit 14"]
pub struct P14_W<'a> {
    w: &'a mut W,
}
impl<'a> P14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P14_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P14_A::CONST_1)
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
#[doc = "Port n Output Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P15_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P15_A> for bool {
    #[inline(always)]
    fn from(variant: P15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P15` reader - Port n Output Bit 15"]
pub struct P15_R(crate::FieldReader<bool, P15_A>);
impl P15_R {
    pub(crate) fn new(bits: bool) -> Self {
        P15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P15_A {
        match self.bits {
            false => P15_A::CONST_0,
            true => P15_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == P15_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == P15_A::CONST_1
    }
}
impl core::ops::Deref for P15_R {
    type Target = crate::FieldReader<bool, P15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P15` writer - Port n Output Bit 15"]
pub struct P15_W<'a> {
    w: &'a mut W,
}
impl<'a> P15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(P15_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(P15_A::CONST_1)
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
impl R {
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0_W {
        P0_W { w: self }
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1_W {
        P1_W { w: self }
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2_W {
        P2_W { w: self }
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3_W {
        P3_W { w: self }
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4_W {
        P4_W { w: self }
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5_W {
        P5_W { w: self }
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6_W {
        P6_W { w: self }
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline(always)]
    pub fn p7(&mut self) -> P7_W {
        P7_W { w: self }
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline(always)]
    pub fn p8(&mut self) -> P8_W {
        P8_W { w: self }
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline(always)]
    pub fn p9(&mut self) -> P9_W {
        P9_W { w: self }
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline(always)]
    pub fn p10(&mut self) -> P10_W {
        P10_W { w: self }
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline(always)]
    pub fn p11(&mut self) -> P11_W {
        P11_W { w: self }
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline(always)]
    pub fn p12(&mut self) -> P12_W {
        P12_W { w: self }
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline(always)]
    pub fn p13(&mut self) -> P13_W {
        P13_W { w: self }
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline(always)]
    pub fn p14(&mut self) -> P14_W {
        P14_W { w: self }
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline(always)]
    pub fn p15(&mut self) -> P15_W {
        P15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](index.html) module"]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out::R](R) reader structure"]
impl crate::Readable for OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out::W](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
