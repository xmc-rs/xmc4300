#[doc = "Register `PRSET2` writer"]
pub type W = crate::W<Prset2Spec>;
#[doc = "WDT Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtrs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Wdtrs> for bool {
    #[inline(always)]
    fn from(variant: Wdtrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Assert"]
pub type WdtrsW<'a, REG> = crate::BitWriter<'a, REG, Wdtrs>;
impl<'a, REG> WdtrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrs::Const1)
    }
}
#[doc = "ETH0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Eth0rs> for bool {
    #[inline(always)]
    fn from(variant: Eth0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` writer - ETH0 Reset Assert"]
pub type Eth0rsW<'a, REG> = crate::BitWriter<'a, REG, Eth0rs>;
impl<'a, REG> Eth0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0rs::Const1)
    }
}
#[doc = "DMA0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Dma0rs> for bool {
    #[inline(always)]
    fn from(variant: Dma0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Assert"]
pub type Dma0rsW<'a, REG> = crate::BitWriter<'a, REG, Dma0rs>;
impl<'a, REG> Dma0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rs::Const1)
    }
}
#[doc = "FCE Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcers {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Fcers> for bool {
    #[inline(always)]
    fn from(variant: Fcers) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Assert"]
pub type FcersW<'a, REG> = crate::BitWriter<'a, REG, Fcers>;
impl<'a, REG> FcersW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcers::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcers::Const1)
    }
}
#[doc = "USB Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Usbrs> for bool {
    #[inline(always)]
    fn from(variant: Usbrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Assert"]
pub type UsbrsW<'a, REG> = crate::BitWriter<'a, REG, Usbrs>;
impl<'a, REG> UsbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrs::Const1)
    }
}
#[doc = "ECAT0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Ecat0rs> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` writer - ECAT0 Reset Assert"]
pub type Ecat0rsW<'a, REG> = crate::BitWriter<'a, REG, Ecat0rs>;
impl<'a, REG> Ecat0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecat0rs::Const1)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrs(&mut self) -> WdtrsW<Prset2Spec> {
        WdtrsW::new(self, 1)
    }
    #[doc = "Bit 2 - ETH0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eth0rs(&mut self) -> Eth0rsW<Prset2Spec> {
        Eth0rsW::new(self, 2)
    }
    #[doc = "Bit 4 - DMA0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rs(&mut self) -> Dma0rsW<Prset2Spec> {
        Dma0rsW::new(self, 4)
    }
    #[doc = "Bit 6 - FCE Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn fcers(&mut self) -> FcersW<Prset2Spec> {
        FcersW::new(self, 6)
    }
    #[doc = "Bit 7 - USB Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usbrs(&mut self) -> UsbrsW<Prset2Spec> {
        UsbrsW::new(self, 7)
    }
    #[doc = "Bit 10 - ECAT0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rs(&mut self) -> Ecat0rsW<Prset2Spec> {
        Ecat0rsW::new(self, 10)
    }
}
#[doc = "RCU Peripheral 2 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prset2Spec;
impl crate::RegisterSpec for Prset2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset2::W`](W) writer structure"]
impl crate::Writable for Prset2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSET2 to value 0"]
impl crate::Resettable for Prset2Spec {
    const RESET_VALUE: u32 = 0;
}
