#[doc = "Register `QINR0` writer"]
pub type W = crate::W<QINR0_SPEC>;
#[doc = "Field `REQCHNR` writer - Request Channel Number"]
pub type REQCHNR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Refill\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_A {
    #[doc = "0: No refill: this queue entry is converted once and then invalidated"]
    VALUE1 = 0,
    #[doc = "1: Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    VALUE2 = 1,
}
impl From<RF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF` writer - Refill"]
pub type RF_W<'a, REG> = crate::BitWriter<'a, REG, RF_A>;
impl<'a, REG> RF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No refill: this queue entry is converted once and then invalidated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RF_A::VALUE1)
    }
    #[doc = "Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RF_A::VALUE2)
    }
}
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSI_A {
    #[doc = "0: No request source interrupt"]
    VALUE1 = 0,
    #[doc = "1: A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    VALUE2 = 1,
}
impl From<ENSI_A> for bool {
    #[inline(always)]
    fn from(variant: ENSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` writer - Enable Source Interrupt"]
pub type ENSI_W<'a, REG> = crate::BitWriter<'a, REG, ENSI_A>;
impl<'a, REG> ENSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENSI_A::VALUE1)
    }
    #[doc = "A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENSI_A::VALUE2)
    }
}
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTR_A {
    #[doc = "0: A valid queue entry immediately leads to a conversion request."]
    VALUE1 = 0,
    #[doc = "1: A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    VALUE2 = 1,
}
impl From<EXTR_A> for bool {
    #[inline(always)]
    fn from(variant: EXTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTR` writer - External Trigger"]
pub type EXTR_W<'a, REG> = crate::BitWriter<'a, REG, EXTR_A>;
impl<'a, REG> EXTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A valid queue entry immediately leads to a conversion request."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTR_A::VALUE1)
    }
    #[doc = "A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTR_A::VALUE2)
    }
}
impl W {
    #[doc = "Bits 0:4 - Request Channel Number"]
    #[inline(always)]
    pub fn reqchnr(&mut self) -> REQCHNR_W<QINR0_SPEC> {
        REQCHNR_W::new(self, 0)
    }
    #[doc = "Bit 5 - Refill"]
    #[inline(always)]
    pub fn rf(&mut self) -> RF_W<QINR0_SPEC> {
        RF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&mut self) -> ENSI_W<QINR0_SPEC> {
        ENSI_W::new(self, 6)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn extr(&mut self) -> EXTR_W<QINR0_SPEC> {
        EXTR_W::new(self, 7)
    }
}
#[doc = "Queue 0 Input Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qinr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QINR0_SPEC;
impl crate::RegisterSpec for QINR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qinr0::W`](W) writer structure"]
impl crate::Writable for QINR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QINR0 to value 0"]
impl crate::Resettable for QINR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
