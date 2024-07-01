#[doc = "Register `NVIC_IABR2` reader"]
pub type R = crate::R<NVIC_IABR2_SPEC>;
#[doc = "Register `NVIC_IABR2` writer"]
pub type W = crate::W<NVIC_IABR2_SPEC>;
#[doc = "Interrupt active flags:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ACTIVE_A {
    #[doc = "0: interrupt not active"]
    VALUE1 = 0,
    #[doc = "1: interrupt active"]
    VALUE2 = 1,
}
impl From<ACTIVE_A> for u32 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTIVE_A {
    type Ux = u32;
}
impl crate::IsEnum for ACTIVE_A {}
#[doc = "Field `ACTIVE` reader - Interrupt active flags:"]
pub type ACTIVE_R = crate::FieldReader<ACTIVE_A>;
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACTIVE_A> {
        match self.bits {
            0 => Some(ACTIVE_A::VALUE1),
            1 => Some(ACTIVE_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "interrupt not active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACTIVE_A::VALUE1
    }
    #[doc = "interrupt active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACTIVE_A::VALUE2
    }
}
#[doc = "Field `ACTIVE` writer - Interrupt active flags:"]
pub type ACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, ACTIVE_A>;
impl<'a, REG> ACTIVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt not active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIVE_A::VALUE1)
    }
    #[doc = "interrupt active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIVE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<NVIC_IABR2_SPEC> {
        ACTIVE_W::new(self, 0)
    }
}
#[doc = "Interrupt Active Bit Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`nvic_iabr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iabr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IABR2_SPEC;
impl crate::RegisterSpec for NVIC_IABR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iabr2::R`](R) reader structure"]
impl crate::Readable for NVIC_IABR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_iabr2::W`](W) writer structure"]
impl crate::Writable for NVIC_IABR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IABR2 to value 0"]
impl crate::Resettable for NVIC_IABR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
