#[doc = "Register `CGATSET1` writer"]
pub struct W(crate::W<CGATSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGATSET1_SPEC>;
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
impl From<crate::W<CGATSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGATSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LEDTS Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<LEDTSCU0_AW> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0` writer - LEDTS Gating Set"]
pub type LEDTSCU0_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET1_SPEC, O, LEDTSCU0_AW>;
impl<'a, const O: u8> LEDTSCU0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LEDTSCU0_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LEDTSCU0_AW::CONST_1)
    }
}
#[doc = "MultiCAN Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<MCAN0_AW> for bool {
    #[inline(always)]
    fn from(variant: MCAN0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0` writer - MultiCAN Gating Set"]
pub type MCAN0_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET1_SPEC, O, MCAN0_AW>;
impl<'a, const O: u8> MCAN0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MCAN0_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MCAN0_AW::CONST_1)
    }
}
#[doc = "DAC Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<DAC_AW> for bool {
    #[inline(always)]
    fn from(variant: DAC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC` writer - DAC Gating Set"]
pub type DAC_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET1_SPEC, O, DAC_AW>;
impl<'a, const O: u8> DAC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DAC_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DAC_AW::CONST_1)
    }
}
#[doc = "MMC Interface Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCI_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<MMCI_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCI` writer - MMC Interface Gating Set"]
pub type MMCI_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET1_SPEC, O, MMCI_AW>;
impl<'a, const O: u8> MMCI_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCI_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCI_AW::CONST_1)
    }
}
#[doc = "USIC1 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<USIC1_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1` writer - USIC1 Gating Set"]
pub type USIC1_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET1_SPEC, O, USIC1_AW>;
impl<'a, const O: u8> USIC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC1_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC1_AW::CONST_1)
    }
}
#[doc = "PORTS Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable gating"]
    CONST_1 = 1,
}
impl From<PPORTS_AW> for bool {
    #[inline(always)]
    fn from(variant: PPORTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTS` writer - PORTS Gating Set"]
pub type PPORTS_W<'a, const O: u8> = crate::BitWriter<'a, CGATSET1_SPEC, O, PPORTS_AW>;
impl<'a, const O: u8> PPORTS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPORTS_AW::CONST_0)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPORTS_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 3 - LEDTS Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0(&mut self) -> LEDTSCU0_W<3> {
        LEDTSCU0_W::new(self)
    }
    #[doc = "Bit 4 - MultiCAN Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0(&mut self) -> MCAN0_W<4> {
        MCAN0_W::new(self)
    }
    #[doc = "Bit 5 - DAC Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<5> {
        DAC_W::new(self)
    }
    #[doc = "Bit 6 - MMC Interface Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn mmci(&mut self) -> MMCI_W<6> {
        MMCI_W::new(self)
    }
    #[doc = "Bit 7 - USIC1 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn usic1(&mut self) -> USIC1_W<7> {
        USIC1_W::new(self)
    }
    #[doc = "Bit 9 - PORTS Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn pports(&mut self) -> PPORTS_W<9> {
        PPORTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral 1 Clock Gating Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatset1](index.html) module"]
pub struct CGATSET1_SPEC;
impl crate::RegisterSpec for CGATSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cgatset1::W](W) writer structure"]
impl crate::Writable for CGATSET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATSET1 to value 0"]
impl crate::Resettable for CGATSET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
