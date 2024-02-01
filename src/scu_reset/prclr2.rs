#[doc = "Register `PRCLR2` writer"]
pub type W = crate::W<PRCLR2_SPEC>;
#[doc = "WDT Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<WDTRS_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Clear"]
pub type WDTRS_W<'a, REG> = crate::BitWriter<'a, REG, WDTRS_AW>;
impl<'a, REG> WDTRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_AW::CONST_1)
    }
}
#[doc = "ETH0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<ETH0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` writer - ETH0 Reset Clear"]
pub type ETH0RS_W<'a, REG> = crate::BitWriter<'a, REG, ETH0RS_AW>;
impl<'a, REG> ETH0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_AW::CONST_1)
    }
}
#[doc = "DMA0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<DMA0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Clear"]
pub type DMA0RS_W<'a, REG> = crate::BitWriter<'a, REG, DMA0RS_AW>;
impl<'a, REG> DMA0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_AW::CONST_1)
    }
}
#[doc = "FCE Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCERS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<FCERS_AW> for bool {
    #[inline(always)]
    fn from(variant: FCERS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Clear"]
pub type FCERS_W<'a, REG> = crate::BitWriter<'a, REG, FCERS_AW>;
impl<'a, REG> FCERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_AW::CONST_1)
    }
}
#[doc = "USB Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<USBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Clear"]
pub type USBRS_W<'a, REG> = crate::BitWriter<'a, REG, USBRS_AW>;
impl<'a, REG> USBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_AW::CONST_1)
    }
}
#[doc = "ECAT0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<ECAT0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` writer - ECAT0 Reset Clear"]
pub type ECAT0RS_W<'a, REG> = crate::BitWriter<'a, REG, ECAT0RS_AW>;
impl<'a, REG> ECAT0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RS_AW::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RS_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrs(&mut self) -> WDTRS_W<PRCLR2_SPEC> {
        WDTRS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ETH0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eth0rs(&mut self) -> ETH0RS_W<PRCLR2_SPEC> {
        ETH0RS_W::new(self, 2)
    }
    #[doc = "Bit 4 - DMA0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rs(&mut self) -> DMA0RS_W<PRCLR2_SPEC> {
        DMA0RS_W::new(self, 4)
    }
    #[doc = "Bit 6 - FCE Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fcers(&mut self) -> FCERS_W<PRCLR2_SPEC> {
        FCERS_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbrs(&mut self) -> USBRS_W<PRCLR2_SPEC> {
        USBRS_W::new(self, 7)
    }
    #[doc = "Bit 10 - ECAT0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rs(&mut self) -> ECAT0RS_W<PRCLR2_SPEC> {
        ECAT0RS_W::new(self, 10)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCU Peripheral 2 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRCLR2_SPEC;
impl crate::RegisterSpec for PRCLR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr2::W`](W) writer structure"]
impl crate::Writable for PRCLR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR2 to value 0"]
impl crate::Resettable for PRCLR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
