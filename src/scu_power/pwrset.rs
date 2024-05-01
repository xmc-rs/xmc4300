#[doc = "Register `PWRSET` writer"]
pub type W = crate::W<PwrsetSpec>;
#[doc = "Set Hibernate Domain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hib {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Enable Hibernate domain"]
    Const1 = 1,
}
impl From<Hib> for bool {
    #[inline(always)]
    fn from(variant: Hib) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` writer - Set Hibernate Domain Enable"]
pub type HibW<'a, REG> = crate::BitWriter<'a, REG, Hib>;
impl<'a, REG> HibW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Const0)
    }
    #[doc = "Enable Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Const1)
    }
}
#[doc = "Set USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbphypdq {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Active"]
    Const1 = 1,
}
impl From<Usbphypdq> for bool {
    #[inline(always)]
    fn from(variant: Usbphypdq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` writer - Set USB PHY Transceiver Disable"]
pub type UsbphypdqW<'a, REG> = crate::BitWriter<'a, REG, Usbphypdq>;
impl<'a, REG> UsbphypdqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbphypdq::Const0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbphypdq::Const1)
    }
}
#[doc = "Set USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbotgen {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Active"]
    Const1 = 1,
}
impl From<Usbotgen> for bool {
    #[inline(always)]
    fn from(variant: Usbotgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTGEN` writer - Set USB On-The-Go Comparators Enable"]
pub type UsbotgenW<'a, REG> = crate::BitWriter<'a, REG, Usbotgen>;
impl<'a, REG> UsbotgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbotgen::Const0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbotgen::Const1)
    }
}
#[doc = "Set USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbpuwq {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Pull-up not active"]
    Const1 = 1,
}
impl From<Usbpuwq> for bool {
    #[inline(always)]
    fn from(variant: Usbpuwq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` writer - Set USB Weak Pull-Up at PADN Enable"]
pub type UsbpuwqW<'a, REG> = crate::BitWriter<'a, REG, Usbpuwq>;
impl<'a, REG> UsbpuwqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbpuwq::Const0)
    }
    #[doc = "Pull-up not active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbpuwq::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - Set Hibernate Domain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HibW<PwrsetSpec> {
        HibW::new(self, 0)
    }
    #[doc = "Bit 16 - Set USB PHY Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbphypdq(&mut self) -> UsbphypdqW<PwrsetSpec> {
        UsbphypdqW::new(self, 16)
    }
    #[doc = "Bit 17 - Set USB On-The-Go Comparators Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbotgen(&mut self) -> UsbotgenW<PwrsetSpec> {
        UsbotgenW::new(self, 17)
    }
    #[doc = "Bit 18 - Set USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbpuwq(&mut self) -> UsbpuwqW<PwrsetSpec> {
        UsbpuwqW::new(self, 18)
    }
}
#[doc = "PCU Set Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrsetSpec;
impl crate::RegisterSpec for PwrsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwrset::W`](W) writer structure"]
impl crate::Writable for PwrsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRSET to value 0"]
impl crate::Resettable for PwrsetSpec {
    const RESET_VALUE: u32 = 0;
}
