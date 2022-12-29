#[doc = "Register `TRAPCLR` writer"]
pub struct W(crate::W<TRAPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRAPCLR_SPEC>;
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
impl From<crate::W<TRAPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRAPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OSC_HP Oscillator Watchdog Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<SOSCWDGT_AW> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` writer - OSC_HP Oscillator Watchdog Trap Clear"]
pub type SOSCWDGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, SOSCWDGT_AW, O>;
impl<'a, const O: u8> SOSCWDGT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SOSCWDGT_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SOSCWDGT_AW::CONST_1)
    }
}
#[doc = "System VCO Lock Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<SVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Clear"]
pub type SVCOLCKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, SVCOLCKT_AW, O>;
impl<'a, const O: u8> SVCOLCKT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SVCOLCKT_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SVCOLCKT_AW::CONST_1)
    }
}
#[doc = "USB VCO Lock Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<UVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Clear"]
pub type UVCOLCKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, UVCOLCKT_AW, O>;
impl<'a, const O: u8> UVCOLCKT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(UVCOLCKT_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(UVCOLCKT_AW::CONST_1)
    }
}
#[doc = "Parity Error Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<PET_AW> for bool {
    #[inline(always)]
    fn from(variant: PET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Clear"]
pub type PET_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, PET_AW, O>;
impl<'a, const O: u8> PET_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PET_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PET_AW::CONST_1)
    }
}
#[doc = "Brown Out Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<BRWNT_AW> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Clear"]
pub type BRWNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, BRWNT_AW, O>;
impl<'a, const O: u8> BRWNT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BRWNT_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BRWNT_AW::CONST_1)
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDGT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<ULPWDGT_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` writer - OSC_ULP Oscillator Watchdog Trap Clear"]
pub type ULPWDGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, ULPWDGT_AW, O>;
impl<'a, const O: u8> ULPWDGT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDGT_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDGT_AW::CONST_1)
    }
}
#[doc = "Peripheral Bridge 0 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<BWERR0T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Clear"]
pub type BWERR0T_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, BWERR0T_AW, O>;
impl<'a, const O: u8> BWERR0T_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR0T_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR0T_AW::CONST_1)
    }
}
#[doc = "Peripheral Bridge 1 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Clear trap request"]
    CONST_1 = 1,
}
impl From<BWERR1T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Clear"]
pub type BWERR1T_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, BWERR1T_AW, O>;
impl<'a, const O: u8> BWERR1T_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR1T_AW::CONST_0)
    }
    #[doc = "Clear trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR1T_AW::CONST_1)
    }
}
#[doc = "EtherCat Reset 0 Trap Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RST_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<ECAT0RST_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RST` writer - EtherCat Reset 0 Trap Clear"]
pub type ECAT0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRAPCLR_SPEC, ECAT0RST_AW, O>;
impl<'a, const O: u8> ECAT0RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0RST_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0RST_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W<0> {
        SOSCWDGT_W::new(self)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W<2> {
        SVCOLCKT_W::new(self)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W<3> {
        UVCOLCKT_W::new(self)
    }
    #[doc = "Bit 4 - Parity Error Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pet(&mut self) -> PET_W<4> {
        PET_W::new(self)
    }
    #[doc = "Bit 5 - Brown Out Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn brwnt(&mut self) -> BRWNT_W<5> {
        BRWNT_W::new(self)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdgt(&mut self) -> ULPWDGT_W<6> {
        ULPWDGT_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr0t(&mut self) -> BWERR0T_W<7> {
        BWERR0T_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr1t(&mut self) -> BWERR1T_W<8> {
        BWERR1T_W::new(self)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rst(&mut self) -> ECAT0RST_W<16> {
        ECAT0RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trap Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapclr](index.html) module"]
pub struct TRAPCLR_SPEC;
impl crate::RegisterSpec for TRAPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [trapclr::W](W) writer structure"]
impl crate::Writable for TRAPCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRAPCLR to value 0"]
impl crate::Resettable for TRAPCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
