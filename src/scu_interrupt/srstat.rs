#[doc = "Register `SRSTAT` reader"]
pub struct R(crate::R<SRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "WDT pre-warning Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Status"]
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
#[doc = "Field `PI` reader - RTC Periodic Interrupt Status"]
pub struct PI_R(crate::FieldReader<bool, bool>);
impl PI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI` reader - Alarm Interrupt Status"]
pub struct AI_R(crate::FieldReader<bool, bool>);
impl AI_R {
    pub(crate) fn new(bits: bool) -> Self {
        AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Status"]
pub struct DLROVR_R(crate::FieldReader<bool, bool>);
impl DLROVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLROVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLROVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HDCLR Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Status"]
pub struct HDCLR_R(crate::FieldReader<bool, HDCLR_A>);
impl HDCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HDCLR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == HDCLR_A::CONST_1
    }
}
impl core::ops::Deref for HDCLR_R {
    type Target = crate::FieldReader<bool, HDCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HDSET Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Status"]
pub struct HDSET_R(crate::FieldReader<bool, HDSET_A>);
impl HDSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HDSET_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == HDSET_A::CONST_1
    }
}
impl core::ops::Deref for HDSET_R {
    type Target = crate::FieldReader<bool, HDSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HDCR Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Status"]
pub struct HDCR_R(crate::FieldReader<bool, HDCR_A>);
impl HDCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HDCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == HDCR_A::CONST_1
    }
}
impl core::ops::Deref for HDCR_R {
    type Target = crate::FieldReader<bool, HDCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OSCSICTRL Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Status"]
pub struct OSCSICTRL_R(crate::FieldReader<bool, OSCSICTRL_A>);
impl OSCSICTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCSICTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == OSCSICTRL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == OSCSICTRL_A::CONST_1
    }
}
impl core::ops::Deref for OSCSICTRL_R {
    type Target = crate::FieldReader<bool, OSCSICTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OSCULCTRL Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Status"]
pub struct OSCULCTRL_R(crate::FieldReader<bool, OSCULCTRL_A>);
impl OSCULCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCULCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == OSCULCTRL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == OSCULCTRL_A::CONST_1
    }
}
impl core::ops::Deref for OSCULCTRL_R {
    type Target = crate::FieldReader<bool, OSCULCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC CTR Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Status"]
pub struct RTC_CTR_R(crate::FieldReader<bool, RTC_CTR_A>);
impl RTC_CTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_CTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RTC_CTR_A::CONST_1
    }
}
impl core::ops::Deref for RTC_CTR_R {
    type Target = crate::FieldReader<bool, RTC_CTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Status"]
pub struct RTC_ATIM0_R(crate::FieldReader<bool, RTC_ATIM0_A>);
impl RTC_ATIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ATIM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_ATIM0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RTC_ATIM0_A::CONST_1
    }
}
impl core::ops::Deref for RTC_ATIM0_R {
    type Target = crate::FieldReader<bool, RTC_ATIM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Status"]
pub struct RTC_ATIM1_R(crate::FieldReader<bool, RTC_ATIM1_A>);
impl RTC_ATIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ATIM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_ATIM1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RTC_ATIM1_A::CONST_1
    }
}
impl core::ops::Deref for RTC_ATIM1_R {
    type Target = crate::FieldReader<bool, RTC_ATIM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC TIM0 Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Status"]
pub struct RTC_TIM0_R(crate::FieldReader<bool, RTC_TIM0_A>);
impl RTC_TIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TIM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_TIM0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RTC_TIM0_A::CONST_1
    }
}
impl core::ops::Deref for RTC_TIM0_R {
    type Target = crate::FieldReader<bool, RTC_TIM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC TIM1 Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Status"]
pub struct RTC_TIM1_R(crate::FieldReader<bool, RTC_TIM1_A>);
impl RTC_TIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TIM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_TIM1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RTC_TIM1_A::CONST_1
    }
}
impl core::ops::Deref for RTC_TIM1_R {
    type Target = crate::FieldReader<bool, RTC_TIM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Retention Memory Mirror Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Status"]
pub struct RMX_R(crate::FieldReader<bool, RMX_A>);
impl RMX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RMX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RMX_A::CONST_1
    }
}
impl core::ops::Deref for RMX_R {
    type Target = crate::FieldReader<bool, RMX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Status"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Status"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alarm Interrupt Status"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Status"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DLROVR_R {
        DLROVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Status"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Status"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Status"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Status"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Status"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Status"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "SCU Service Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srstat](index.html) module"]
pub struct SRSTAT_SPEC;
impl crate::RegisterSpec for SRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srstat::R](R) reader structure"]
impl crate::Readable for SRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRSTAT to value 0"]
impl crate::Resettable for SRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
