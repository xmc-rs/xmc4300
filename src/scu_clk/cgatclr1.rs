#[doc = "Register `CGATCLR1` writer"]
pub type W = crate::W<Cgatclr1Spec>;
#[doc = "LEDTS Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledtscu0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Ledtscu0> for bool {
    #[inline(always)]
    fn from(variant: Ledtscu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0` writer - LEDTS Gating Clear"]
pub type Ledtscu0W<'a, REG> = crate::BitWriter<'a, REG, Ledtscu0>;
impl<'a, REG> Ledtscu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ledtscu0::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ledtscu0::Const1)
    }
}
#[doc = "MultiCAN Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcan0 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Mcan0> for bool {
    #[inline(always)]
    fn from(variant: Mcan0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0` writer - MultiCAN Gating Clear"]
pub type Mcan0W<'a, REG> = crate::BitWriter<'a, REG, Mcan0>;
impl<'a, REG> Mcan0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcan0::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcan0::Const1)
    }
}
#[doc = "DAC Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Dac> for bool {
    #[inline(always)]
    fn from(variant: Dac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC` writer - DAC Gating Clear"]
pub type DacW<'a, REG> = crate::BitWriter<'a, REG, Dac>;
impl<'a, REG> DacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac::Const1)
    }
}
#[doc = "MMC Interface Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmci {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Mmci> for bool {
    #[inline(always)]
    fn from(variant: Mmci) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCI` writer - MMC Interface Gating Clear"]
pub type MmciW<'a, REG> = crate::BitWriter<'a, REG, Mmci>;
impl<'a, REG> MmciW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mmci::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmci::Const1)
    }
}
#[doc = "USIC1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic1 {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Usic1> for bool {
    #[inline(always)]
    fn from(variant: Usic1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1` writer - USIC1 Gating Clear"]
pub type Usic1W<'a, REG> = crate::BitWriter<'a, REG, Usic1>;
impl<'a, REG> Usic1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1::Const1)
    }
}
#[doc = "PORTS Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pports {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Disable gating"]
    Const1 = 1,
}
impl From<Pports> for bool {
    #[inline(always)]
    fn from(variant: Pports) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTS` writer - PORTS Gating Clear"]
pub type PportsW<'a, REG> = crate::BitWriter<'a, REG, Pports>;
impl<'a, REG> PportsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pports::Const0)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pports::Const1)
    }
}
impl W {
    #[doc = "Bit 3 - LEDTS Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0(&mut self) -> Ledtscu0W<Cgatclr1Spec> {
        Ledtscu0W::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0(&mut self) -> Mcan0W<Cgatclr1Spec> {
        Mcan0W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DacW<Cgatclr1Spec> {
        DacW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Interface Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mmci(&mut self) -> MmciW<Cgatclr1Spec> {
        MmciW::new(self, 6)
    }
    #[doc = "Bit 7 - USIC1 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic1(&mut self) -> Usic1W<Cgatclr1Spec> {
        Usic1W::new(self, 7)
    }
    #[doc = "Bit 9 - PORTS Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pports(&mut self) -> PportsW<Cgatclr1Spec> {
        PportsW::new(self, 9)
    }
}
#[doc = "Peripheral 1 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatclr1Spec;
impl crate::RegisterSpec for Cgatclr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr1::W`](W) writer structure"]
impl crate::Writable for Cgatclr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATCLR1 to value 0"]
impl crate::Resettable for Cgatclr1Spec {
    const RESET_VALUE: u32 = 0;
}
