#[doc = "Register `TRBSCR` writer"]
pub type W = crate::W<TrbscrSpec>;
#[doc = "Clear Standard Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csrbi {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: Clear TRBSR.SRBI."]
    Value2 = 1,
}
impl From<Csrbi> for bool {
    #[inline(always)]
    fn from(variant: Csrbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSRBI` writer - Clear Standard Receive Buffer Event"]
pub type CsrbiW<'a, REG> = crate::BitWriter<'a, REG, Csrbi>;
impl<'a, REG> CsrbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Csrbi::Value1)
    }
    #[doc = "Clear TRBSR.SRBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Csrbi::Value2)
    }
}
#[doc = "Clear Receive Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crberi {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: Clear TRBSR.RBERI."]
    Value2 = 1,
}
impl From<Crberi> for bool {
    #[inline(always)]
    fn from(variant: Crberi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRBERI` writer - Clear Receive Buffer Error Event"]
pub type CrberiW<'a, REG> = crate::BitWriter<'a, REG, Crberi>;
impl<'a, REG> CrberiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Crberi::Value1)
    }
    #[doc = "Clear TRBSR.RBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Crberi::Value2)
    }
}
#[doc = "Clear Alternative Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carbi {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: Clear TRBSR.ARBI."]
    Value2 = 1,
}
impl From<Carbi> for bool {
    #[inline(always)]
    fn from(variant: Carbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARBI` writer - Clear Alternative Receive Buffer Event"]
pub type CarbiW<'a, REG> = crate::BitWriter<'a, REG, Carbi>;
impl<'a, REG> CarbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Carbi::Value1)
    }
    #[doc = "Clear TRBSR.ARBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Carbi::Value2)
    }
}
#[doc = "Clear Standard Transmit Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstbi {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: Clear TRBSR.STBI."]
    Value2 = 1,
}
impl From<Cstbi> for bool {
    #[inline(always)]
    fn from(variant: Cstbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTBI` writer - Clear Standard Transmit Buffer Event"]
pub type CstbiW<'a, REG> = crate::BitWriter<'a, REG, Cstbi>;
impl<'a, REG> CstbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstbi::Value1)
    }
    #[doc = "Clear TRBSR.STBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cstbi::Value2)
    }
}
#[doc = "Clear Transmit Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctberi {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: Clear TRBSR.TBERI."]
    Value2 = 1,
}
impl From<Ctberi> for bool {
    #[inline(always)]
    fn from(variant: Ctberi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTBERI` writer - Clear Transmit Buffer Error Event"]
pub type CtberiW<'a, REG> = crate::BitWriter<'a, REG, Ctberi>;
impl<'a, REG> CtberiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctberi::Value1)
    }
    #[doc = "Clear TRBSR.TBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctberi::Value2)
    }
}
#[doc = "Clear Bypass Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbdv {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: Clear BYPCR.BDV."]
    Value2 = 1,
}
impl From<Cbdv> for bool {
    #[inline(always)]
    fn from(variant: Cbdv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBDV` writer - Clear Bypass Data Valid"]
pub type CbdvW<'a, REG> = crate::BitWriter<'a, REG, Cbdv>;
impl<'a, REG> CbdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cbdv::Value1)
    }
    #[doc = "Clear BYPCR.BDV."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cbdv::Value2)
    }
}
#[doc = "Flush Receive Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flushrb {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    Value2 = 1,
}
impl From<Flushrb> for bool {
    #[inline(always)]
    fn from(variant: Flushrb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHRB` writer - Flush Receive Buffer"]
pub type FlushrbW<'a, REG> = crate::BitWriter<'a, REG, Flushrb>;
impl<'a, REG> FlushrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Flushrb::Value1)
    }
    #[doc = "The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flushrb::Value2)
    }
}
#[doc = "Flush Transmit Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flushtb {
    #[doc = "0: No effect."]
    Value1 = 0,
    #[doc = "1: The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    Value2 = 1,
}
impl From<Flushtb> for bool {
    #[inline(always)]
    fn from(variant: Flushtb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHTB` writer - Flush Transmit Buffer"]
pub type FlushtbW<'a, REG> = crate::BitWriter<'a, REG, Flushtb>;
impl<'a, REG> FlushtbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Flushtb::Value1)
    }
    #[doc = "The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flushtb::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Standard Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn csrbi(&mut self) -> CsrbiW<TrbscrSpec> {
        CsrbiW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Receive Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn crberi(&mut self) -> CrberiW<TrbscrSpec> {
        CrberiW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Alternative Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn carbi(&mut self) -> CarbiW<TrbscrSpec> {
        CarbiW::new(self, 2)
    }
    #[doc = "Bit 8 - Clear Standard Transmit Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn cstbi(&mut self) -> CstbiW<TrbscrSpec> {
        CstbiW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Transmit Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn ctberi(&mut self) -> CtberiW<TrbscrSpec> {
        CtberiW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Bypass Data Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cbdv(&mut self) -> CbdvW<TrbscrSpec> {
        CbdvW::new(self, 10)
    }
    #[doc = "Bit 14 - Flush Receive Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn flushrb(&mut self) -> FlushrbW<TrbscrSpec> {
        FlushrbW::new(self, 14)
    }
    #[doc = "Bit 15 - Flush Transmit Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn flushtb(&mut self) -> FlushtbW<TrbscrSpec> {
        FlushtbW::new(self, 15)
    }
}
#[doc = "Transmit/Receive Buffer Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trbscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrbscrSpec;
impl crate::RegisterSpec for TrbscrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trbscr::W`](W) writer structure"]
impl crate::Writable for TrbscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRBSCR to value 0"]
impl crate::Resettable for TrbscrSpec {
    const RESET_VALUE: u32 = 0;
}
