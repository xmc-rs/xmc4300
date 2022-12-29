#[doc = "Register `SRRAW` reader"]
pub struct R(crate::R<SRRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Status Before Masking"]
pub type PRWARN_R = crate::BitReader<PRWARN_A>;
#[doc = "WDT pre-warning Interrupt Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_A {
    #[doc = "0: Inactive"]
    CONST_0 = 0,
    #[doc = "1: Active"]
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
#[doc = "Field `PI` reader - RTC Raw Periodic Interrupt Status Before Masking"]
pub type PI_R = crate::BitReader<bool>;
#[doc = "Field `AI` reader - RTC Raw Alarm Interrupt Status Before Masking"]
pub type AI_R = crate::BitReader<bool>;
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Status Before Masking"]
pub type DLROVR_R = crate::BitReader<bool>;
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Status Before Masking"]
pub type HDCLR_R = crate::BitReader<HDCLR_A>;
#[doc = "HDCLR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Status Before Masking"]
pub type HDSET_R = crate::BitReader<HDSET_A>;
#[doc = "HDSET Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Status Before Masking"]
pub type HDCR_R = crate::BitReader<HDCR_A>;
#[doc = "HDCR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Status Before Masking"]
pub type OSCSICTRL_R = crate::BitReader<OSCSICTRL_A>;
#[doc = "OSCSICTRL Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Status Before Masking"]
pub type OSCULCTRL_R = crate::BitReader<OSCULCTRL_A>;
#[doc = "OSCULCTRL Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Status Before Masking"]
pub type RTC_CTR_R = crate::BitReader<RTC_CTR_A>;
#[doc = "RTC CTR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Status Before Masking"]
pub type RTC_ATIM0_R = crate::BitReader<RTC_ATIM0_A>;
#[doc = "RTC ATIM0 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Status Before Masking"]
pub type RTC_ATIM1_R = crate::BitReader<RTC_ATIM1_A>;
#[doc = "RTC ATIM1 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Before Masking Status"]
pub type RTC_TIM0_R = crate::BitReader<RTC_TIM0_A>;
#[doc = "RTC TIM0 Mirror Register Update Before Masking Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Status Before Masking"]
pub type RTC_TIM1_R = crate::BitReader<RTC_TIM1_A>;
#[doc = "RTC TIM1 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Status Before Masking"]
pub type RMX_R = crate::BitReader<RMX_A>;
#[doc = "Retention Memory Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_A {
    #[doc = "0: Not updated"]
    CONST_0 = 0,
    #[doc = "1: Update completed"]
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
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Raw Periodic Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Raw Alarm Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DLROVR_R {
        DLROVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Before Masking Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "SCU Raw Service Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srraw](index.html) module"]
pub struct SRRAW_SPEC;
impl crate::RegisterSpec for SRRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srraw::R](R) reader structure"]
impl crate::Readable for SRRAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRRAW to value 0"]
impl crate::Resettable for SRRAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
