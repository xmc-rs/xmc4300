#[doc = "Register `NVIC_ISER0` reader"]
pub type R = crate::R<NvicIser0Spec>;
#[doc = "Register `NVIC_ISER0` writer"]
pub type W = crate::W<NvicIser0Spec>;
#[doc = "Interrupt set-enable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Setena {
    #[doc = "0: interrupt disabled"]
    Value3 = 0,
    #[doc = "1: interrupt enabled."]
    Value4 = 1,
}
impl From<Setena> for u32 {
    #[inline(always)]
    fn from(variant: Setena) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setena {
    type Ux = u32;
}
#[doc = "Field `SETENA` reader - Interrupt set-enable bits"]
pub type SetenaR = crate::FieldReader<Setena>;
impl SetenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setena> {
        match self.bits {
            0 => Some(Setena::Value3),
            1 => Some(Setena::Value4),
            _ => None,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Setena::Value3
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Setena::Value4
    }
}
#[doc = "Field `SETENA` writer - Interrupt set-enable bits"]
pub type SetenaW<'a, REG> = crate::FieldWriter<'a, REG, 32, Setena>;
impl<'a, REG> SetenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Setena::Value3)
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Setena::Value4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    pub fn setena(&self) -> SetenaR {
        SetenaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn setena(&mut self) -> SetenaW<NvicIser0Spec> {
        SetenaW::new(self, 0)
    }
}
#[doc = "Interrupt Set-enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIser0Spec;
impl crate::RegisterSpec for NvicIser0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iser0::R`](R) reader structure"]
impl crate::Readable for NvicIser0Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_iser0::W`](W) writer structure"]
impl crate::Writable for NvicIser0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISER0 to value 0"]
impl crate::Resettable for NvicIser0Spec {
    const RESET_VALUE: u32 = 0;
}
