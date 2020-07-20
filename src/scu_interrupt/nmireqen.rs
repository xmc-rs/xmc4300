#[doc = "Reader of register NMIREQEN"]
pub type R = crate::R<u32, super::NMIREQEN>;
#[doc = "Writer for register NMIREQEN"]
pub type W = crate::W<u32, super::NMIREQEN>;
#[doc = "Register NMIREQEN `reset()`'s with value 0"]
impl crate::ResetValue for super::NMIREQEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `PRWARN`"]
pub type PRWARN_R = crate::R<bool, PRWARN_A>;
impl PRWARN_R {
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
        *self == PRWARN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PRWARN_A::CONST_1
    }
}
#[doc = "Write proxy for field `PRWARN`"]
pub struct PRWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRWARN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `PI`"]
pub type PI_R = crate::R<bool, PI_A>;
impl PI_R {
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
        *self == PI_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PI_A::CONST_1
    }
}
#[doc = "Write proxy for field `PI`"]
pub struct PI_W<'a> {
    w: &'a mut W,
}
impl<'a> PI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
#[doc = "Reader of field `AI`"]
pub type AI_R = crate::R<bool, AI_A>;
impl AI_R {
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
        *self == AI_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == AI_A::CONST_1
    }
}
#[doc = "Write proxy for field `AI`"]
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Reader of field `ERU00`"]
pub type ERU00_R = crate::R<bool, ERU00_A>;
impl ERU00_R {
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
        *self == ERU00_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU00_A::CONST_1
    }
}
#[doc = "Write proxy for field `ERU00`"]
pub struct ERU00_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU00_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU00_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
#[doc = "Reader of field `ERU01`"]
pub type ERU01_R = crate::R<bool, ERU01_A>;
impl ERU01_R {
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
        *self == ERU01_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU01_A::CONST_1
    }
}
#[doc = "Write proxy for field `ERU01`"]
pub struct ERU01_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU01_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU01_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
#[doc = "Reader of field `ERU02`"]
pub type ERU02_R = crate::R<bool, ERU02_A>;
impl ERU02_R {
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
        *self == ERU02_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU02_A::CONST_1
    }
}
#[doc = "Write proxy for field `ERU02`"]
pub struct ERU02_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU02_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU02_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
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
#[doc = "Reader of field `ERU03`"]
pub type ERU03_R = crate::R<bool, ERU03_A>;
impl ERU03_R {
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
        *self == ERU03_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ERU03_A::CONST_1
    }
}
#[doc = "Write proxy for field `ERU03`"]
pub struct ERU03_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU03_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU03_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
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
}
