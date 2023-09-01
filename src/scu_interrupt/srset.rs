#[doc = "Register `SRSET` writer"]
pub type W = crate::W<SRSET_SPEC>;
#[doc = "WDT pre-warning Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<PRWARN_AW> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Set"]
pub type PRWARN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRWARN_AW>;
impl<'a, REG, const O: u8> PRWARN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_AW::CONST_1)
    }
}
#[doc = "RTC Periodic Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<PI_AW> for bool {
    #[inline(always)]
    fn from(variant: PI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Set"]
pub type PI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PI_AW>;
impl<'a, REG, const O: u8> PI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PI_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PI_AW::CONST_1)
    }
}
#[doc = "RTC Alarm Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<AI_AW> for bool {
    #[inline(always)]
    fn from(variant: AI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Set"]
pub type AI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AI_AW>;
impl<'a, REG, const O: u8> AI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(AI_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(AI_AW::CONST_1)
    }
}
#[doc = "DLR Request Overrun Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLROVR_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<DLROVR_AW> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt Set"]
pub type DLROVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DLROVR_AW>;
impl<'a, REG, const O: u8> DLROVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_AW::CONST_1)
    }
}
#[doc = "HDCRCLR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCRCLR_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<HDCRCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCRCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCRCLR` writer - HDCRCLR Mirror Register Update Set"]
pub type HDCRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HDCRCLR_AW>;
impl<'a, REG, const O: u8> HDCRCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HDCRCLR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCRCLR_AW::CONST_1)
    }
}
#[doc = "HDCRSET Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCRSET_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<HDCRSET_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCRSET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCRSET` writer - HDCRSET Mirror Register Update Set"]
pub type HDCRSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HDCRSET_AW>;
impl<'a, REG, const O: u8> HDCRSET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HDCRSET_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCRSET_AW::CONST_1)
    }
}
#[doc = "HDCR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<HDCR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Set"]
pub type HDCR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HDCR_AW>;
impl<'a, REG, const O: u8> HDCR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_AW::CONST_1)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<OSCSICTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Set"]
pub type OSCSICTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCSICTRL_AW>;
impl<'a, REG, const O: u8> OSCSICTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_AW::CONST_1)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<OSCULCTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Set"]
pub type OSCULCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCULCTRL_AW>;
impl<'a, REG, const O: u8> OSCULCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_AW::CONST_1)
    }
}
#[doc = "RTC CTR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<RTC_CTR_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Set"]
pub type RTC_CTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_CTR_AW>;
impl<'a, REG, const O: u8> RTC_CTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_AW::CONST_1)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<RTC_ATIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Set"]
pub type RTC_ATIM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_ATIM0_AW>;
impl<'a, REG, const O: u8> RTC_ATIM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_AW::CONST_1)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<RTC_ATIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Set"]
pub type RTC_ATIM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_ATIM1_AW>;
impl<'a, REG, const O: u8> RTC_ATIM1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_AW::CONST_1)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<RTC_TIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Set"]
pub type RTC_TIM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_TIM0_AW>;
impl<'a, REG, const O: u8> RTC_TIM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_AW::CONST_1)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<RTC_TIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Set"]
pub type RTC_TIM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_TIM1_AW>;
impl<'a, REG, const O: u8> RTC_TIM1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_AW::CONST_1)
    }
}
#[doc = "Retention Memory Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: set the status bit"]
    CONST_1 = 1,
}
impl From<RMX_AW> for bool {
    #[inline(always)]
    fn from(variant: RMX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Set"]
pub type RMX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RMX_AW>;
impl<'a, REG, const O: u8> RMX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_AW::CONST_0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PRWARN_W<SRSET_SPEC, 0> {
        PRWARN_W::new(self)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<SRSET_SPEC, 1> {
        PI_W::new(self)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<SRSET_SPEC, 2> {
        AI_W::new(self)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DLROVR_W<SRSET_SPEC, 3> {
        DLROVR_W::new(self)
    }
    #[doc = "Bit 17 - HDCRCLR Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn hdcrclr(&mut self) -> HDCRCLR_W<SRSET_SPEC, 17> {
        HDCRCLR_W::new(self)
    }
    #[doc = "Bit 18 - HDCRSET Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn hdcrset(&mut self) -> HDCRSET_W<SRSET_SPEC, 18> {
        HDCRSET_W::new(self)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HDCR_W<SRSET_SPEC, 19> {
        HDCR_W::new(self)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W<SRSET_SPEC, 21> {
        OSCSICTRL_W::new(self)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W<SRSET_SPEC, 23> {
        OSCULCTRL_W::new(self)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W<SRSET_SPEC, 24> {
        RTC_CTR_W::new(self)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W<SRSET_SPEC, 25> {
        RTC_ATIM0_W::new(self)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W<SRSET_SPEC, 26> {
        RTC_ATIM1_W::new(self)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W<SRSET_SPEC, 27> {
        RTC_TIM0_W::new(self)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W<SRSET_SPEC, 28> {
        RTC_TIM1_W::new(self)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RMX_W<SRSET_SPEC, 29> {
        RMX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SCU Service Request Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSET_SPEC;
impl crate::RegisterSpec for SRSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srset::W`](W) writer structure"]
impl crate::Writable for SRSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSET to value 0"]
impl crate::Resettable for SRSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
