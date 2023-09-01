#[doc = "Register `RSTSET` writer"]
pub type W = crate::W<RSTSET_SPEC>;
#[doc = "Set Hibernate Wake-up Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBWK_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset status bit"]
    CONST_1 = 1,
}
impl From<HIBWK_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBWK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` writer - Set Hibernate Wake-up Reset Status"]
pub type HIBWK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HIBWK_AW>;
impl<'a, REG, const O: u8> HIBWK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HIBWK_AW::CONST_0)
    }
    #[doc = "Assert reset status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBWK_AW::CONST_1)
    }
}
#[doc = "Set Hibernate Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<HIBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` writer - Set Hibernate Reset"]
pub type HIBRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HIBRS_AW>;
impl<'a, REG, const O: u8> HIBRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(HIBRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBRS_AW::CONST_1)
    }
}
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKEN_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Enable reset when Lockup gets asserted"]
    CONST_1 = 1,
}
impl From<LCKEN_AW> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKEN` writer - Enable Lockup Reset"]
pub type LCKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LCKEN_AW>;
impl<'a, REG, const O: u8> LCKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LCKEN_AW::CONST_0)
    }
    #[doc = "Enable reset when Lockup gets asserted"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LCKEN_AW::CONST_1)
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset status bit"]
    CONST_1 = 1,
}
impl From<ECAT0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` writer - ECAT0 Reset Status Information"]
pub type ECAT0RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ECAT0RS_AW>;
impl<'a, REG, const O: u8> ECAT0RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RS_AW::CONST_0)
    }
    #[doc = "Assert reset status bit"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RS_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 8 - Set Hibernate Wake-up Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn hibwk(&mut self) -> HIBWK_W<RSTSET_SPEC, 8> {
        HIBWK_W::new(self)
    }
    #[doc = "Bit 9 - Set Hibernate Reset"]
    #[inline(always)]
    #[must_use]
    pub fn hibrs(&mut self) -> HIBRS_W<RSTSET_SPEC, 9> {
        HIBRS_W::new(self)
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lcken(&mut self) -> LCKEN_W<RSTSET_SPEC, 10> {
        LCKEN_W::new(self)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rs(&mut self) -> ECAT0RS_W<RSTSET_SPEC, 12> {
        ECAT0RS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCU Reset Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTSET_SPEC;
impl crate::RegisterSpec for RSTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstset::W`](W) writer structure"]
impl crate::Writable for RSTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTSET to value 0"]
impl crate::Resettable for RSTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
