#[doc = "Register `DSLEEPCR` reader"]
pub type R = crate::R<DsleepcrSpec>;
#[doc = "Register `DSLEEPCR` writer"]
pub type W = crate::W<DsleepcrSpec>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syssel {
    #[doc = "0: fOFI clock"]
    Const0 = 0,
    #[doc = "1: fPLL clock"]
    Const1 = 1,
}
impl From<Syssel> for bool {
    #[inline(always)]
    fn from(variant: Syssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SysselR = crate::BitReader<Syssel>;
impl SysselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syssel {
        match self.bits {
            false => Syssel::Const0,
            true => Syssel::Const1,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Syssel::Const0
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Syssel::Const1
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SysselW<'a, REG> = crate::BitWriter<'a, REG, Syssel>;
impl<'a, REG> SysselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Const0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Const1)
    }
}
#[doc = "Flash Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpdn {
    #[doc = "1: Flash power down module"]
    Const1 = 1,
    #[doc = "0: No effect"]
    Const0 = 0,
}
impl From<Fpdn> for bool {
    #[inline(always)]
    fn from(variant: Fpdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDN` reader - Flash Power Down"]
pub type FpdnR = crate::BitReader<Fpdn>;
impl FpdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpdn {
        match self.bits {
            true => Fpdn::Const1,
            false => Fpdn::Const0,
        }
    }
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Fpdn::Const1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Fpdn::Const0
    }
}
#[doc = "Field `FPDN` writer - Flash Power Down"]
pub type FpdnW<'a, REG> = crate::BitWriter<'a, REG, Fpdn>;
impl<'a, REG> FpdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fpdn::Const1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fpdn::Const0)
    }
}
#[doc = "PLL Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllpdn {
    #[doc = "1: Switch off main PLL"]
    Const1 = 1,
    #[doc = "0: No effect"]
    Const0 = 0,
}
impl From<Pllpdn> for bool {
    #[inline(always)]
    fn from(variant: Pllpdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPDN` reader - PLL Power Down"]
pub type PllpdnR = crate::BitReader<Pllpdn>;
impl PllpdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllpdn {
        match self.bits {
            true => Pllpdn::Const1,
            false => Pllpdn::Const0,
        }
    }
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pllpdn::Const1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pllpdn::Const0
    }
}
#[doc = "Field `PLLPDN` writer - PLL Power Down"]
pub type PllpdnW<'a, REG> = crate::BitWriter<'a, REG, Pllpdn>;
impl<'a, REG> PllpdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdn::Const1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdn::Const0)
    }
}
#[doc = "VCO Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcopdn {
    #[doc = "1: Switch off VCO of main PLL"]
    Const1 = 1,
    #[doc = "0: No effect"]
    Const0 = 0,
}
impl From<Vcopdn> for bool {
    #[inline(always)]
    fn from(variant: Vcopdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPDN` reader - VCO Power Down"]
pub type VcopdnR = crate::BitReader<Vcopdn>;
impl VcopdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcopdn {
        match self.bits {
            true => Vcopdn::Const1,
            false => Vcopdn::Const0,
        }
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Vcopdn::Const1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Vcopdn::Const0
    }
}
#[doc = "Field `VCOPDN` writer - VCO Power Down"]
pub type VcopdnW<'a, REG> = crate::BitWriter<'a, REG, Vcopdn>;
impl<'a, REG> VcopdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopdn::Const1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopdn::Const0)
    }
}
#[doc = "USB Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcr {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Usbcr> for bool {
    #[inline(always)]
    fn from(variant: Usbcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCR` reader - USB Clock Control in Deep Sleep Mode"]
pub type UsbcrR = crate::BitReader<Usbcr>;
impl UsbcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbcr {
        match self.bits {
            false => Usbcr::Const0,
            true => Usbcr::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usbcr::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usbcr::Const1
    }
}
#[doc = "Field `USBCR` writer - USB Clock Control in Deep Sleep Mode"]
pub type UsbcrW<'a, REG> = crate::BitWriter<'a, REG, Usbcr>;
impl<'a, REG> UsbcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcr::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcr::Const1)
    }
}
#[doc = "MMC Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmccr {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Mmccr> for bool {
    #[inline(always)]
    fn from(variant: Mmccr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCR` reader - MMC Clock Control in Deep Sleep Mode"]
pub type MmccrR = crate::BitReader<Mmccr>;
impl MmccrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmccr {
        match self.bits {
            false => Mmccr::Const0,
            true => Mmccr::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Mmccr::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Mmccr::Const1
    }
}
#[doc = "Field `MMCCR` writer - MMC Clock Control in Deep Sleep Mode"]
pub type MmccrW<'a, REG> = crate::BitWriter<'a, REG, Mmccr>;
impl<'a, REG> MmccrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccr::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccr::Const1)
    }
}
#[doc = "Ethernet Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0cr {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Eth0cr> for bool {
    #[inline(always)]
    fn from(variant: Eth0cr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CR` reader - Ethernet Clock Control in Deep Sleep Mode"]
pub type Eth0crR = crate::BitReader<Eth0cr>;
impl Eth0crR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eth0cr {
        match self.bits {
            false => Eth0cr::Const0,
            true => Eth0cr::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Eth0cr::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Eth0cr::Const1
    }
}
#[doc = "Field `ETH0CR` writer - Ethernet Clock Control in Deep Sleep Mode"]
pub type Eth0crW<'a, REG> = crate::BitWriter<'a, REG, Eth0cr>;
impl<'a, REG> Eth0crW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cr::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cr::Const1)
    }
}
#[doc = "CCU Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucr {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Ccucr> for bool {
    #[inline(always)]
    fn from(variant: Ccucr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCR` reader - CCU Clock Control in Deep Sleep Mode"]
pub type CcucrR = crate::BitReader<Ccucr>;
impl CcucrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccucr {
        match self.bits {
            false => Ccucr::Const0,
            true => Ccucr::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ccucr::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ccucr::Const1
    }
}
#[doc = "Field `CCUCR` writer - CCU Clock Control in Deep Sleep Mode"]
pub type CcucrW<'a, REG> = crate::BitWriter<'a, REG, Ccucr>;
impl<'a, REG> CcucrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucr::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucr::Const1)
    }
}
#[doc = "WDT Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcr {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Wdtcr> for bool {
    #[inline(always)]
    fn from(variant: Wdtcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCR` reader - WDT Clock Control in Deep Sleep Mode"]
pub type WdtcrR = crate::BitReader<Wdtcr>;
impl WdtcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtcr {
        match self.bits {
            false => Wdtcr::Const0,
            true => Wdtcr::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Wdtcr::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Wdtcr::Const1
    }
}
#[doc = "Field `WDTCR` writer - WDT Clock Control in Deep Sleep Mode"]
pub type WdtcrW<'a, REG> = crate::BitWriter<'a, REG, Wdtcr>;
impl<'a, REG> WdtcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcr::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcr::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SysselR {
        SysselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&self) -> FpdnR {
        FpdnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&self) -> PllpdnR {
        PllpdnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&self) -> VcopdnR {
        VcopdnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn usbcr(&self) -> UsbcrR {
        UsbcrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn mmccr(&self) -> MmccrR {
        MmccrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn eth0cr(&self) -> Eth0crR {
        Eth0crR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn ccucr(&self) -> CcucrR {
        CcucrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WdtcrR {
        WdtcrR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SysselW<DsleepcrSpec> {
        SysselW::new(self, 0)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn fpdn(&mut self) -> FpdnW<DsleepcrSpec> {
        FpdnW::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pllpdn(&mut self) -> PllpdnW<DsleepcrSpec> {
        PllpdnW::new(self, 12)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn vcopdn(&mut self) -> VcopdnW<DsleepcrSpec> {
        VcopdnW::new(self, 13)
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbcr(&mut self) -> UsbcrW<DsleepcrSpec> {
        UsbcrW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mmccr(&mut self) -> MmccrW<DsleepcrSpec> {
        MmccrW::new(self, 17)
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cr(&mut self) -> Eth0crW<DsleepcrSpec> {
        Eth0crW::new(self, 18)
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccucr(&mut self) -> CcucrW<DsleepcrSpec> {
        CcucrW::new(self, 20)
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcr(&mut self) -> WdtcrW<DsleepcrSpec> {
        WdtcrW::new(self, 21)
    }
}
#[doc = "Deep Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsleepcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsleepcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsleepcrSpec;
impl crate::RegisterSpec for DsleepcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsleepcr::R`](R) reader structure"]
impl crate::Readable for DsleepcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsleepcr::W`](W) writer structure"]
impl crate::Writable for DsleepcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSLEEPCR to value 0"]
impl crate::Resettable for DsleepcrSpec {
    const RESET_VALUE: u32 = 0;
}
