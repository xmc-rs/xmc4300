#[doc = "Register `MIRRSTS` reader"]
pub struct R(crate::R<MIRRSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIRRSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIRRSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIRRSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "HDCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCLR_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<HDCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Write Status"]
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
#[doc = "HDSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSET_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<HDSET_A> for bool {
    #[inline(always)]
    fn from(variant: HDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` reader - HDSET Mirror Register Write Status"]
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
#[doc = "HDCR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCR_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<HDCR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` reader - HDCR Mirror Register Write Status"]
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
#[doc = "OSCSICTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<OSCSICTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Write Status"]
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
#[doc = "OSCULCTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<OSCULCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Write Status"]
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
#[doc = "RTC CTR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CTR_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RTC_CTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Write Status"]
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
#[doc = "RTC ATIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RTC_ATIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Write Status"]
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
#[doc = "RTC ATIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RTC_ATIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Write Status"]
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
#[doc = "RTC TIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RTC_TIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Write Status"]
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
#[doc = "RTC TIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RTC_TIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Write Status"]
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
#[doc = "Retention Memory Access Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMX_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RMX_A> for bool {
    #[inline(always)]
    fn from(variant: RMX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` reader - Retention Memory Access Register Update Status"]
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
#[doc = "RTC MSKSSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_MSKSR_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RTC_MSKSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_MSKSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_MSKSR` reader - RTC MSKSSR Mirror Register Write Status"]
pub struct RTC_MSKSR_R(crate::FieldReader<bool, RTC_MSKSR_A>);
impl RTC_MSKSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MSKSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_MSKSR_A {
        match self.bits {
            false => RTC_MSKSR_A::CONST_0,
            true => RTC_MSKSR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == RTC_MSKSR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RTC_MSKSR_A::CONST_1
    }
}
impl core::ops::Deref for RTC_MSKSR_R {
    type Target = crate::FieldReader<bool, RTC_MSKSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC CLRSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CLRSR_A {
    #[doc = "0: Ready"]
    CONST_0 = 0,
    #[doc = "1: Busy"]
    CONST_1 = 1,
}
impl From<RTC_CLRSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CLRSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CLRSR` reader - RTC CLRSR Mirror Register Write Status"]
pub struct RTC_CLRSR_R(crate::FieldReader<bool, RTC_CLRSR_A>);
impl RTC_CLRSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CLRSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CLRSR_A {
        match self.bits {
            false => RTC_CLRSR_A::CONST_0,
            true => RTC_CLRSR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == RTC_CLRSR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RTC_CLRSR_A::CONST_1
    }
}
impl core::ops::Deref for RTC_CLRSR_R {
    type Target = crate::FieldReader<bool, RTC_CLRSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - HDCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HDSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HDCR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OSCSICTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OSCULCTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTC CTR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC ATIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC ATIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RTC TIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RTC TIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Retention Memory Access Register Update Status"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTC MSKSSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_msksr(&self) -> RTC_MSKSR_R {
        RTC_MSKSR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC CLRSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_clrsr(&self) -> RTC_CLRSR_R {
        RTC_CLRSR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Mirror Write Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mirrsts](index.html) module"]
pub struct MIRRSTS_SPEC;
impl crate::RegisterSpec for MIRRSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mirrsts::R](R) reader structure"]
impl crate::Readable for MIRRSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIRRSTS to value 0"]
impl crate::Resettable for MIRRSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
