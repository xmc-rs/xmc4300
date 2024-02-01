#[doc = "Register `CLKSET` writer"]
pub type W = crate::W<CLKSET_SPEC>;
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<USBCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCEN` writer - USB Clock Enable"]
pub type USBCEN_W<'a, REG> = crate::BitWriter<'a, REG, USBCEN_AW>;
impl<'a, REG> USBCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCEN_AW::CONST_1)
    }
}
#[doc = "MMC Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<MMCCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCEN` writer - MMC Clock Enable"]
pub type MMCCEN_W<'a, REG> = crate::BitWriter<'a, REG, MMCCEN_AW>;
impl<'a, REG> MMCCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCEN_AW::CONST_1)
    }
}
#[doc = "Ethernet Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<ETH0CEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0CEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CEN` writer - Ethernet Clock Enable"]
pub type ETH0CEN_W<'a, REG> = crate::BitWriter<'a, REG, ETH0CEN_AW>;
impl<'a, REG> ETH0CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CEN_AW::CONST_1)
    }
}
#[doc = "CCU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<CCUCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCEN` writer - CCU Clock Enable"]
pub type CCUCEN_W<'a, REG> = crate::BitWriter<'a, REG, CCUCEN_AW>;
impl<'a, REG> CCUCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCEN_AW::CONST_1)
    }
}
#[doc = "WDT Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable"]
    CONST_1 = 1,
}
impl From<WDTCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCEN` writer - WDT Clock Enable"]
pub type WDTCEN_W<'a, REG> = crate::BitWriter<'a, REG, WDTCEN_AW>;
impl<'a, REG> WDTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCEN_AW::CONST_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCEN_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcen(&mut self) -> USBCEN_W<CLKSET_SPEC> {
        USBCEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mmccen(&mut self) -> MMCCEN_W<CLKSET_SPEC> {
        MMCCEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cen(&mut self) -> ETH0CEN_W<CLKSET_SPEC> {
        ETH0CEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucen(&mut self) -> CCUCEN_W<CLKSET_SPEC> {
        CCUCEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcen(&mut self) -> WDTCEN_W<CLKSET_SPEC> {
        WDTCEN_W::new(self, 5)
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
#[doc = "CLK Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSET_SPEC;
impl crate::RegisterSpec for CLKSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkset::W`](W) writer structure"]
impl crate::Writable for CLKSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSET to value 0"]
impl crate::Resettable for CLKSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
