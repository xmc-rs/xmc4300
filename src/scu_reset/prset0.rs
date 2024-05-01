#[doc = "Register `PRSET0` writer"]
pub type W = crate::W<Prset0Spec>;
#[doc = "VADC Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vadcrs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Vadcrs> for bool {
    #[inline(always)]
    fn from(variant: Vadcrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` writer - VADC Reset Assert"]
pub type VadcrsW<'a, REG> = crate::BitWriter<'a, REG, Vadcrs>;
impl<'a, REG> VadcrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vadcrs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vadcrs::Const1)
    }
}
#[doc = "CCU40 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu40rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Ccu40rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu40rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` writer - CCU40 Reset Assert"]
pub type Ccu40rsW<'a, REG> = crate::BitWriter<'a, REG, Ccu40rs>;
impl<'a, REG> Ccu40rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu40rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu40rs::Const1)
    }
}
#[doc = "CCU41 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu41rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Ccu41rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu41rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` writer - CCU41 Reset Assert"]
pub type Ccu41rsW<'a, REG> = crate::BitWriter<'a, REG, Ccu41rs>;
impl<'a, REG> Ccu41rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu41rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu41rs::Const1)
    }
}
#[doc = "CCU80 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu80rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Ccu80rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu80rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` writer - CCU80 Reset Assert"]
pub type Ccu80rsW<'a, REG> = crate::BitWriter<'a, REG, Ccu80rs>;
impl<'a, REG> Ccu80rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu80rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu80rs::Const1)
    }
}
#[doc = "USIC0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic0rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Usic0rs> for bool {
    #[inline(always)]
    fn from(variant: Usic0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` writer - USIC0 Reset Assert"]
pub type Usic0rsW<'a, REG> = crate::BitWriter<'a, REG, Usic0rs>;
impl<'a, REG> Usic0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0rs::Const1)
    }
}
#[doc = "ERU1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru1rs {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Assert reset"]
    Const1 = 1,
}
impl From<Eru1rs> for bool {
    #[inline(always)]
    fn from(variant: Eru1rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` writer - ERU1 Reset Assert"]
pub type Eru1rsW<'a, REG> = crate::BitWriter<'a, REG, Eru1rs>;
impl<'a, REG> Eru1rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eru1rs::Const0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eru1rs::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn vadcrs(&mut self) -> VadcrsW<Prset0Spec> {
        VadcrsW::new(self, 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40rs(&mut self) -> Ccu40rsW<Prset0Spec> {
        Ccu40rsW::new(self, 2)
    }
    #[doc = "Bit 3 - CCU41 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41rs(&mut self) -> Ccu41rsW<Prset0Spec> {
        Ccu41rsW::new(self, 3)
    }
    #[doc = "Bit 7 - CCU80 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80rs(&mut self) -> Ccu80rsW<Prset0Spec> {
        Ccu80rsW::new(self, 7)
    }
    #[doc = "Bit 11 - USIC0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usic0rs(&mut self) -> Usic0rsW<Prset0Spec> {
        Usic0rsW::new(self, 11)
    }
    #[doc = "Bit 16 - ERU1 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eru1rs(&mut self) -> Eru1rsW<Prset0Spec> {
        Eru1rsW::new(self, 16)
    }
}
#[doc = "RCU Peripheral 0 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prset0Spec;
impl crate::RegisterSpec for Prset0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset0::W`](W) writer structure"]
impl crate::Writable for Prset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSET0 to value 0"]
impl crate::Resettable for Prset0Spec {
    const RESET_VALUE: u32 = 0;
}
