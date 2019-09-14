#[doc = "Reader of register HDCR"]
pub type R = crate::R<u32, super::HDCR>;
#[doc = "Writer for register HDCR"]
pub type W = crate::W<u32, super::HDCR>;
#[doc = "Register HDCR `reset()`'s with value 0x000c_2000"]
impl crate::ResetValue for super::HDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000c_2000
    }
}
#[doc = "Wake-Up on Pin Event Positive Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKPEP_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1,
}
impl From<WKPEP_A> for bool {
    #[inline(always)]
    fn from(variant: WKPEP_A) -> Self {
        match variant {
            WKPEP_A::CONST_0 => false,
            WKPEP_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `WKPEP`"]
pub type WKPEP_R = crate::R<bool, WKPEP_A>;
impl WKPEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKPEP_A {
        match self.bits {
            false => WKPEP_A::CONST_0,
            true => WKPEP_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WKPEP_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WKPEP_A::CONST_1
    }
}
#[doc = "Write proxy for field `WKPEP`"]
pub struct WKPEP_W<'a> {
    w: &'a mut W,
}
impl<'a> WKPEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKPEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WKPEP_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WKPEP_A::CONST_1)
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
#[doc = "Wake-up on Pin Event Negative Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKPEN_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1,
}
impl From<WKPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKPEN_A) -> Self {
        match variant {
            WKPEN_A::CONST_0 => false,
            WKPEN_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `WKPEN`"]
pub type WKPEN_R = crate::R<bool, WKPEN_A>;
impl WKPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKPEN_A {
        match self.bits {
            false => WKPEN_A::CONST_0,
            true => WKPEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WKPEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WKPEN_A::CONST_1
    }
}
#[doc = "Write proxy for field `WKPEN`"]
pub struct WKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WKPEN_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WKPEN_A::CONST_1)
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
#[doc = "Wake-up on RTC Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCE_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1,
}
impl From<RTCE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCE_A) -> Self {
        match variant {
            RTCE_A::CONST_0 => false,
            RTCE_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTCE`"]
pub type RTCE_R = crate::R<bool, RTCE_A>;
impl RTCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCE_A {
        match self.bits {
            false => RTCE_A::CONST_0,
            true => RTCE_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTCE_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTCE_A::CONST_1
    }
}
#[doc = "Write proxy for field `RTCE`"]
pub struct RTCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTCE_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTCE_A::CONST_1)
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
#[doc = "ULP WDG Alarm Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGEN_A {
    #[doc = "0: Wake-up event disabled"]
    CONST_0,
    #[doc = "1: Wake-up event enabled"]
    CONST_1,
}
impl From<ULPWDGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGEN_A) -> Self {
        match variant {
            ULPWDGEN_A::CONST_0 => false,
            ULPWDGEN_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `ULPWDGEN`"]
pub type ULPWDGEN_R = crate::R<bool, ULPWDGEN_A>;
impl ULPWDGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGEN_A {
        match self.bits {
            false => ULPWDGEN_A::CONST_0,
            true => ULPWDGEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGEN_A::CONST_1
    }
}
#[doc = "Write proxy for field `ULPWDGEN`"]
pub struct ULPWDGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPWDGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULPWDGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDGEN_A::CONST_0)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDGEN_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Hibernate Request Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_A {
    #[doc = "0: External hibernate request inactive"]
    CONST_0,
    #[doc = "1: External hibernate request active"]
    CONST_1,
}
impl From<HIB_A> for bool {
    #[inline(always)]
    fn from(variant: HIB_A) -> Self {
        match variant {
            HIB_A::CONST_0 => false,
            HIB_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `HIB`"]
pub type HIB_R = crate::R<bool, HIB_A>;
impl HIB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIB_A {
        match self.bits {
            false => HIB_A::CONST_0,
            true => HIB_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIB_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIB_A::CONST_1
    }
}
#[doc = "Write proxy for field `HIB`"]
pub struct HIB_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIB_A::CONST_0)
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIB_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "fRTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCS_A {
    #[doc = "0: fOSI selected"]
    CONST_0,
    #[doc = "1: fULP selected"]
    CONST_1,
}
impl From<RCS_A> for bool {
    #[inline(always)]
    fn from(variant: RCS_A) -> Self {
        match variant {
            RCS_A::CONST_0 => false,
            RCS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RCS`"]
pub type RCS_R = crate::R<bool, RCS_A>;
impl RCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCS_A {
        match self.bits {
            false => RCS_A::CONST_0,
            true => RCS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RCS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RCS_A::CONST_1
    }
}
#[doc = "Write proxy for field `RCS`"]
pub struct RCS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RCS_A::CONST_0)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RCS_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "fSTDBY Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDBYSEL_A {
    #[doc = "0: fOSI selected"]
    CONST_0,
    #[doc = "1: fULP selected"]
    CONST_1,
}
impl From<STDBYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: STDBYSEL_A) -> Self {
        match variant {
            STDBYSEL_A::CONST_0 => false,
            STDBYSEL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `STDBYSEL`"]
pub type STDBYSEL_R = crate::R<bool, STDBYSEL_A>;
impl STDBYSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STDBYSEL_A {
        match self.bits {
            false => STDBYSEL_A::CONST_0,
            true => STDBYSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == STDBYSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == STDBYSEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `STDBYSEL`"]
pub struct STDBYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STDBYSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STDBYSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(STDBYSEL_A::CONST_0)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(STDBYSEL_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Wake-Up from Hibernate Trigger Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPSEL_A {
    #[doc = "0: HIB_IO_1 pin selected"]
    CONST_0,
    #[doc = "1: HIB_IO_0 pin selected"]
    CONST_1,
}
impl From<WKUPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPSEL_A) -> Self {
        match variant {
            WKUPSEL_A::CONST_0 => false,
            WKUPSEL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `WKUPSEL`"]
pub type WKUPSEL_R = crate::R<bool, WKUPSEL_A>;
impl WKUPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPSEL_A {
        match self.bits {
            false => WKUPSEL_A::CONST_0,
            true => WKUPSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WKUPSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WKUPSEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `WKUPSEL`"]
pub struct WKUPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WKUPSEL_A::CONST_0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WKUPSEL_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "General Purpose Input 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPI0SEL_A {
    #[doc = "0: #0"]
    CONST_0,
    #[doc = "1: HIB_IO_0 pin selected"]
    CONST_1,
}
impl From<GPI0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPI0SEL_A) -> Self {
        match variant {
            GPI0SEL_A::CONST_0 => false,
            GPI0SEL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `GPI0SEL`"]
pub type GPI0SEL_R = crate::R<bool, GPI0SEL_A>;
impl GPI0SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPI0SEL_A {
        match self.bits {
            false => GPI0SEL_A::CONST_0,
            true => GPI0SEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == GPI0SEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == GPI0SEL_A::CONST_1
    }
}
#[doc = "Write proxy for field `GPI0SEL`"]
pub struct GPI0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPI0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPI0SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "#0"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(GPI0SEL_A::CONST_0)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(GPI0SEL_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "HIBIO0 Polarity Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO0POL_A {
    #[doc = "0: Direct value"]
    CONST_0,
    #[doc = "1: Inverted value"]
    CONST_1,
}
impl From<HIBIO0POL_A> for bool {
    #[inline(always)]
    fn from(variant: HIBIO0POL_A) -> Self {
        match variant {
            HIBIO0POL_A::CONST_0 => false,
            HIBIO0POL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `HIBIO0POL`"]
pub type HIBIO0POL_R = crate::R<bool, HIBIO0POL_A>;
impl HIBIO0POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBIO0POL_A {
        match self.bits {
            false => HIBIO0POL_A::CONST_0,
            true => HIBIO0POL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBIO0POL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBIO0POL_A::CONST_1
    }
}
#[doc = "Write proxy for field `HIBIO0POL`"]
pub struct HIBIO0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBIO0POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBIO0POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBIO0POL_A::CONST_0)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBIO0POL_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "HIBIO1 Polarity Set\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO1POL_A {
    #[doc = "0: Direct value"]
    CONST_0,
    #[doc = "1: Inverted value"]
    CONST_1,
}
impl From<HIBIO1POL_A> for bool {
    #[inline(always)]
    fn from(variant: HIBIO1POL_A) -> Self {
        match variant {
            HIBIO1POL_A::CONST_0 => false,
            HIBIO1POL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `HIBIO1POL`"]
pub type HIBIO1POL_R = crate::R<bool, HIBIO1POL_A>;
impl HIBIO1POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBIO1POL_A {
        match self.bits {
            false => HIBIO1POL_A::CONST_0,
            true => HIBIO1POL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBIO1POL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBIO1POL_A::CONST_1
    }
}
#[doc = "Write proxy for field `HIBIO1POL`"]
pub struct HIBIO1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBIO1POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBIO1POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIBIO1POL_A::CONST_0)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIBIO1POL_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "HIB_IO_0 Pin I/O Control (default HIBOUT)\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO0SEL_A {
    #[doc = "0: Direct input, No input pull device connected"]
    CONST_0000,
    #[doc = "1: Direct input, Input pull-down device connected"]
    CONST_0001,
    #[doc = "2: Direct input, Input pull-up device connected"]
    CONST_0010,
    #[doc = "8: Push-pull HIB Control output"]
    CONST_1000,
    #[doc = "9: Push-pull WDT service output"]
    CONST_1001,
    #[doc = "10: Push-pull GPIO output"]
    CONST_1010,
    #[doc = "12: Open-drain HIB Control output"]
    CONST_1100,
    #[doc = "13: Open-drain WDT service output"]
    CONST_1101,
    #[doc = "14: Open-drain GPIO output"]
    CONST_1110,
    #[doc = "15: #1111"]
    CONST_1111,
}
impl From<HIBIO0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIBIO0SEL_A) -> Self {
        match variant {
            HIBIO0SEL_A::CONST_0000 => 0,
            HIBIO0SEL_A::CONST_0001 => 1,
            HIBIO0SEL_A::CONST_0010 => 2,
            HIBIO0SEL_A::CONST_1000 => 8,
            HIBIO0SEL_A::CONST_1001 => 9,
            HIBIO0SEL_A::CONST_1010 => 10,
            HIBIO0SEL_A::CONST_1100 => 12,
            HIBIO0SEL_A::CONST_1101 => 13,
            HIBIO0SEL_A::CONST_1110 => 14,
            HIBIO0SEL_A::CONST_1111 => 15,
        }
    }
}
#[doc = "Reader of field `HIBIO0SEL`"]
pub type HIBIO0SEL_R = crate::R<u8, HIBIO0SEL_A>;
impl HIBIO0SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HIBIO0SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HIBIO0SEL_A::CONST_0000),
            1 => Val(HIBIO0SEL_A::CONST_0001),
            2 => Val(HIBIO0SEL_A::CONST_0010),
            8 => Val(HIBIO0SEL_A::CONST_1000),
            9 => Val(HIBIO0SEL_A::CONST_1001),
            10 => Val(HIBIO0SEL_A::CONST_1010),
            12 => Val(HIBIO0SEL_A::CONST_1100),
            13 => Val(HIBIO0SEL_A::CONST_1101),
            14 => Val(HIBIO0SEL_A::CONST_1110),
            15 => Val(HIBIO0SEL_A::CONST_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0000`"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_0000
    }
    #[doc = "Checks if the value of the field is `CONST_0001`"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_0001
    }
    #[doc = "Checks if the value of the field is `CONST_0010`"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_0010
    }
    #[doc = "Checks if the value of the field is `CONST_1000`"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1000
    }
    #[doc = "Checks if the value of the field is `CONST_1001`"]
    #[inline(always)]
    pub fn is_const_1001(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1001
    }
    #[doc = "Checks if the value of the field is `CONST_1010`"]
    #[inline(always)]
    pub fn is_const_1010(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1010
    }
    #[doc = "Checks if the value of the field is `CONST_1100`"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1100
    }
    #[doc = "Checks if the value of the field is `CONST_1101`"]
    #[inline(always)]
    pub fn is_const_1101(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1101
    }
    #[doc = "Checks if the value of the field is `CONST_1110`"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1110
    }
    #[doc = "Checks if the value of the field is `CONST_1111`"]
    #[inline(always)]
    pub fn is_const_1111(&self) -> bool {
        *self == HIBIO0SEL_A::CONST_1111
    }
}
#[doc = "Write proxy for field `HIBIO0SEL`"]
pub struct HIBIO0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBIO0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBIO0SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn const_1001(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn const_1010(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn const_1101(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_1110)
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn const_1111(self) -> &'a mut W {
        self.variant(HIBIO0SEL_A::CONST_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "HIB_IO_1 Pin I/O Control (Default WKUP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO1SEL_A {
    #[doc = "0: Direct input, No input pull device connected"]
    CONST_0000,
    #[doc = "1: Direct input, Input pull-down device connected"]
    CONST_0001,
    #[doc = "2: Direct input, Input pull-up device connected"]
    CONST_0010,
    #[doc = "8: Push-pull HIB Control output"]
    CONST_1000,
    #[doc = "9: Push-pull WDT service output"]
    CONST_1001,
    #[doc = "10: Push-pull GPIO output"]
    CONST_1010,
    #[doc = "12: Open-drain HIB Control output"]
    CONST_1100,
    #[doc = "13: Open-drain WDT service output"]
    CONST_1101,
    #[doc = "14: Open-drain GPIO output"]
    CONST_1110,
    #[doc = "15: #1111"]
    CONST_1111,
}
impl From<HIBIO1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIBIO1SEL_A) -> Self {
        match variant {
            HIBIO1SEL_A::CONST_0000 => 0,
            HIBIO1SEL_A::CONST_0001 => 1,
            HIBIO1SEL_A::CONST_0010 => 2,
            HIBIO1SEL_A::CONST_1000 => 8,
            HIBIO1SEL_A::CONST_1001 => 9,
            HIBIO1SEL_A::CONST_1010 => 10,
            HIBIO1SEL_A::CONST_1100 => 12,
            HIBIO1SEL_A::CONST_1101 => 13,
            HIBIO1SEL_A::CONST_1110 => 14,
            HIBIO1SEL_A::CONST_1111 => 15,
        }
    }
}
#[doc = "Reader of field `HIBIO1SEL`"]
pub type HIBIO1SEL_R = crate::R<u8, HIBIO1SEL_A>;
impl HIBIO1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HIBIO1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HIBIO1SEL_A::CONST_0000),
            1 => Val(HIBIO1SEL_A::CONST_0001),
            2 => Val(HIBIO1SEL_A::CONST_0010),
            8 => Val(HIBIO1SEL_A::CONST_1000),
            9 => Val(HIBIO1SEL_A::CONST_1001),
            10 => Val(HIBIO1SEL_A::CONST_1010),
            12 => Val(HIBIO1SEL_A::CONST_1100),
            13 => Val(HIBIO1SEL_A::CONST_1101),
            14 => Val(HIBIO1SEL_A::CONST_1110),
            15 => Val(HIBIO1SEL_A::CONST_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0000`"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_0000
    }
    #[doc = "Checks if the value of the field is `CONST_0001`"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_0001
    }
    #[doc = "Checks if the value of the field is `CONST_0010`"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_0010
    }
    #[doc = "Checks if the value of the field is `CONST_1000`"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1000
    }
    #[doc = "Checks if the value of the field is `CONST_1001`"]
    #[inline(always)]
    pub fn is_const_1001(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1001
    }
    #[doc = "Checks if the value of the field is `CONST_1010`"]
    #[inline(always)]
    pub fn is_const_1010(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1010
    }
    #[doc = "Checks if the value of the field is `CONST_1100`"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1100
    }
    #[doc = "Checks if the value of the field is `CONST_1101`"]
    #[inline(always)]
    pub fn is_const_1101(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1101
    }
    #[doc = "Checks if the value of the field is `CONST_1110`"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1110
    }
    #[doc = "Checks if the value of the field is `CONST_1111`"]
    #[inline(always)]
    pub fn is_const_1111(&self) -> bool {
        *self == HIBIO1SEL_A::CONST_1111
    }
}
#[doc = "Write proxy for field `HIBIO1SEL`"]
pub struct HIBIO1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBIO1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBIO1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_0000)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_0001)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_0010)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_1000)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn const_1001(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_1001)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn const_1010(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_1010)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_1100)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn const_1101(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_1101)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_1110)
    }
    #[doc = "#1111"]
    #[inline(always)]
    pub fn const_1111(self) -> &'a mut W {
        self.variant(HIBIO1SEL_A::CONST_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    pub fn wkpep(&self) -> WKPEP_R {
        WKPEP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    pub fn wkpen(&self) -> WKPEN_R {
        WKPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    pub fn rtce(&self) -> RTCE_R {
        RTCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    pub fn ulpwdgen(&self) -> ULPWDGEN_R {
        ULPWDGEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    pub fn hib(&self) -> HIB_R {
        HIB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    pub fn rcs(&self) -> RCS_R {
        RCS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    pub fn stdbysel(&self) -> STDBYSEL_R {
        STDBYSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    pub fn wkupsel(&self) -> WKUPSEL_R {
        WKUPSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    pub fn gpi0sel(&self) -> GPI0SEL_R {
        GPI0SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    pub fn hibio0pol(&self) -> HIBIO0POL_R {
        HIBIO0POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    pub fn hibio1pol(&self) -> HIBIO1POL_R {
        HIBIO1POL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    pub fn hibio0sel(&self) -> HIBIO0SEL_R {
        HIBIO0SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    pub fn hibio1sel(&self) -> HIBIO1SEL_R {
        HIBIO1SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    pub fn wkpep(&mut self) -> WKPEP_W {
        WKPEP_W { w: self }
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    pub fn wkpen(&mut self) -> WKPEN_W {
        WKPEN_W { w: self }
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    pub fn rtce(&mut self) -> RTCE_W {
        RTCE_W { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    pub fn ulpwdgen(&mut self) -> ULPWDGEN_W {
        ULPWDGEN_W { w: self }
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    pub fn hib(&mut self) -> HIB_W {
        HIB_W { w: self }
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    pub fn rcs(&mut self) -> RCS_W {
        RCS_W { w: self }
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    pub fn stdbysel(&mut self) -> STDBYSEL_W {
        STDBYSEL_W { w: self }
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    pub fn wkupsel(&mut self) -> WKUPSEL_W {
        WKUPSEL_W { w: self }
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    pub fn gpi0sel(&mut self) -> GPI0SEL_W {
        GPI0SEL_W { w: self }
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    pub fn hibio0pol(&mut self) -> HIBIO0POL_W {
        HIBIO0POL_W { w: self }
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    pub fn hibio1pol(&mut self) -> HIBIO1POL_W {
        HIBIO1POL_W { w: self }
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    pub fn hibio0sel(&mut self) -> HIBIO0SEL_W {
        HIBIO0SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    pub fn hibio1sel(&mut self) -> HIBIO1SEL_W {
        HIBIO1SEL_W { w: self }
    }
}
