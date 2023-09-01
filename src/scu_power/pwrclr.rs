#[doc = "Register `PWRCLR` writer"]
pub type W = crate::W<PWRCLR_SPEC>;
#[doc = "Clear Disable Hibernate Domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIB_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Disable Hibernate domain"]
    CONST_1 = 1,
}
impl From<HIB_AW> for bool {
    #[inline(always)]
    fn from(variant: HIB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` writer - Clear Disable Hibernate Domain"]
pub type HIB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HIB_AW>;
impl<'a, REG, const O: u8> HIB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_AW::CONST_0)
    }
    #[doc = "Disable Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_AW::CONST_1)
    }
}
#[doc = "Clear USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPHYPDQ_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Power-down"]
    CONST_1 = 1,
}
impl From<USBPHYPDQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` writer - Clear USB PHY Transceiver Disable"]
pub type USBPHYPDQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBPHYPDQ_AW>;
impl<'a, REG, const O: u8> USBPHYPDQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYPDQ_AW::CONST_0)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYPDQ_AW::CONST_1)
    }
}
#[doc = "Clear USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTGEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Power-down"]
    CONST_1 = 1,
}
impl From<USBOTGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTGEN` writer - Clear USB On-The-Go Comparators Enable"]
pub type USBOTGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBOTGEN_AW>;
impl<'a, REG, const O: u8> USBOTGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTGEN_AW::CONST_0)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTGEN_AW::CONST_1)
    }
}
#[doc = "Clear USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPUWQ_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Pull-up active"]
    CONST_1 = 1,
}
impl From<USBPUWQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` writer - Clear USB Weak Pull-Up at PADN Enable"]
pub type USBPUWQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBPUWQ_AW>;
impl<'a, REG, const O: u8> USBPUWQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBPUWQ_AW::CONST_0)
    }
    #[doc = "Pull-up active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBPUWQ_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Disable Hibernate Domain"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HIB_W<PWRCLR_SPEC, 0> {
        HIB_W::new(self)
    }
    #[doc = "Bit 16 - Clear USB PHY Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbphypdq(&mut self) -> USBPHYPDQ_W<PWRCLR_SPEC, 16> {
        USBPHYPDQ_W::new(self)
    }
    #[doc = "Bit 17 - Clear USB On-The-Go Comparators Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbotgen(&mut self) -> USBOTGEN_W<PWRCLR_SPEC, 17> {
        USBOTGEN_W::new(self)
    }
    #[doc = "Bit 18 - Clear USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbpuwq(&mut self) -> USBPUWQ_W<PWRCLR_SPEC, 18> {
        USBPUWQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PCU Clear Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCLR_SPEC;
impl crate::RegisterSpec for PWRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwrclr::W`](W) writer structure"]
impl crate::Writable for PWRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCLR to value 0"]
impl crate::Resettable for PWRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
