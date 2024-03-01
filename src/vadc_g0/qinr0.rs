#[doc = "Register `QINR0` writer"]
pub type W = crate::W<Qinr0Spec>;
#[doc = "Field `REQCHNR` writer - Request Channel Number"]
pub type ReqchnrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Refill\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf {
    #[doc = "0: No refill: this queue entry is converted once and then invalidated"]
    Value1 = 0,
    #[doc = "1: Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    Value2 = 1,
}
impl From<Rf> for bool {
    #[inline(always)]
    fn from(variant: Rf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF` writer - Refill"]
pub type RfW<'a, REG> = crate::BitWriter<'a, REG, Rf>;
impl<'a, REG> RfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No refill: this queue entry is converted once and then invalidated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf::Value1)
    }
    #[doc = "Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rf::Value2)
    }
}
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ensi {
    #[doc = "0: No request source interrupt"]
    Value1 = 0,
    #[doc = "1: A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    Value2 = 1,
}
impl From<Ensi> for bool {
    #[inline(always)]
    fn from(variant: Ensi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` writer - Enable Source Interrupt"]
pub type EnsiW<'a, REG> = crate::BitWriter<'a, REG, Ensi>;
impl<'a, REG> EnsiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ensi::Value1)
    }
    #[doc = "A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ensi::Value2)
    }
}
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extr {
    #[doc = "0: A valid queue entry immediately leads to a conversion request."]
    Value1 = 0,
    #[doc = "1: A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    Value2 = 1,
}
impl From<Extr> for bool {
    #[inline(always)]
    fn from(variant: Extr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTR` writer - External Trigger"]
pub type ExtrW<'a, REG> = crate::BitWriter<'a, REG, Extr>;
impl<'a, REG> ExtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A valid queue entry immediately leads to a conversion request."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Extr::Value1)
    }
    #[doc = "A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Extr::Value2)
    }
}
impl W {
    #[doc = "Bits 0:4 - Request Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn reqchnr(&mut self) -> ReqchnrW<Qinr0Spec> {
        ReqchnrW::new(self, 0)
    }
    #[doc = "Bit 5 - Refill"]
    #[inline(always)]
    #[must_use]
    pub fn rf(&mut self) -> RfW<Qinr0Spec> {
        RfW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ensi(&mut self) -> EnsiW<Qinr0Spec> {
        EnsiW::new(self, 6)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn extr(&mut self) -> ExtrW<Qinr0Spec> {
        ExtrW::new(self, 7)
    }
}
#[doc = "Queue 0 Input Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qinr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qinr0Spec;
impl crate::RegisterSpec for Qinr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qinr0::W`](W) writer structure"]
impl crate::Writable for Qinr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QINR0 to value 0"]
impl crate::Resettable for Qinr0Spec {
    const RESET_VALUE: u32 = 0;
}
