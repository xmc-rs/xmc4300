#[doc = "Register `PRSET0` writer"]
pub type W = crate::W<PRSET0_SPEC>;
#[doc = "VADC Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADCRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<VADCRS_AW> for bool {
    #[inline(always)]
    fn from(variant: VADCRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` writer - VADC Reset Assert"]
pub type VADCRS_W<'a, REG> = crate::BitWriter<'a, REG, VADCRS_AW>;
impl<'a, REG> VADCRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(VADCRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(VADCRS_AW::CONST_1)
    }
}
#[doc = "CCU40 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<CCU40RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` writer - CCU40 Reset Assert"]
pub type CCU40RS_W<'a, REG> = crate::BitWriter<'a, REG, CCU40RS_AW>;
impl<'a, REG> CCU40RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCU40RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU40RS_AW::CONST_1)
    }
}
#[doc = "CCU41 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<CCU41RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` writer - CCU41 Reset Assert"]
pub type CCU41RS_W<'a, REG> = crate::BitWriter<'a, REG, CCU41RS_AW>;
impl<'a, REG> CCU41RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCU41RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU41RS_AW::CONST_1)
    }
}
#[doc = "CCU80 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<CCU80RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` writer - CCU80 Reset Assert"]
pub type CCU80RS_W<'a, REG> = crate::BitWriter<'a, REG, CCU80RS_AW>;
impl<'a, REG> CCU80RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCU80RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU80RS_AW::CONST_1)
    }
}
#[doc = "USIC0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<USIC0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` writer - USIC0 Reset Assert"]
pub type USIC0RS_W<'a, REG> = crate::BitWriter<'a, REG, USIC0RS_AW>;
impl<'a, REG> USIC0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0RS_AW::CONST_1)
    }
}
#[doc = "ERU1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<ERU1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` writer - ERU1 Reset Assert"]
pub type ERU1RS_W<'a, REG> = crate::BitWriter<'a, REG, ERU1RS_AW>;
impl<'a, REG> ERU1RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ERU1RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ERU1RS_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn vadcrs(&mut self) -> VADCRS_W<PRSET0_SPEC> {
        VADCRS_W::new(self, 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40rs(&mut self) -> CCU40RS_W<PRSET0_SPEC> {
        CCU40RS_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCU41 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41rs(&mut self) -> CCU41RS_W<PRSET0_SPEC> {
        CCU41RS_W::new(self, 3)
    }
    #[doc = "Bit 7 - CCU80 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80rs(&mut self) -> CCU80RS_W<PRSET0_SPEC> {
        CCU80RS_W::new(self, 7)
    }
    #[doc = "Bit 11 - USIC0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usic0rs(&mut self) -> USIC0RS_W<PRSET0_SPEC> {
        USIC0RS_W::new(self, 11)
    }
    #[doc = "Bit 16 - ERU1 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eru1rs(&mut self) -> ERU1RS_W<PRSET0_SPEC> {
        ERU1RS_W::new(self, 16)
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
#[doc = "RCU Peripheral 0 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSET0_SPEC;
impl crate::RegisterSpec for PRSET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset0::W`](W) writer structure"]
impl crate::Writable for PRSET0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSET0 to value 0"]
impl crate::Resettable for PRSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
