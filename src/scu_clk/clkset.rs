#[doc = "Register `CLKSET` writer"]
pub type W = crate::W<ClksetSpec>;
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcen {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Enable"]
    Const1 = 1,
}
impl From<Usbcen> for bool {
    #[inline(always)]
    fn from(variant: Usbcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCEN` writer - USB Clock Enable"]
pub type UsbcenW<'a, REG> = crate::BitWriter<'a, REG, Usbcen>;
impl<'a, REG> UsbcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcen::Const0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcen::Const1)
    }
}
#[doc = "MMC Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmccen {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Enable"]
    Const1 = 1,
}
impl From<Mmccen> for bool {
    #[inline(always)]
    fn from(variant: Mmccen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCEN` writer - MMC Clock Enable"]
pub type MmccenW<'a, REG> = crate::BitWriter<'a, REG, Mmccen>;
impl<'a, REG> MmccenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccen::Const0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccen::Const1)
    }
}
#[doc = "Ethernet Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0cen {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Enable"]
    Const1 = 1,
}
impl From<Eth0cen> for bool {
    #[inline(always)]
    fn from(variant: Eth0cen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CEN` writer - Ethernet Clock Enable"]
pub type Eth0cenW<'a, REG> = crate::BitWriter<'a, REG, Eth0cen>;
impl<'a, REG> Eth0cenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cen::Const0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cen::Const1)
    }
}
#[doc = "CCU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucen {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Enable"]
    Const1 = 1,
}
impl From<Ccucen> for bool {
    #[inline(always)]
    fn from(variant: Ccucen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCEN` writer - CCU Clock Enable"]
pub type CcucenW<'a, REG> = crate::BitWriter<'a, REG, Ccucen>;
impl<'a, REG> CcucenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucen::Const0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucen::Const1)
    }
}
#[doc = "WDT Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcen {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Enable"]
    Const1 = 1,
}
impl From<Wdtcen> for bool {
    #[inline(always)]
    fn from(variant: Wdtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCEN` writer - WDT Clock Enable"]
pub type WdtcenW<'a, REG> = crate::BitWriter<'a, REG, Wdtcen>;
impl<'a, REG> WdtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcen::Const0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcen::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcen(&mut self) -> UsbcenW<ClksetSpec> {
        UsbcenW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mmccen(&mut self) -> MmccenW<ClksetSpec> {
        MmccenW::new(self, 1)
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cen(&mut self) -> Eth0cenW<ClksetSpec> {
        Eth0cenW::new(self, 2)
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucen(&mut self) -> CcucenW<ClksetSpec> {
        CcucenW::new(self, 4)
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcen(&mut self) -> WdtcenW<ClksetSpec> {
        WdtcenW::new(self, 5)
    }
}
#[doc = "CLK Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClksetSpec;
impl crate::RegisterSpec for ClksetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkset::W`](W) writer structure"]
impl crate::Writable for ClksetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSET to value 0"]
impl crate::Resettable for ClksetSpec {
    const RESET_VALUE: u32 = 0;
}
