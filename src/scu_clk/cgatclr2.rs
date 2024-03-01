#[doc = "Register `CGATCLR2` writer"]
pub type W = crate::W<Cgatclr2Spec>;
#[doc = "WDT Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Wdt> for bool {
    #[inline(always)]
    fn from(variant: Wdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` writer - WDT Gating Clear"]
pub type WdtW<'a, REG> = crate::BitWriter<'a, REG, Wdt>;
impl<'a, REG> WdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Const1)
    }
}
#[doc = "ETH0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Eth0> for bool {
    #[inline(always)]
    fn from(variant: Eth0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0` writer - ETH0 Gating Clear"]
pub type Eth0W<'a, REG> = crate::BitWriter<'a, REG, Eth0>;
impl<'a, REG> Eth0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0::Const1)
    }
}
#[doc = "DMA0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Dma0> for bool {
    #[inline(always)]
    fn from(variant: Dma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` writer - DMA0 Gating Clear"]
pub type Dma0W<'a, REG> = crate::BitWriter<'a, REG, Dma0>;
impl<'a, REG> Dma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Const1)
    }
}
#[doc = "FCE Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fce {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Fce> for bool {
    #[inline(always)]
    fn from(variant: Fce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCE` writer - FCE Gating Clear"]
pub type FceW<'a, REG> = crate::BitWriter<'a, REG, Fce>;
impl<'a, REG> FceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fce::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fce::Const1)
    }
}
#[doc = "USB Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Usb> for bool {
    #[inline(always)]
    fn from(variant: Usb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` writer - USB Gating Clear"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG, Usb>;
impl<'a, REG> UsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb::Const1)
    }
}
#[doc = "ECAT0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Ecat0> for bool {
    #[inline(always)]
    fn from(variant: Ecat0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0` writer - ECAT0 Gating Clear"]
pub type Ecat0W<'a, REG> = crate::BitWriter<'a, REG, Ecat0>;
impl<'a, REG> Ecat0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0::Const1)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WdtW<Cgatclr2Spec> {
        WdtW::new(self, 1)
    }
    #[doc = "Bit 2 - ETH0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eth0(&mut self) -> Eth0W<Cgatclr2Spec> {
        Eth0W::new(self, 2)
    }
    #[doc = "Bit 4 - DMA0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> Dma0W<Cgatclr2Spec> {
        Dma0W::new(self, 4)
    }
    #[doc = "Bit 6 - FCE Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fce(&mut self) -> FceW<Cgatclr2Spec> {
        FceW::new(self, 6)
    }
    #[doc = "Bit 7 - USB Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> UsbW<Cgatclr2Spec> {
        UsbW::new(self, 7)
    }
    #[doc = "Bit 10 - ECAT0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0(&mut self) -> Ecat0W<Cgatclr2Spec> {
        Ecat0W::new(self, 10)
    }
}
#[doc = "Peripheral 2 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatclr2Spec;
impl crate::RegisterSpec for Cgatclr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr2::W`](W) writer structure"]
impl crate::Writable for Cgatclr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATCLR2 to value 0"]
impl crate::Resettable for Cgatclr2Spec {
    const RESET_VALUE: u32 = 0;
}
