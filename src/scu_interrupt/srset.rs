#[doc = "Register `SRSET` writer"]
pub type W = crate::W<SrsetSpec>;
#[doc = "WDT pre-warning Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prwarn {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Prwarn> for bool {
    #[inline(always)]
    fn from(variant: Prwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Set"]
pub type PrwarnW<'a, REG> = crate::BitWriter<'a, REG, Prwarn>;
impl<'a, REG> PrwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Const1)
    }
}
#[doc = "RTC Periodic Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pi {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Pi> for bool {
    #[inline(always)]
    fn from(variant: Pi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Set"]
pub type PiW<'a, REG> = crate::BitWriter<'a, REG, Pi>;
impl<'a, REG> PiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Const1)
    }
}
#[doc = "RTC Alarm Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ai {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Ai> for bool {
    #[inline(always)]
    fn from(variant: Ai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Set"]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG, Ai>;
impl<'a, REG> AiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Const1)
    }
}
#[doc = "DLR Request Overrun Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlrovr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Dlrovr> for bool {
    #[inline(always)]
    fn from(variant: Dlrovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt Set"]
pub type DlrovrW<'a, REG> = crate::BitWriter<'a, REG, Dlrovr>;
impl<'a, REG> DlrovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlrovr::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlrovr::Const1)
    }
}
#[doc = "HDCRCLR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcrclr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Hdcrclr> for bool {
    #[inline(always)]
    fn from(variant: Hdcrclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCRCLR` writer - HDCRCLR Mirror Register Update Set"]
pub type HdcrclrW<'a, REG> = crate::BitWriter<'a, REG, Hdcrclr>;
impl<'a, REG> HdcrclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcrclr::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcrclr::Const1)
    }
}
#[doc = "HDCRSET Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcrset {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Hdcrset> for bool {
    #[inline(always)]
    fn from(variant: Hdcrset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCRSET` writer - HDCRSET Mirror Register Update Set"]
pub type HdcrsetW<'a, REG> = crate::BitWriter<'a, REG, Hdcrset>;
impl<'a, REG> HdcrsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcrset::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcrset::Const1)
    }
}
#[doc = "HDCR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Hdcr> for bool {
    #[inline(always)]
    fn from(variant: Hdcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Set"]
pub type HdcrW<'a, REG> = crate::BitWriter<'a, REG, Hdcr>;
impl<'a, REG> HdcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcr::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcr::Const1)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscsictrl {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Oscsictrl> for bool {
    #[inline(always)]
    fn from(variant: Oscsictrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Set"]
pub type OscsictrlW<'a, REG> = crate::BitWriter<'a, REG, Oscsictrl>;
impl<'a, REG> OscsictrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsictrl::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsictrl::Const1)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osculctrl {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Osculctrl> for bool {
    #[inline(always)]
    fn from(variant: Osculctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Set"]
pub type OsculctrlW<'a, REG> = crate::BitWriter<'a, REG, Osculctrl>;
impl<'a, REG> OsculctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Osculctrl::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Osculctrl::Const1)
    }
}
#[doc = "RTC CTR Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcCtr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<RtcCtr> for bool {
    #[inline(always)]
    fn from(variant: RtcCtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Set"]
pub type RtcCtrW<'a, REG> = crate::BitWriter<'a, REG, RtcCtr>;
impl<'a, REG> RtcCtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCtr::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCtr::Const1)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<RtcAtim0> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Set"]
pub type RtcAtim0W<'a, REG> = crate::BitWriter<'a, REG, RtcAtim0>;
impl<'a, REG> RtcAtim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim0::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim0::Const1)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim1 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<RtcAtim1> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Set"]
pub type RtcAtim1W<'a, REG> = crate::BitWriter<'a, REG, RtcAtim1>;
impl<'a, REG> RtcAtim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim1::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim1::Const1)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<RtcTim0> for bool {
    #[inline(always)]
    fn from(variant: RtcTim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Set"]
pub type RtcTim0W<'a, REG> = crate::BitWriter<'a, REG, RtcTim0>;
impl<'a, REG> RtcTim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim0::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim0::Const1)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim1 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<RtcTim1> for bool {
    #[inline(always)]
    fn from(variant: RtcTim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Set"]
pub type RtcTim1W<'a, REG> = crate::BitWriter<'a, REG, RtcTim1>;
impl<'a, REG> RtcTim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim1::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim1::Const1)
    }
}
#[doc = "Retention Memory Mirror Register Update Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmx {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: set the status bit"]
    Const1 = 1,
}
impl From<Rmx> for bool {
    #[inline(always)]
    fn from(variant: Rmx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Set"]
pub type RmxW<'a, REG> = crate::BitWriter<'a, REG, Rmx>;
impl<'a, REG> RmxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmx::Const0)
    }
    #[doc = "set the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmx::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PrwarnW<SrsetSpec> {
        PrwarnW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PiW<SrsetSpec> {
        PiW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AiW<SrsetSpec> {
        AiW::new(self, 2)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DlrovrW<SrsetSpec> {
        DlrovrW::new(self, 3)
    }
    #[doc = "Bit 17 - HDCRCLR Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn hdcrclr(&mut self) -> HdcrclrW<SrsetSpec> {
        HdcrclrW::new(self, 17)
    }
    #[doc = "Bit 18 - HDCRSET Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn hdcrset(&mut self) -> HdcrsetW<SrsetSpec> {
        HdcrsetW::new(self, 18)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HdcrW<SrsetSpec> {
        HdcrW::new(self, 19)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OscsictrlW<SrsetSpec> {
        OscsictrlW::new(self, 21)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OsculctrlW<SrsetSpec> {
        OsculctrlW::new(self, 23)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RtcCtrW<SrsetSpec> {
        RtcCtrW::new(self, 24)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RtcAtim0W<SrsetSpec> {
        RtcAtim0W::new(self, 25)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RtcAtim1W<SrsetSpec> {
        RtcAtim1W::new(self, 26)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RtcTim0W<SrsetSpec> {
        RtcTim0W::new(self, 27)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RtcTim1W<SrsetSpec> {
        RtcTim1W::new(self, 28)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Set"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RmxW<SrsetSpec> {
        RmxW::new(self, 29)
    }
}
#[doc = "SCU Service Request Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrsetSpec;
impl crate::RegisterSpec for SrsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srset::W`](W) writer structure"]
impl crate::Writable for SrsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSET to value 0"]
impl crate::Resettable for SrsetSpec {
    const RESET_VALUE: u32 = 0;
}
