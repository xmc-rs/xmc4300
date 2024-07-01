#[doc = "Register `TRBSCR` writer"]
pub type W = crate::W<TRBSCR_SPEC>;
#[doc = "Clear Standard Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSRBI_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.SRBI."]
    VALUE2 = 1,
}
impl From<CSRBI_A> for bool {
    #[inline(always)]
    fn from(variant: CSRBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSRBI` writer - Clear Standard Receive Buffer Event"]
pub type CSRBI_W<'a, REG> = crate::BitWriter<'a, REG, CSRBI_A>;
impl<'a, REG> CSRBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CSRBI_A::VALUE1)
    }
    #[doc = "Clear TRBSR.SRBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSRBI_A::VALUE2)
    }
}
#[doc = "Clear Receive Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRBERI_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.RBERI."]
    VALUE2 = 1,
}
impl From<CRBERI_A> for bool {
    #[inline(always)]
    fn from(variant: CRBERI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRBERI` writer - Clear Receive Buffer Error Event"]
pub type CRBERI_W<'a, REG> = crate::BitWriter<'a, REG, CRBERI_A>;
impl<'a, REG> CRBERI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRBERI_A::VALUE1)
    }
    #[doc = "Clear TRBSR.RBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRBERI_A::VALUE2)
    }
}
#[doc = "Clear Alternative Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARBI_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.ARBI."]
    VALUE2 = 1,
}
impl From<CARBI_A> for bool {
    #[inline(always)]
    fn from(variant: CARBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARBI` writer - Clear Alternative Receive Buffer Event"]
pub type CARBI_W<'a, REG> = crate::BitWriter<'a, REG, CARBI_A>;
impl<'a, REG> CARBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARBI_A::VALUE1)
    }
    #[doc = "Clear TRBSR.ARBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARBI_A::VALUE2)
    }
}
#[doc = "Clear Standard Transmit Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTBI_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.STBI."]
    VALUE2 = 1,
}
impl From<CSTBI_A> for bool {
    #[inline(always)]
    fn from(variant: CSTBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTBI` writer - Clear Standard Transmit Buffer Event"]
pub type CSTBI_W<'a, REG> = crate::BitWriter<'a, REG, CSTBI_A>;
impl<'a, REG> CSTBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTBI_A::VALUE1)
    }
    #[doc = "Clear TRBSR.STBI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSTBI_A::VALUE2)
    }
}
#[doc = "Clear Transmit Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTBERI_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear TRBSR.TBERI."]
    VALUE2 = 1,
}
impl From<CTBERI_A> for bool {
    #[inline(always)]
    fn from(variant: CTBERI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTBERI` writer - Clear Transmit Buffer Error Event"]
pub type CTBERI_W<'a, REG> = crate::BitWriter<'a, REG, CTBERI_A>;
impl<'a, REG> CTBERI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTBERI_A::VALUE1)
    }
    #[doc = "Clear TRBSR.TBERI."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTBERI_A::VALUE2)
    }
}
#[doc = "Clear Bypass Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBDV_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Clear BYPCR.BDV."]
    VALUE2 = 1,
}
impl From<CBDV_A> for bool {
    #[inline(always)]
    fn from(variant: CBDV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBDV` writer - Clear Bypass Data Valid"]
pub type CBDV_W<'a, REG> = crate::BitWriter<'a, REG, CBDV_A>;
impl<'a, REG> CBDV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CBDV_A::VALUE1)
    }
    #[doc = "Clear BYPCR.BDV."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CBDV_A::VALUE2)
    }
}
#[doc = "Flush Receive Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSHRB_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2 = 1,
}
impl From<FLUSHRB_A> for bool {
    #[inline(always)]
    fn from(variant: FLUSHRB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHRB` writer - Flush Receive Buffer"]
pub type FLUSHRB_W<'a, REG> = crate::BitWriter<'a, REG, FLUSHRB_A>;
impl<'a, REG> FLUSHRB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSHRB_A::VALUE1)
    }
    #[doc = "The receive FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSHRB_A::VALUE2)
    }
}
#[doc = "Flush Transmit Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSHTB_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    VALUE2 = 1,
}
impl From<FLUSHTB_A> for bool {
    #[inline(always)]
    fn from(variant: FLUSHTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHTB` writer - Flush Transmit Buffer"]
pub type FLUSHTB_W<'a, REG> = crate::BitWriter<'a, REG, FLUSHTB_A>;
impl<'a, REG> FLUSHTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSHTB_A::VALUE1)
    }
    #[doc = "The transmit FIFO buffer is cleared (filling level is cleared and output pointer is set to input pointer value). Should only be used while the FIFO buffer is not taking part in data traffic."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSHTB_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Standard Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn csrbi(&mut self) -> CSRBI_W<TRBSCR_SPEC> {
        CSRBI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Receive Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn crberi(&mut self) -> CRBERI_W<TRBSCR_SPEC> {
        CRBERI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Alternative Receive Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn carbi(&mut self) -> CARBI_W<TRBSCR_SPEC> {
        CARBI_W::new(self, 2)
    }
    #[doc = "Bit 8 - Clear Standard Transmit Buffer Event"]
    #[inline(always)]
    #[must_use]
    pub fn cstbi(&mut self) -> CSTBI_W<TRBSCR_SPEC> {
        CSTBI_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Transmit Buffer Error Event"]
    #[inline(always)]
    #[must_use]
    pub fn ctberi(&mut self) -> CTBERI_W<TRBSCR_SPEC> {
        CTBERI_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Bypass Data Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cbdv(&mut self) -> CBDV_W<TRBSCR_SPEC> {
        CBDV_W::new(self, 10)
    }
    #[doc = "Bit 14 - Flush Receive Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn flushrb(&mut self) -> FLUSHRB_W<TRBSCR_SPEC> {
        FLUSHRB_W::new(self, 14)
    }
    #[doc = "Bit 15 - Flush Transmit Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn flushtb(&mut self) -> FLUSHTB_W<TRBSCR_SPEC> {
        FLUSHTB_W::new(self, 15)
    }
}
#[doc = "Transmit/Receive Buffer Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trbscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRBSCR_SPEC;
impl crate::RegisterSpec for TRBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trbscr::W`](W) writer structure"]
impl crate::Writable for TRBSCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRBSCR to value 0"]
impl crate::Resettable for TRBSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
