#[doc = "Register `SRMSK` reader"]
pub struct R(crate::R<SRMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRMSK` writer"]
pub struct W(crate::W<SRMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRMSK_SPEC>;
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
impl From<crate::W<SRMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Mask"]
pub type PRWARN_R = crate::BitReader<PRWARN_A>;
#[doc = "WDT pre-warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PRWARN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Mask"]
pub type PRWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, PRWARN_A, O>;
impl<'a, const O: u8> PRWARN_W<'a, O> {
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
}
#[doc = "Field `PI` reader - RTC Periodic Interrupt Mask"]
pub type PI_R = crate::BitReader<PI_A>;
#[doc = "RTC Periodic Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PI_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `PI` writer - RTC Periodic Interrupt Mask"]
pub type PI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, PI_A, O>;
impl<'a, const O: u8> PI_W<'a, O> {
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
}
#[doc = "Field `AI` reader - RTC Alarm Interrupt Mask"]
pub type AI_R = crate::BitReader<AI_A>;
#[doc = "RTC Alarm Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl AI_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `AI` writer - RTC Alarm Interrupt Mask"]
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, AI_A, O>;
impl<'a, const O: u8> AI_W<'a, O> {
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
}
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Mask"]
pub type DLROVR_R = crate::BitReader<DLROVR_A>;
#[doc = "DLR Request Overrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLROVR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<DLROVR_A> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_A) -> Self {
        variant as u8 != 0
    }
}
impl DLROVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLROVR_A {
        match self.bits {
            false => DLROVR_A::CONST_0,
            true => DLROVR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == DLROVR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == DLROVR_A::CONST_1
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt Mask"]
pub type DLROVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, DLROVR_A, O>;
impl<'a, const O: u8> DLROVR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DLROVR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DLROVR_A::CONST_1)
    }
}
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Mask"]
pub type HDCLR_R = crate::BitReader<HDCLR_A>;
#[doc = "HDCLR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<HDCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl HDCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCLR_A {
        match self.bits {
            false => HDCLR_A::CONST_0,
            true => HDCLR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDCLR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDCLR_A::CONST_1
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Mask"]
pub type HDCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, HDCLR_A, O>;
impl<'a, const O: u8> HDCLR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDCLR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDCLR_A::CONST_1)
    }
}
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Mask"]
pub type HDSET_R = crate::BitReader<HDSET_A>;
#[doc = "HDSET Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<HDSET_A> for bool {
    #[inline(always)]
    fn from(variant: HDSET_A) -> Self {
        variant as u8 != 0
    }
}
impl HDSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSET_A {
        match self.bits {
            false => HDSET_A::CONST_0,
            true => HDSET_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDSET_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDSET_A::CONST_1
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Mask"]
pub type HDSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, HDSET_A, O>;
impl<'a, const O: u8> HDSET_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDSET_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDSET_A::CONST_1)
    }
}
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Mask"]
pub type HDCR_R = crate::BitReader<HDCR_A>;
#[doc = "HDCR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<HDCR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCR_A) -> Self {
        variant as u8 != 0
    }
}
impl HDCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCR_A {
        match self.bits {
            false => HDCR_A::CONST_0,
            true => HDCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDCR_A::CONST_1
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Mask"]
pub type HDCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, HDCR_A, O>;
impl<'a, const O: u8> HDCR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDCR_A::CONST_1)
    }
}
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Mask"]
pub type OSCSICTRL_R = crate::BitReader<OSCSICTRL_A>;
#[doc = "OSCSICTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<OSCSICTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCSICTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSICTRL_A {
        match self.bits {
            false => OSCSICTRL_A::CONST_0,
            true => OSCSICTRL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCSICTRL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCSICTRL_A::CONST_1
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Mask"]
pub type OSCSICTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, OSCSICTRL_A, O>;
impl<'a, const O: u8> OSCSICTRL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCSICTRL_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCSICTRL_A::CONST_1)
    }
}
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Mask"]
pub type OSCULCTRL_R = crate::BitReader<OSCULCTRL_A>;
#[doc = "OSCULCTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<OSCULCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCULCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCULCTRL_A {
        match self.bits {
            false => OSCULCTRL_A::CONST_0,
            true => OSCULCTRL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCULCTRL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCULCTRL_A::CONST_1
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Mask"]
pub type OSCULCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, OSCULCTRL_A, O>;
impl<'a, const O: u8> OSCULCTRL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCULCTRL_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCULCTRL_A::CONST_1)
    }
}
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Mask"]
pub type RTC_CTR_R = crate::BitReader<RTC_CTR_A>;
#[doc = "RTC CTR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<RTC_CTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_CTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CTR_A {
        match self.bits {
            false => RTC_CTR_A::CONST_0,
            true => RTC_CTR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_CTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_CTR_A::CONST_1
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Mask"]
pub type RTC_CTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, RTC_CTR_A, O>;
impl<'a, const O: u8> RTC_CTR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_CTR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_CTR_A::CONST_1)
    }
}
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Mask"]
pub type RTC_ATIM0_R = crate::BitReader<RTC_ATIM0_A>;
#[doc = "RTC ATIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<RTC_ATIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_ATIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM0_A {
        match self.bits {
            false => RTC_ATIM0_A::CONST_0,
            true => RTC_ATIM0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM0_A::CONST_1
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Mask"]
pub type RTC_ATIM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, RTC_ATIM0_A, O>;
impl<'a, const O: u8> RTC_ATIM0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_ATIM0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_ATIM0_A::CONST_1)
    }
}
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Mask"]
pub type RTC_ATIM1_R = crate::BitReader<RTC_ATIM1_A>;
#[doc = "RTC ATIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<RTC_ATIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_ATIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM1_A {
        match self.bits {
            false => RTC_ATIM1_A::CONST_0,
            true => RTC_ATIM1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM1_A::CONST_1
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Mask"]
pub type RTC_ATIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, RTC_ATIM1_A, O>;
impl<'a, const O: u8> RTC_ATIM1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_ATIM1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_ATIM1_A::CONST_1)
    }
}
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Mask"]
pub type RTC_TIM0_R = crate::BitReader<RTC_TIM0_A>;
#[doc = "RTC TIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<RTC_TIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_TIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM0_A {
        match self.bits {
            false => RTC_TIM0_A::CONST_0,
            true => RTC_TIM0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM0_A::CONST_1
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Mask"]
pub type RTC_TIM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, RTC_TIM0_A, O>;
impl<'a, const O: u8> RTC_TIM0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_TIM0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_TIM0_A::CONST_1)
    }
}
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Mask"]
pub type RTC_TIM1_R = crate::BitReader<RTC_TIM1_A>;
#[doc = "RTC TIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<RTC_TIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_TIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM1_A {
        match self.bits {
            false => RTC_TIM1_A::CONST_0,
            true => RTC_TIM1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM1_A::CONST_1
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Mask"]
pub type RTC_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, RTC_TIM1_A, O>;
impl<'a, const O: u8> RTC_TIM1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_TIM1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_TIM1_A::CONST_1)
    }
}
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Mask"]
pub type RMX_R = crate::BitReader<RMX_A>;
#[doc = "Retention Memory Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<RMX_A> for bool {
    #[inline(always)]
    fn from(variant: RMX_A) -> Self {
        variant as u8 != 0
    }
}
impl RMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMX_A {
        match self.bits {
            false => RMX_A::CONST_0,
            true => RMX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RMX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RMX_A::CONST_1
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Mask"]
pub type RMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRMSK_SPEC, RMX_A, O>;
impl<'a, const O: u8> RMX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RMX_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RMX_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DLROVR_R {
        DLROVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PRWARN_W<0> {
        PRWARN_W::new(self)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<1> {
        PI_W::new(self)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<2> {
        AI_W::new(self)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DLROVR_W<3> {
        DLROVR_W::new(self)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdclr(&mut self) -> HDCLR_W<17> {
        HDCLR_W::new(self)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdset(&mut self) -> HDSET_W<18> {
        HDSET_W::new(self)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HDCR_W<19> {
        HDCR_W::new(self)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W<21> {
        OSCSICTRL_W::new(self)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W<23> {
        OSCULCTRL_W::new(self)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W<24> {
        RTC_CTR_W::new(self)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W<25> {
        RTC_ATIM0_W::new(self)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W<26> {
        RTC_ATIM1_W::new(self)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W<27> {
        RTC_TIM0_W::new(self)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W<28> {
        RTC_TIM1_W::new(self)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RMX_W<29> {
        RMX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCU Service Request Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srmsk](index.html) module"]
pub struct SRMSK_SPEC;
impl crate::RegisterSpec for SRMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srmsk::R](R) reader structure"]
impl crate::Readable for SRMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srmsk::W](W) writer structure"]
impl crate::Writable for SRMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRMSK to value 0"]
impl crate::Resettable for SRMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
