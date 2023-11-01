#[doc = "Register `TRAPSET` writer"]
pub type W = crate::W<TRAPSET_SPEC>;
#[doc = "OSC_HP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<SOSCWDGT_AW> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` writer - OSC_HP Oscillator Watchdog Trap Set"]
pub type SOSCWDGT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SOSCWDGT_AW>;
impl<'a, REG, const O: u8> SOSCWDGT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_AW::CONST_1)
    }
}
#[doc = "System VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<SVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Set"]
pub type SVCOLCKT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SVCOLCKT_AW>;
impl<'a, REG, const O: u8> SVCOLCKT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_AW::CONST_1)
    }
}
#[doc = "USB VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<UVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Set"]
pub type UVCOLCKT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UVCOLCKT_AW>;
impl<'a, REG, const O: u8> UVCOLCKT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_AW::CONST_1)
    }
}
#[doc = "Parity Error Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<PET_AW> for bool {
    #[inline(always)]
    fn from(variant: PET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Set"]
pub type PET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PET_AW>;
impl<'a, REG, const O: u8> PET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PET_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PET_AW::CONST_1)
    }
}
#[doc = "Brown Out Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<BRWNT_AW> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Set"]
pub type BRWNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRWNT_AW>;
impl<'a, REG, const O: u8> BRWNT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_AW::CONST_1)
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDT_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<ULPWDT_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDT` writer - OSC_ULP Oscillator Watchdog Trap Set"]
pub type ULPWDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ULPWDT_AW>;
impl<'a, REG, const O: u8> ULPWDT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDT_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDT_AW::CONST_1)
    }
}
#[doc = "Peripheral Bridge 0 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<BWERR0T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Set"]
pub type BWERR0T_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BWERR0T_AW>;
impl<'a, REG, const O: u8> BWERR0T_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_AW::CONST_1)
    }
}
#[doc = "Peripheral Bridge 1 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Set trap request"]
    CONST_1 = 1,
}
impl From<BWERR1T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Set"]
pub type BWERR1T_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BWERR1T_AW>;
impl<'a, REG, const O: u8> BWERR1T_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_AW::CONST_1)
    }
}
#[doc = "EtherCat Reset 0 Trap Set\n\nValue on reset: 0"]
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
#[doc = "Field `ECAT0RST` writer - EtherCat Reset 0 Trap Set"]
pub type ECAT0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ECAT0RST_AW>;
impl<'a, REG, const O: u8> ECAT0RST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RST_AW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RST_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W<TRAPSET_SPEC, 0> {
        SOSCWDGT_W::new(self)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W<TRAPSET_SPEC, 2> {
        SVCOLCKT_W::new(self)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W<TRAPSET_SPEC, 3> {
        UVCOLCKT_W::new(self)
    }
    #[doc = "Bit 4 - Parity Error Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn pet(&mut self) -> PET_W<TRAPSET_SPEC, 4> {
        PET_W::new(self)
    }
    #[doc = "Bit 5 - Brown Out Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn brwnt(&mut self) -> BRWNT_W<TRAPSET_SPEC, 5> {
        BRWNT_W::new(self)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdt(&mut self) -> ULPWDT_W<TRAPSET_SPEC, 6> {
        ULPWDT_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr0t(&mut self) -> BWERR0T_W<TRAPSET_SPEC, 7> {
        BWERR0T_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr1t(&mut self) -> BWERR1T_W<TRAPSET_SPEC, 8> {
        BWERR1T_W::new(self)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rst(&mut self) -> ECAT0RST_W<TRAPSET_SPEC, 16> {
        ECAT0RST_W::new(self)
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
#[doc = "Trap Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAPSET_SPEC;
impl crate::RegisterSpec for TRAPSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trapset::W`](W) writer structure"]
impl crate::Writable for TRAPSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRAPSET to value 0"]
impl crate::Resettable for TRAPSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
