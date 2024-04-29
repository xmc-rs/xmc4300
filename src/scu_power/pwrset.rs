#[doc = "Register `PWRSET` writer"]
pub type W = crate::W<PWRSET_SPEC>;
#[doc = "Set Hibernate Domain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIB_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable Hibernate domain"]
    CONST_1 = 1,
}
impl From<HIB_A> for bool {
    #[inline(always)]
    fn from(variant: HIB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` writer - Set Hibernate Domain Enable"]
pub type HIB_W<'a, REG> = crate::BitWriter<'a, REG, HIB_A>;
impl<'a, REG> HIB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::CONST_0)
    }
    #[doc = "Enable Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::CONST_1)
    }
}
#[doc = "Set USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPHYPDQ_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Active"]
    CONST_1 = 1,
}
impl From<USBPHYPDQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` writer - Set USB PHY Transceiver Disable"]
pub type USBPHYPDQ_W<'a, REG> = crate::BitWriter<'a, REG, USBPHYPDQ_A>;
impl<'a, REG> USBPHYPDQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYPDQ_A::CONST_0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYPDQ_A::CONST_1)
    }
}
#[doc = "Set USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTGEN_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Active"]
    CONST_1 = 1,
}
impl From<USBOTGEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTGEN` writer - Set USB On-The-Go Comparators Enable"]
pub type USBOTGEN_W<'a, REG> = crate::BitWriter<'a, REG, USBOTGEN_A>;
impl<'a, REG> USBOTGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTGEN_A::CONST_0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTGEN_A::CONST_1)
    }
}
#[doc = "Set USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPUWQ_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Pull-up not active"]
    CONST_1 = 1,
}
impl From<USBPUWQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` writer - Set USB Weak Pull-Up at PADN Enable"]
pub type USBPUWQ_W<'a, REG> = crate::BitWriter<'a, REG, USBPUWQ_A>;
impl<'a, REG> USBPUWQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBPUWQ_A::CONST_0)
    }
    #[doc = "Pull-up not active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBPUWQ_A::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - Set Hibernate Domain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HIB_W<PWRSET_SPEC> {
        HIB_W::new(self, 0)
    }
    #[doc = "Bit 16 - Set USB PHY Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbphypdq(&mut self) -> USBPHYPDQ_W<PWRSET_SPEC> {
        USBPHYPDQ_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set USB On-The-Go Comparators Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbotgen(&mut self) -> USBOTGEN_W<PWRSET_SPEC> {
        USBOTGEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbpuwq(&mut self) -> USBPUWQ_W<PWRSET_SPEC> {
        USBPUWQ_W::new(self, 18)
    }
}
#[doc = "PCU Set Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRSET_SPEC;
impl crate::RegisterSpec for PWRSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwrset::W`](W) writer structure"]
impl crate::Writable for PWRSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRSET to value 0"]
impl crate::Resettable for PWRSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
