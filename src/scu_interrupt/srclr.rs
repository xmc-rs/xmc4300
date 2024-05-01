#[doc = "Register `SRCLR` writer"]
pub type W = crate::W<SrclrSpec>;
#[doc = "WDT pre-warning Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prwarn {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Prwarn> for bool {
    #[inline(always)]
    fn from(variant: Prwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Const1)
    }
}
#[doc = "RTC Periodic Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pi {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Pi> for bool {
    #[inline(always)]
    fn from(variant: Pi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Const1)
    }
}
#[doc = "RTC Alarm Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ai {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Ai> for bool {
    #[inline(always)]
    fn from(variant: Ai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Const1)
    }
}
#[doc = "DLR Request Overrun Interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlrovr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Dlrovr> for bool {
    #[inline(always)]
    fn from(variant: Dlrovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlrovr::Const1)
    }
}
#[doc = "HDCLR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdclr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Hdclr> for bool {
    #[inline(always)]
    fn from(variant: Hdclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Clear"]
pub type HdclrW<'a, REG> = crate::BitWriter<'a, REG, Hdclr>;
impl<'a, REG> HdclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdclr::Const0)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdclr::Const1)
    }
}
#[doc = "HDSET Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdset {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Hdset> for bool {
    #[inline(always)]
    fn from(variant: Hdset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Clear"]
pub type HdsetW<'a, REG> = crate::BitWriter<'a, REG, Hdset>;
impl<'a, REG> HdsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdset::Const0)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdset::Const1)
    }
}
#[doc = "HDCR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Hdcr> for bool {
    #[inline(always)]
    fn from(variant: Hdcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcr::Const1)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscsictrl {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Oscsictrl> for bool {
    #[inline(always)]
    fn from(variant: Oscsictrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsictrl::Const1)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osculctrl {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Osculctrl> for bool {
    #[inline(always)]
    fn from(variant: Osculctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Osculctrl::Const1)
    }
}
#[doc = "RTC CTR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcCtr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<RtcCtr> for bool {
    #[inline(always)]
    fn from(variant: RtcCtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCtr::Const1)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<RtcAtim0> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim0::Const1)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim1 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<RtcAtim1> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim1::Const1)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<RtcTim0> for bool {
    #[inline(always)]
    fn from(variant: RtcTim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim0::Const1)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim1 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<RtcTim1> for bool {
    #[inline(always)]
    fn from(variant: RtcTim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim1::Const1)
    }
}
#[doc = "Retention Memory Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmx {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Clear the status bit"]
    Const1 = 1,
}
impl From<Rmx> for bool {
    #[inline(always)]
    fn from(variant: Rmx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Clear"]
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
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmx::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PrwarnW<SrclrSpec> {
        PrwarnW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PiW<SrclrSpec> {
        PiW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AiW<SrclrSpec> {
        AiW::new(self, 2)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DlrovrW<SrclrSpec> {
        DlrovrW::new(self, 3)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdclr(&mut self) -> HdclrW<SrclrSpec> {
        HdclrW::new(self, 17)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdset(&mut self) -> HdsetW<SrclrSpec> {
        HdsetW::new(self, 18)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HdcrW<SrclrSpec> {
        HdcrW::new(self, 19)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OscsictrlW<SrclrSpec> {
        OscsictrlW::new(self, 21)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OsculctrlW<SrclrSpec> {
        OsculctrlW::new(self, 23)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RtcCtrW<SrclrSpec> {
        RtcCtrW::new(self, 24)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RtcAtim0W<SrclrSpec> {
        RtcAtim0W::new(self, 25)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RtcAtim1W<SrclrSpec> {
        RtcAtim1W::new(self, 26)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RtcTim0W<SrclrSpec> {
        RtcTim0W::new(self, 27)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RtcTim1W<SrclrSpec> {
        RtcTim1W::new(self, 28)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RmxW<SrclrSpec> {
        RmxW::new(self, 29)
    }
}
#[doc = "SCU Service Request Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrclrSpec;
impl crate::RegisterSpec for SrclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srclr::W`](W) writer structure"]
impl crate::Writable for SrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCLR to value 0"]
impl crate::Resettable for SrclrSpec {
    const RESET_VALUE: u32 = 0;
}
