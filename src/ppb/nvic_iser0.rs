#[doc = "Register `NVIC_ISER0` reader"]
pub type R = crate::R<NVIC_ISER0_SPEC>;
#[doc = "Register `NVIC_ISER0` writer"]
pub type W = crate::W<NVIC_ISER0_SPEC>;
#[doc = "Interrupt set-enable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SETENA_A {
    #[doc = "0: interrupt disabled"]
    VALUE3 = 0,
    #[doc = "1: interrupt enabled."]
    VALUE4 = 1,
}
impl From<SETENA_A> for u32 {
    #[inline(always)]
    fn from(variant: SETENA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SETENA_A {
    type Ux = u32;
}
impl crate::IsEnum for SETENA_A {}
#[doc = "Field `SETENA` reader - Interrupt set-enable bits"]
pub type SETENA_R = crate::FieldReader<SETENA_A>;
impl SETENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SETENA_A> {
        match self.bits {
            0 => Some(SETENA_A::VALUE3),
            1 => Some(SETENA_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SETENA_A::VALUE3
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SETENA_A::VALUE4
    }
}
#[doc = "Field `SETENA` writer - Interrupt set-enable bits"]
pub type SETENA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, SETENA_A>;
impl<'a, REG> SETENA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SETENA_A::VALUE3)
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SETENA_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn setena(&mut self) -> SETENA_W<NVIC_ISER0_SPEC> {
        SETENA_W::new(self, 0)
    }
}
#[doc = "Interrupt Set-enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`nvic_iser0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iser0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ISER0_SPEC;
impl crate::RegisterSpec for NVIC_ISER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iser0::R`](R) reader structure"]
impl crate::Readable for NVIC_ISER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_iser0::W`](W) writer structure"]
impl crate::Writable for NVIC_ISER0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISER0 to value 0"]
impl crate::Resettable for NVIC_ISER0_SPEC {
    const RESET_VALUE: u32 = 0;
}
