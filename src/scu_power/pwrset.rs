#[doc = "Register `PWRSET` writer"]
pub struct W(crate::W<PWRSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PWRSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set Hibernate Domain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIB_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable Hibernate domain"]
    CONST_1 = 1,
}
impl From<HIB_AW> for bool {
    #[inline(always)]
    fn from(variant: HIB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` writer - Set Hibernate Domain Enable"]
pub type HIB_W<'a, const O: u8> = crate::BitWriter<'a, PWRSET_SPEC, O, HIB_AW>;
impl<'a, const O: u8> HIB_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIB_AW::CONST_0)
    }
    #[doc = "Enable Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIB_AW::CONST_1)
    }
}
#[doc = "Set USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPHYPDQ_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Active"]
    CONST_1 = 1,
}
impl From<USBPHYPDQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` writer - Set USB PHY Transceiver Disable"]
pub type USBPHYPDQ_W<'a, const O: u8> = crate::BitWriter<'a, PWRSET_SPEC, O, USBPHYPDQ_AW>;
impl<'a, const O: u8> USBPHYPDQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::CONST_0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::CONST_1)
    }
}
#[doc = "Set USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTGEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Active"]
    CONST_1 = 1,
}
impl From<USBOTGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTGEN` writer - Set USB On-The-Go Comparators Enable"]
pub type USBOTGEN_W<'a, const O: u8> = crate::BitWriter<'a, PWRSET_SPEC, O, USBOTGEN_AW>;
impl<'a, const O: u8> USBOTGEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::CONST_0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::CONST_1)
    }
}
#[doc = "Set USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPUWQ_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Pull-up not active"]
    CONST_1 = 1,
}
impl From<USBPUWQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` writer - Set USB Weak Pull-Up at PADN Enable"]
pub type USBPUWQ_W<'a, const O: u8> = crate::BitWriter<'a, PWRSET_SPEC, O, USBPUWQ_AW>;
impl<'a, const O: u8> USBPUWQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::CONST_0)
    }
    #[doc = "Pull-up not active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - Set Hibernate Domain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HIB_W<0> {
        HIB_W::new(self)
    }
    #[doc = "Bit 16 - Set USB PHY Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbphypdq(&mut self) -> USBPHYPDQ_W<16> {
        USBPHYPDQ_W::new(self)
    }
    #[doc = "Bit 17 - Set USB On-The-Go Comparators Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbotgen(&mut self) -> USBOTGEN_W<17> {
        USBOTGEN_W::new(self)
    }
    #[doc = "Bit 18 - Set USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbpuwq(&mut self) -> USBPUWQ_W<18> {
        USBPUWQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCU Set Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrset](index.html) module"]
pub struct PWRSET_SPEC;
impl crate::RegisterSpec for PWRSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwrset::W](W) writer structure"]
impl crate::Writable for PWRSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRSET to value 0"]
impl crate::Resettable for PWRSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
