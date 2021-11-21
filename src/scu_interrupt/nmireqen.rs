#[doc = "Register `NMIREQEN` reader"]
pub struct R(crate::R<NMIREQEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMIREQEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMIREQEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMIREQEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMIREQEN` writer"]
pub struct W(crate::W<NMIREQEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMIREQEN_SPEC>;
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
impl From<crate::W<NMIREQEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMIREQEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Promote Pre-Warning Interrupt Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRWARN_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PRWARN_A> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` reader - Promote Pre-Warning Interrupt Request to NMI Request"]
pub struct PRWARN_R(crate::FieldReader<bool, PRWARN_A>);
impl PRWARN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRWARN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRWARN_A {
        match self.bits {
            false => PRWARN_A::CONST_0,
            true => PRWARN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PRWARN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PRWARN_A::CONST_1
    }
}
impl core::ops::Deref for PRWARN_R {
    type Target = crate::FieldReader<bool, PRWARN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRWARN` writer - Promote Pre-Warning Interrupt Request to NMI Request"]
pub struct PRWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRWARN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PRWARN_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PRWARN_A::CONST_1)
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
#[doc = "Promote RTC Periodic Interrupt request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` reader - Promote RTC Periodic Interrupt request to NMI Request"]
pub struct PI_R(crate::FieldReader<bool, PI_A>);
impl PI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::CONST_0,
            true => PI_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PI_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PI_A::CONST_1
    }
}
impl core::ops::Deref for PI_R {
    type Target = crate::FieldReader<bool, PI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PI` writer - Promote RTC Periodic Interrupt request to NMI Request"]
pub struct PI_W<'a> {
    w: &'a mut W,
}
impl<'a> PI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PI_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PI_A::CONST_1)
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
#[doc = "Promote RTC Alarm Interrupt Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AI_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<AI_A> for bool {
    #[inline(always)]
    fn from(variant: AI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` reader - Promote RTC Alarm Interrupt Request to NMI Request"]
pub struct AI_R(crate::FieldReader<bool, AI_A>);
impl AI_R {
    pub(crate) fn new(bits: bool) -> Self {
        AI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AI_A {
        match self.bits {
            false => AI_A::CONST_0,
            true => AI_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == AI_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == AI_A::CONST_1
    }
}
impl core::ops::Deref for AI_R {
    type Target = crate::FieldReader<bool, AI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI` writer - Promote RTC Alarm Interrupt Request to NMI Request"]
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(AI_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(AI_A::CONST_1)
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
#[doc = "Promote Channel 0 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU00_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU00_A> for bool {
    #[inline(always)]
    fn from(variant: ERU00_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU00` reader - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU00_R(crate::FieldReader<bool, ERU00_A>);
impl ERU00_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERU00_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU00_A {
        match self.bits {
            false => ERU00_A::CONST_0,
            true => ERU00_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ERU00_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ERU00_A::CONST_1
    }
}
impl core::ops::Deref for ERU00_R {
    type Target = crate::FieldReader<bool, ERU00_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERU00` writer - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU00_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU00_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU00_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU00_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU00_A::CONST_1)
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
#[doc = "Promote Channel 1 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU01_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU01_A> for bool {
    #[inline(always)]
    fn from(variant: ERU01_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU01` reader - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU01_R(crate::FieldReader<bool, ERU01_A>);
impl ERU01_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERU01_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU01_A {
        match self.bits {
            false => ERU01_A::CONST_0,
            true => ERU01_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ERU01_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ERU01_A::CONST_1
    }
}
impl core::ops::Deref for ERU01_R {
    type Target = crate::FieldReader<bool, ERU01_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERU01` writer - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU01_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU01_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU01_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU01_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU01_A::CONST_1)
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
#[doc = "Promote Channel 2 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU02_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU02_A> for bool {
    #[inline(always)]
    fn from(variant: ERU02_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU02` reader - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU02_R(crate::FieldReader<bool, ERU02_A>);
impl ERU02_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERU02_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU02_A {
        match self.bits {
            false => ERU02_A::CONST_0,
            true => ERU02_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ERU02_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ERU02_A::CONST_1
    }
}
impl core::ops::Deref for ERU02_R {
    type Target = crate::FieldReader<bool, ERU02_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERU02` writer - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU02_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU02_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU02_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU02_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU02_A::CONST_1)
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
#[doc = "Promote Channel 3 Interrupt of ERU0 Request to NMI Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU03_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ERU03_A> for bool {
    #[inline(always)]
    fn from(variant: ERU03_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU03` reader - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU03_R(crate::FieldReader<bool, ERU03_A>);
impl ERU03_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERU03_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU03_A {
        match self.bits {
            false => ERU03_A::CONST_0,
            true => ERU03_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == ERU03_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == ERU03_A::CONST_1
    }
}
impl core::ops::Deref for ERU03_R {
    type Target = crate::FieldReader<bool, ERU03_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERU03` writer - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
pub struct ERU03_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU03_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU03_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ERU03_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ERU03_A::CONST_1)
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
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru00(&self) -> ERU00_R {
        ERU00_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru01(&self) -> ERU01_R {
        ERU01_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru02(&self) -> ERU02_R {
        ERU02_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru03(&self) -> ERU03_R {
        ERU03_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn prwarn(&mut self) -> PRWARN_W {
        PRWARN_W { w: self }
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline(always)]
    pub fn pi(&mut self) -> PI_W {
        PI_W { w: self }
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W { w: self }
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru00(&mut self) -> ERU00_W {
        ERU00_W { w: self }
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru01(&mut self) -> ERU01_W {
        ERU01_W { w: self }
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru02(&mut self) -> ERU02_W {
        ERU02_W { w: self }
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline(always)]
    pub fn eru03(&mut self) -> ERU03_W {
        ERU03_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCU Service Request Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmireqen](index.html) module"]
pub struct NMIREQEN_SPEC;
impl crate::RegisterSpec for NMIREQEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmireqen::R](R) reader structure"]
impl crate::Readable for NMIREQEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmireqen::W](W) writer structure"]
impl crate::Writable for NMIREQEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMIREQEN to value 0"]
impl crate::Resettable for NMIREQEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
