#[doc = "Writer for register SRSET"]
pub type W = crate::W<u32, super::SRSET>;
#[doc = "Register SRSET `reset()`'s with value 0"]
impl crate::ResetValue for super::SRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WDT pre-warning Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRWARN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<PRWARN_AW> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_AW) -> Self {
        match variant {
            PRWARN_AW::CONST_0 => false,
            PRWARN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PRWARN`"]
pub struct PRWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRWARN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PRWARN_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PRWARN_AW::CONST_1)
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
#[doc = "RTC Periodic Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<PI_AW> for bool {
    #[inline(always)]
    fn from(variant: PI_AW) -> Self {
        match variant {
            PI_AW::CONST_0 => false,
            PI_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PI`"]
pub struct PI_W<'a> {
    w: &'a mut W,
}
impl<'a> PI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PI_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PI_AW::CONST_1)
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
#[doc = "RTC Alarm Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AI_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<AI_AW> for bool {
    #[inline(always)]
    fn from(variant: AI_AW) -> Self {
        match variant {
            AI_AW::CONST_0 => false,
            AI_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `AI`"]
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(AI_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(AI_AW::CONST_1)
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
#[doc = "DLR Request Overrun Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLROVR_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<DLROVR_AW> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_AW) -> Self {
        match variant {
            DLROVR_AW::CONST_0 => false,
            DLROVR_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `DLROVR`"]
pub struct DLROVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DLROVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLROVR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DLROVR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DLROVR_AW::CONST_1)
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
#[doc = "HDCRCLR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCRCLR_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<HDCRCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCRCLR_AW) -> Self {
        match variant {
            HDCRCLR_AW::CONST_0 => false,
            HDCRCLR_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `HDCRCLR`"]
pub struct HDCRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDCRCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDCRCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDCRCLR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDCRCLR_AW::CONST_1)
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
#[doc = "HDCRSET Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCRSET_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<HDCRSET_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCRSET_AW) -> Self {
        match variant {
            HDCRSET_AW::CONST_0 => false,
            HDCRSET_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `HDCRSET`"]
pub struct HDCRSET_W<'a> {
    w: &'a mut W,
}
impl<'a> HDCRSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDCRSET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDCRSET_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDCRSET_AW::CONST_1)
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
#[doc = "HDCR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCR_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<HDCR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCR_AW) -> Self {
        match variant {
            HDCR_AW::CONST_0 => false,
            HDCR_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `HDCR`"]
pub struct HDCR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDCR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDCR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDCR_AW::CONST_1)
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
#[doc = "OSCSICTRL Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSICTRL_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<OSCSICTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_AW) -> Self {
        match variant {
            OSCSICTRL_AW::CONST_0 => false,
            OSCSICTRL_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `OSCSICTRL`"]
pub struct OSCSICTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSICTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSICTRL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCSICTRL_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCSICTRL_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "OSCULCTRL Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCULCTRL_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<OSCULCTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_AW) -> Self {
        match variant {
            OSCULCTRL_AW::CONST_0 => false,
            OSCULCTRL_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `OSCULCTRL`"]
pub struct OSCULCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCULCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCULCTRL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCULCTRL_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCULCTRL_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "RTC CTR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CTR_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<RTC_CTR_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_AW) -> Self {
        match variant {
            RTC_CTR_AW::CONST_0 => false,
            RTC_CTR_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RTC_CTR`"]
pub struct RTC_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_CTR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_CTR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_CTR_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM0_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<RTC_ATIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_AW) -> Self {
        match variant {
            RTC_ATIM0_AW::CONST_0 => false,
            RTC_ATIM0_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RTC_ATIM0`"]
pub struct RTC_ATIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ATIM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_ATIM0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_ATIM0_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_ATIM0_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM1_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<RTC_ATIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_AW) -> Self {
        match variant {
            RTC_ATIM1_AW::CONST_0 => false,
            RTC_ATIM1_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RTC_ATIM1`"]
pub struct RTC_ATIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ATIM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_ATIM1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_ATIM1_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_ATIM1_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "RTC TIM0 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM0_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<RTC_TIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_AW) -> Self {
        match variant {
            RTC_TIM0_AW::CONST_0 => false,
            RTC_TIM0_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RTC_TIM0`"]
pub struct RTC_TIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_TIM0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_TIM0_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_TIM0_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "RTC TIM1 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM1_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<RTC_TIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_AW) -> Self {
        match variant {
            RTC_TIM1_AW::CONST_0 => false,
            RTC_TIM1_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RTC_TIM1`"]
pub struct RTC_TIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_TIM1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_TIM1_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_TIM1_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Retention Memory Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMX_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: set the status bit"]
    CONST_1,
}
impl From<RMX_AW> for bool {
    #[inline(always)]
    fn from(variant: RMX_AW) -> Self {
        match variant {
            RMX_AW::CONST_0 => false,
            RMX_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RMX`"]
pub struct RMX_W<'a> {
    w: &'a mut W,
}
impl<'a> RMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RMX_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RMX_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Set"]
    #[inline(always)]
    pub fn prwarn(&mut self) -> PRWARN_W {
        PRWARN_W { w: self }
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Set"]
    #[inline(always)]
    pub fn pi(&mut self) -> PI_W {
        PI_W { w: self }
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Set"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W { w: self }
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Set"]
    #[inline(always)]
    pub fn dlrovr(&mut self) -> DLROVR_W {
        DLROVR_W { w: self }
    }
    #[doc = "Bit 17 - HDCRCLR Mirror Register Update Set"]
    #[inline(always)]
    pub fn hdcrclr(&mut self) -> HDCRCLR_W {
        HDCRCLR_W { w: self }
    }
    #[doc = "Bit 18 - HDCRSET Mirror Register Update Set"]
    #[inline(always)]
    pub fn hdcrset(&mut self) -> HDCRSET_W {
        HDCRSET_W { w: self }
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Set"]
    #[inline(always)]
    pub fn hdcr(&mut self) -> HDCR_W {
        HDCR_W { w: self }
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Set"]
    #[inline(always)]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W {
        OSCSICTRL_W { w: self }
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Set"]
    #[inline(always)]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W {
        OSCULCTRL_W { w: self }
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Set"]
    #[inline(always)]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W {
        RTC_CTR_W { w: self }
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Set"]
    #[inline(always)]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W {
        RTC_ATIM0_W { w: self }
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Set"]
    #[inline(always)]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W {
        RTC_ATIM1_W { w: self }
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Set"]
    #[inline(always)]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W {
        RTC_TIM0_W { w: self }
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Set"]
    #[inline(always)]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W {
        RTC_TIM1_W { w: self }
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Set"]
    #[inline(always)]
    pub fn rmx(&mut self) -> RMX_W {
        RMX_W { w: self }
    }
}
