#[doc = "Register `SRMSK` reader"]
pub type R = crate::R<SRMSK_SPEC>;
#[doc = "Register `SRMSK` writer"]
pub type W = crate::W<SRMSK_SPEC>;
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
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Mask"]
pub type PRWARN_R = crate::BitReader<PRWARN_A>;
impl PRWARN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRWARN_A {
        match self.bits {
            false => PRWARN_A::CONST_0,
            true => PRWARN_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PRWARN_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PRWARN_A::CONST_1
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Mask"]
pub type PRWARN_W<'a, REG> = crate::BitWriter<'a, REG, PRWARN_A>;
impl<'a, REG> PRWARN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::CONST_1)
    }
}
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
#[doc = "Field `PI` reader - RTC Periodic Interrupt Mask"]
pub type PI_R = crate::BitReader<PI_A>;
impl PI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::CONST_0,
            true => PI_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PI_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PI_A::CONST_1
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Mask"]
pub type PI_W<'a, REG> = crate::BitWriter<'a, REG, PI_A>;
impl<'a, REG> PI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::CONST_1)
    }
}
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
#[doc = "Field `AI` reader - RTC Alarm Interrupt Mask"]
pub type AI_R = crate::BitReader<AI_A>;
impl AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AI_A {
        match self.bits {
            false => AI_A::CONST_0,
            true => AI_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == AI_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == AI_A::CONST_1
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Mask"]
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG, AI_A>;
impl<'a, REG> AI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::CONST_1)
    }
}
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
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Mask"]
pub type DLROVR_R = crate::BitReader<DLROVR_A>;
impl DLROVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLROVR_A {
        match self.bits {
            false => DLROVR_A::CONST_0,
            true => DLROVR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == DLROVR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == DLROVR_A::CONST_1
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt Mask"]
pub type DLROVR_W<'a, REG> = crate::BitWriter<'a, REG, DLROVR_A>;
impl<'a, REG> DLROVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_A::CONST_1)
    }
}
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
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Mask"]
pub type HDCLR_R = crate::BitReader<HDCLR_A>;
impl HDCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDCLR_A {
        match self.bits {
            false => HDCLR_A::CONST_0,
            true => HDCLR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDCLR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDCLR_A::CONST_1
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Mask"]
pub type HDCLR_W<'a, REG> = crate::BitWriter<'a, REG, HDCLR_A>;
impl<'a, REG> HDCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_A::CONST_1)
    }
}
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
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Mask"]
pub type HDSET_R = crate::BitReader<HDSET_A>;
impl HDSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDSET_A {
        match self.bits {
            false => HDSET_A::CONST_0,
            true => HDSET_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDSET_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDSET_A::CONST_1
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Mask"]
pub type HDSET_W<'a, REG> = crate::BitWriter<'a, REG, HDSET_A>;
impl<'a, REG> HDSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_A::CONST_1)
    }
}
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
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Mask"]
pub type HDCR_R = crate::BitReader<HDCR_A>;
impl HDCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDCR_A {
        match self.bits {
            false => HDCR_A::CONST_0,
            true => HDCR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDCR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDCR_A::CONST_1
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Mask"]
pub type HDCR_W<'a, REG> = crate::BitWriter<'a, REG, HDCR_A>;
impl<'a, REG> HDCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_A::CONST_1)
    }
}
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
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Mask"]
pub type OSCSICTRL_R = crate::BitReader<OSCSICTRL_A>;
impl OSCSICTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCSICTRL_A {
        match self.bits {
            false => OSCSICTRL_A::CONST_0,
            true => OSCSICTRL_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCSICTRL_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCSICTRL_A::CONST_1
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Mask"]
pub type OSCSICTRL_W<'a, REG> = crate::BitWriter<'a, REG, OSCSICTRL_A>;
impl<'a, REG> OSCSICTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_A::CONST_1)
    }
}
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
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Mask"]
pub type OSCULCTRL_R = crate::BitReader<OSCULCTRL_A>;
impl OSCULCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCULCTRL_A {
        match self.bits {
            false => OSCULCTRL_A::CONST_0,
            true => OSCULCTRL_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCULCTRL_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCULCTRL_A::CONST_1
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Mask"]
pub type OSCULCTRL_W<'a, REG> = crate::BitWriter<'a, REG, OSCULCTRL_A>;
impl<'a, REG> OSCULCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_A::CONST_1)
    }
}
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
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Mask"]
pub type RTC_CTR_R = crate::BitReader<RTC_CTR_A>;
impl RTC_CTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_CTR_A {
        match self.bits {
            false => RTC_CTR_A::CONST_0,
            true => RTC_CTR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_CTR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_CTR_A::CONST_1
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Mask"]
pub type RTC_CTR_W<'a, REG> = crate::BitWriter<'a, REG, RTC_CTR_A>;
impl<'a, REG> RTC_CTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_A::CONST_1)
    }
}
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
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Mask"]
pub type RTC_ATIM0_R = crate::BitReader<RTC_ATIM0_A>;
impl RTC_ATIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_ATIM0_A {
        match self.bits {
            false => RTC_ATIM0_A::CONST_0,
            true => RTC_ATIM0_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM0_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM0_A::CONST_1
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Mask"]
pub type RTC_ATIM0_W<'a, REG> = crate::BitWriter<'a, REG, RTC_ATIM0_A>;
impl<'a, REG> RTC_ATIM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_A::CONST_1)
    }
}
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
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Mask"]
pub type RTC_ATIM1_R = crate::BitReader<RTC_ATIM1_A>;
impl RTC_ATIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_ATIM1_A {
        match self.bits {
            false => RTC_ATIM1_A::CONST_0,
            true => RTC_ATIM1_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM1_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM1_A::CONST_1
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Mask"]
pub type RTC_ATIM1_W<'a, REG> = crate::BitWriter<'a, REG, RTC_ATIM1_A>;
impl<'a, REG> RTC_ATIM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_A::CONST_1)
    }
}
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
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Mask"]
pub type RTC_TIM0_R = crate::BitReader<RTC_TIM0_A>;
impl RTC_TIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_TIM0_A {
        match self.bits {
            false => RTC_TIM0_A::CONST_0,
            true => RTC_TIM0_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM0_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM0_A::CONST_1
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Mask"]
pub type RTC_TIM0_W<'a, REG> = crate::BitWriter<'a, REG, RTC_TIM0_A>;
impl<'a, REG> RTC_TIM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_A::CONST_1)
    }
}
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
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Mask"]
pub type RTC_TIM1_R = crate::BitReader<RTC_TIM1_A>;
impl RTC_TIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_TIM1_A {
        match self.bits {
            false => RTC_TIM1_A::CONST_0,
            true => RTC_TIM1_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM1_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM1_A::CONST_1
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Mask"]
pub type RTC_TIM1_W<'a, REG> = crate::BitWriter<'a, REG, RTC_TIM1_A>;
impl<'a, REG> RTC_TIM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_A::CONST_1)
    }
}
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
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Mask"]
pub type RMX_R = crate::BitReader<RMX_A>;
impl RMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMX_A {
        match self.bits {
            false => RMX_A::CONST_0,
            true => RMX_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RMX_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RMX_A::CONST_1
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Mask"]
pub type RMX_W<'a, REG> = crate::BitWriter<'a, REG, RMX_A>;
impl<'a, REG> RMX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
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
    pub fn prwarn(&mut self) -> PRWARN_W<SRMSK_SPEC> {
        PRWARN_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    pub fn pi(&mut self) -> PI_W<SRMSK_SPEC> {
        PI_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W<SRMSK_SPEC> {
        AI_W::new(self, 2)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn dlrovr(&mut self) -> DLROVR_W<SRMSK_SPEC> {
        DLROVR_W::new(self, 3)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdclr(&mut self) -> HDCLR_W<SRMSK_SPEC> {
        HDCLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdset(&mut self) -> HDSET_W<SRMSK_SPEC> {
        HDSET_W::new(self, 18)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdcr(&mut self) -> HDCR_W<SRMSK_SPEC> {
        HDCR_W::new(self, 19)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W<SRMSK_SPEC> {
        OSCSICTRL_W::new(self, 21)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W<SRMSK_SPEC> {
        OSCULCTRL_W::new(self, 23)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W<SRMSK_SPEC> {
        RTC_CTR_W::new(self, 24)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W<SRMSK_SPEC> {
        RTC_ATIM0_W::new(self, 25)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W<SRMSK_SPEC> {
        RTC_ATIM1_W::new(self, 26)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W<SRMSK_SPEC> {
        RTC_TIM0_W::new(self, 27)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W<SRMSK_SPEC> {
        RTC_TIM1_W::new(self, 28)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rmx(&mut self) -> RMX_W<SRMSK_SPEC> {
        RMX_W::new(self, 29)
    }
}
#[doc = "SCU Service Request Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`srmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRMSK_SPEC;
impl crate::RegisterSpec for SRMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srmsk::R`](R) reader structure"]
impl crate::Readable for SRMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srmsk::W`](W) writer structure"]
impl crate::Writable for SRMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRMSK to value 0"]
impl crate::Resettable for SRMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
