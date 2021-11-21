#[doc = "Register `PPS` reader"]
pub struct R(crate::R<PPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS` writer"]
pub struct W(crate::W<PPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_SPEC>;
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
impl From<crate::W<PPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port n Pin Power Save Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS0_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS0_A> for bool {
    #[inline(always)]
    fn from(variant: PPS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS0` reader - Port n Pin Power Save Bit 0"]
pub struct PPS0_R(crate::FieldReader<bool, PPS0_A>);
impl PPS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS0_A {
        match self.bits {
            false => PPS0_A::CONST_0,
            true => PPS0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS0_A::CONST_1
    }
}
impl core::ops::Deref for PPS0_R {
    type Target = crate::FieldReader<bool, PPS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS0` writer - Port n Pin Power Save Bit 0"]
pub struct PPS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS0_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS0_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS1_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS1_A> for bool {
    #[inline(always)]
    fn from(variant: PPS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS1` reader - Port n Pin Power Save Bit 1"]
pub struct PPS1_R(crate::FieldReader<bool, PPS1_A>);
impl PPS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS1_A {
        match self.bits {
            false => PPS1_A::CONST_0,
            true => PPS1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS1_A::CONST_1
    }
}
impl core::ops::Deref for PPS1_R {
    type Target = crate::FieldReader<bool, PPS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS1` writer - Port n Pin Power Save Bit 1"]
pub struct PPS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS1_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS1_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS2_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS2_A> for bool {
    #[inline(always)]
    fn from(variant: PPS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS2` reader - Port n Pin Power Save Bit 2"]
pub struct PPS2_R(crate::FieldReader<bool, PPS2_A>);
impl PPS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS2_A {
        match self.bits {
            false => PPS2_A::CONST_0,
            true => PPS2_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS2_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS2_A::CONST_1
    }
}
impl core::ops::Deref for PPS2_R {
    type Target = crate::FieldReader<bool, PPS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS2` writer - Port n Pin Power Save Bit 2"]
pub struct PPS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS2_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS2_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS3_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS3_A> for bool {
    #[inline(always)]
    fn from(variant: PPS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS3` reader - Port n Pin Power Save Bit 3"]
pub struct PPS3_R(crate::FieldReader<bool, PPS3_A>);
impl PPS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS3_A {
        match self.bits {
            false => PPS3_A::CONST_0,
            true => PPS3_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS3_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS3_A::CONST_1
    }
}
impl core::ops::Deref for PPS3_R {
    type Target = crate::FieldReader<bool, PPS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS3` writer - Port n Pin Power Save Bit 3"]
pub struct PPS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS3_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS3_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS4_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS4_A> for bool {
    #[inline(always)]
    fn from(variant: PPS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS4` reader - Port n Pin Power Save Bit 4"]
pub struct PPS4_R(crate::FieldReader<bool, PPS4_A>);
impl PPS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS4_A {
        match self.bits {
            false => PPS4_A::CONST_0,
            true => PPS4_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS4_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS4_A::CONST_1
    }
}
impl core::ops::Deref for PPS4_R {
    type Target = crate::FieldReader<bool, PPS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS4` writer - Port n Pin Power Save Bit 4"]
pub struct PPS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS4_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS4_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS5_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS5_A> for bool {
    #[inline(always)]
    fn from(variant: PPS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS5` reader - Port n Pin Power Save Bit 5"]
pub struct PPS5_R(crate::FieldReader<bool, PPS5_A>);
impl PPS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS5_A {
        match self.bits {
            false => PPS5_A::CONST_0,
            true => PPS5_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS5_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS5_A::CONST_1
    }
}
impl core::ops::Deref for PPS5_R {
    type Target = crate::FieldReader<bool, PPS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS5` writer - Port n Pin Power Save Bit 5"]
pub struct PPS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS5_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS5_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS6_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS6_A> for bool {
    #[inline(always)]
    fn from(variant: PPS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS6` reader - Port n Pin Power Save Bit 6"]
pub struct PPS6_R(crate::FieldReader<bool, PPS6_A>);
impl PPS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS6_A {
        match self.bits {
            false => PPS6_A::CONST_0,
            true => PPS6_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS6_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS6_A::CONST_1
    }
}
impl core::ops::Deref for PPS6_R {
    type Target = crate::FieldReader<bool, PPS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS6` writer - Port n Pin Power Save Bit 6"]
pub struct PPS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS6_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS6_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS7_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS7_A> for bool {
    #[inline(always)]
    fn from(variant: PPS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS7` reader - Port n Pin Power Save Bit 7"]
pub struct PPS7_R(crate::FieldReader<bool, PPS7_A>);
impl PPS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS7_A {
        match self.bits {
            false => PPS7_A::CONST_0,
            true => PPS7_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS7_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS7_A::CONST_1
    }
}
impl core::ops::Deref for PPS7_R {
    type Target = crate::FieldReader<bool, PPS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS7` writer - Port n Pin Power Save Bit 7"]
pub struct PPS7_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS7_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS7_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS8_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS8_A> for bool {
    #[inline(always)]
    fn from(variant: PPS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS8` reader - Port n Pin Power Save Bit 8"]
pub struct PPS8_R(crate::FieldReader<bool, PPS8_A>);
impl PPS8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS8_A {
        match self.bits {
            false => PPS8_A::CONST_0,
            true => PPS8_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS8_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS8_A::CONST_1
    }
}
impl core::ops::Deref for PPS8_R {
    type Target = crate::FieldReader<bool, PPS8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS8` writer - Port n Pin Power Save Bit 8"]
pub struct PPS8_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS8_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS8_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS9_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS9_A> for bool {
    #[inline(always)]
    fn from(variant: PPS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS9` reader - Port n Pin Power Save Bit 9"]
pub struct PPS9_R(crate::FieldReader<bool, PPS9_A>);
impl PPS9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS9_A {
        match self.bits {
            false => PPS9_A::CONST_0,
            true => PPS9_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS9_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS9_A::CONST_1
    }
}
impl core::ops::Deref for PPS9_R {
    type Target = crate::FieldReader<bool, PPS9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS9` writer - Port n Pin Power Save Bit 9"]
pub struct PPS9_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS9_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS9_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS10_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS10_A> for bool {
    #[inline(always)]
    fn from(variant: PPS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS10` reader - Port n Pin Power Save Bit 10"]
pub struct PPS10_R(crate::FieldReader<bool, PPS10_A>);
impl PPS10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS10_A {
        match self.bits {
            false => PPS10_A::CONST_0,
            true => PPS10_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS10_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS10_A::CONST_1
    }
}
impl core::ops::Deref for PPS10_R {
    type Target = crate::FieldReader<bool, PPS10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS10` writer - Port n Pin Power Save Bit 10"]
pub struct PPS10_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS10_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS10_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS11_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS11_A> for bool {
    #[inline(always)]
    fn from(variant: PPS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS11` reader - Port n Pin Power Save Bit 11"]
pub struct PPS11_R(crate::FieldReader<bool, PPS11_A>);
impl PPS11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS11_A {
        match self.bits {
            false => PPS11_A::CONST_0,
            true => PPS11_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS11_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS11_A::CONST_1
    }
}
impl core::ops::Deref for PPS11_R {
    type Target = crate::FieldReader<bool, PPS11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS11` writer - Port n Pin Power Save Bit 11"]
pub struct PPS11_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS11_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS11_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS12_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS12_A> for bool {
    #[inline(always)]
    fn from(variant: PPS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS12` reader - Port n Pin Power Save Bit 12"]
pub struct PPS12_R(crate::FieldReader<bool, PPS12_A>);
impl PPS12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS12_A {
        match self.bits {
            false => PPS12_A::CONST_0,
            true => PPS12_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS12_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS12_A::CONST_1
    }
}
impl core::ops::Deref for PPS12_R {
    type Target = crate::FieldReader<bool, PPS12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS12` writer - Port n Pin Power Save Bit 12"]
pub struct PPS12_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS12_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS12_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS13_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS13_A> for bool {
    #[inline(always)]
    fn from(variant: PPS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS13` reader - Port n Pin Power Save Bit 13"]
pub struct PPS13_R(crate::FieldReader<bool, PPS13_A>);
impl PPS13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS13_A {
        match self.bits {
            false => PPS13_A::CONST_0,
            true => PPS13_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS13_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS13_A::CONST_1
    }
}
impl core::ops::Deref for PPS13_R {
    type Target = crate::FieldReader<bool, PPS13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS13` writer - Port n Pin Power Save Bit 13"]
pub struct PPS13_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS13_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS13_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS14_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS14_A> for bool {
    #[inline(always)]
    fn from(variant: PPS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS14` reader - Port n Pin Power Save Bit 14"]
pub struct PPS14_R(crate::FieldReader<bool, PPS14_A>);
impl PPS14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS14_A {
        match self.bits {
            false => PPS14_A::CONST_0,
            true => PPS14_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS14_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS14_A::CONST_1
    }
}
impl core::ops::Deref for PPS14_R {
    type Target = crate::FieldReader<bool, PPS14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS14` writer - Port n Pin Power Save Bit 14"]
pub struct PPS14_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS14_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS14_A::CONST_1)
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
#[doc = "Port n Pin Power Save Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS15_A {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    CONST_0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    CONST_1 = 1,
}
impl From<PPS15_A> for bool {
    #[inline(always)]
    fn from(variant: PPS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS15` reader - Port n Pin Power Save Bit 15"]
pub struct PPS15_R(crate::FieldReader<bool, PPS15_A>);
impl PPS15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS15_A {
        match self.bits {
            false => PPS15_A::CONST_0,
            true => PPS15_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PPS15_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PPS15_A::CONST_1
    }
}
impl core::ops::Deref for PPS15_R {
    type Target = crate::FieldReader<bool, PPS15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS15` writer - Port n Pin Power Save Bit 15"]
pub struct PPS15_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPS15_A::CONST_0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPS15_A::CONST_1)
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
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline(always)]
    pub fn pps0(&self) -> PPS0_R {
        PPS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline(always)]
    pub fn pps1(&self) -> PPS1_R {
        PPS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline(always)]
    pub fn pps2(&self) -> PPS2_R {
        PPS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline(always)]
    pub fn pps3(&self) -> PPS3_R {
        PPS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline(always)]
    pub fn pps4(&self) -> PPS4_R {
        PPS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline(always)]
    pub fn pps5(&self) -> PPS5_R {
        PPS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline(always)]
    pub fn pps6(&self) -> PPS6_R {
        PPS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline(always)]
    pub fn pps7(&self) -> PPS7_R {
        PPS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline(always)]
    pub fn pps8(&self) -> PPS8_R {
        PPS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline(always)]
    pub fn pps9(&self) -> PPS9_R {
        PPS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline(always)]
    pub fn pps10(&self) -> PPS10_R {
        PPS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline(always)]
    pub fn pps11(&self) -> PPS11_R {
        PPS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline(always)]
    pub fn pps12(&self) -> PPS12_R {
        PPS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline(always)]
    pub fn pps13(&self) -> PPS13_R {
        PPS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline(always)]
    pub fn pps14(&self) -> PPS14_R {
        PPS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline(always)]
    pub fn pps15(&self) -> PPS15_R {
        PPS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline(always)]
    pub fn pps0(&mut self) -> PPS0_W {
        PPS0_W { w: self }
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline(always)]
    pub fn pps1(&mut self) -> PPS1_W {
        PPS1_W { w: self }
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline(always)]
    pub fn pps2(&mut self) -> PPS2_W {
        PPS2_W { w: self }
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline(always)]
    pub fn pps3(&mut self) -> PPS3_W {
        PPS3_W { w: self }
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline(always)]
    pub fn pps4(&mut self) -> PPS4_W {
        PPS4_W { w: self }
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline(always)]
    pub fn pps5(&mut self) -> PPS5_W {
        PPS5_W { w: self }
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline(always)]
    pub fn pps6(&mut self) -> PPS6_W {
        PPS6_W { w: self }
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline(always)]
    pub fn pps7(&mut self) -> PPS7_W {
        PPS7_W { w: self }
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline(always)]
    pub fn pps8(&mut self) -> PPS8_W {
        PPS8_W { w: self }
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline(always)]
    pub fn pps9(&mut self) -> PPS9_W {
        PPS9_W { w: self }
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline(always)]
    pub fn pps10(&mut self) -> PPS10_W {
        PPS10_W { w: self }
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline(always)]
    pub fn pps11(&mut self) -> PPS11_W {
        PPS11_W { w: self }
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline(always)]
    pub fn pps12(&mut self) -> PPS12_W {
        PPS12_W { w: self }
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline(always)]
    pub fn pps13(&mut self) -> PPS13_W {
        PPS13_W { w: self }
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline(always)]
    pub fn pps14(&mut self) -> PPS14_W {
        PPS14_W { w: self }
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline(always)]
    pub fn pps15(&mut self) -> PPS15_W {
        PPS15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Pin Power Save Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps](index.html) module"]
pub struct PPS_SPEC;
impl crate::RegisterSpec for PPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps::R](R) reader structure"]
impl crate::Readable for PPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps::W](W) writer structure"]
impl crate::Writable for PPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPS to value 0"]
impl crate::Resettable for PPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
