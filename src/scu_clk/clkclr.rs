#[doc = "Register `CLKCLR` writer"]
pub type W = crate::W<ClkclrSpec>;
#[doc = "USB Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcdi {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable clock"]
    Const1 = 1,
}
impl From<Usbcdi> for bool {
    #[inline(always)]
    fn from(variant: Usbcdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCDI` writer - USB Clock Disable"]
pub type UsbcdiW<'a, REG> = crate::BitWriter<'a, REG, Usbcdi>;
impl<'a, REG> UsbcdiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcdi::Const0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcdi::Const1)
    }
}
#[doc = "MMC Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmccdi {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable clock"]
    Const1 = 1,
}
impl From<Mmccdi> for bool {
    #[inline(always)]
    fn from(variant: Mmccdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCDI` writer - MMC Clock Disable"]
pub type MmccdiW<'a, REG> = crate::BitWriter<'a, REG, Mmccdi>;
impl<'a, REG> MmccdiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccdi::Const0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccdi::Const1)
    }
}
#[doc = "Ethernet Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0cdi {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable clock"]
    Const1 = 1,
}
impl From<Eth0cdi> for bool {
    #[inline(always)]
    fn from(variant: Eth0cdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CDI` writer - Ethernet Clock Disable"]
pub type Eth0cdiW<'a, REG> = crate::BitWriter<'a, REG, Eth0cdi>;
impl<'a, REG> Eth0cdiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cdi::Const0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cdi::Const1)
    }
}
#[doc = "CCU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucdi {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable clock"]
    Const1 = 1,
}
impl From<Ccucdi> for bool {
    #[inline(always)]
    fn from(variant: Ccucdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCDI` writer - CCU Clock Disable"]
pub type CcucdiW<'a, REG> = crate::BitWriter<'a, REG, Ccucdi>;
impl<'a, REG> CcucdiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucdi::Const0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucdi::Const1)
    }
}
#[doc = "WDT Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcdi {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable clock"]
    Const1 = 1,
}
impl From<Wdtcdi> for bool {
    #[inline(always)]
    fn from(variant: Wdtcdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCDI` writer - WDT Clock Disable"]
pub type WdtcdiW<'a, REG> = crate::BitWriter<'a, REG, Wdtcdi>;
impl<'a, REG> WdtcdiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcdi::Const0)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcdi::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcdi(&mut self) -> UsbcdiW<ClkclrSpec> {
        UsbcdiW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mmccdi(&mut self) -> MmccdiW<ClkclrSpec> {
        MmccdiW::new(self, 1)
    }
    #[doc = "Bit 2 - Ethernet Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cdi(&mut self) -> Eth0cdiW<ClkclrSpec> {
        Eth0cdiW::new(self, 2)
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucdi(&mut self) -> CcucdiW<ClkclrSpec> {
        CcucdiW::new(self, 4)
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcdi(&mut self) -> WdtcdiW<ClkclrSpec> {
        WdtcdiW::new(self, 5)
    }
}
#[doc = "CLK Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkclrSpec;
impl crate::RegisterSpec for ClkclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkclr::W`](W) writer structure"]
impl crate::Writable for ClkclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCLR to value 0"]
impl crate::Resettable for ClkclrSpec {
    const RESET_VALUE: u32 = 0;
}
