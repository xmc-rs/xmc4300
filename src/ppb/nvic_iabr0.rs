#[doc = "Register `NVIC_IABR0` reader"]
pub type R = crate::R<NvicIabr0Spec>;
#[doc = "Register `NVIC_IABR0` writer"]
pub type W = crate::W<NvicIabr0Spec>;
#[doc = "Interrupt active flags:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Active {
    #[doc = "0: interrupt not active"]
    Value1 = 0,
    #[doc = "1: interrupt active"]
    Value2 = 1,
}
impl From<Active> for u32 {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Active {
    type Ux = u32;
}
impl crate::IsEnum for Active {}
#[doc = "Field `ACTIVE` reader - Interrupt active flags:"]
pub type ActiveR = crate::FieldReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Active> {
        match self.bits {
            0 => Some(Active::Value1),
            1 => Some(Active::Value2),
            _ => None,
        }
    }
    #[doc = "interrupt not active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Active::Value1
    }
    #[doc = "interrupt active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Active::Value2
    }
}
#[doc = "Field `ACTIVE` writer - Interrupt active flags:"]
pub type ActiveW<'a, REG> = crate::FieldWriter<'a, REG, 32, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt not active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Value1)
    }
    #[doc = "interrupt active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Value2)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<NvicIabr0Spec> {
        ActiveW::new(self, 0)
    }
}
#[doc = "Interrupt Active Bit Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIabr0Spec;
impl crate::RegisterSpec for NvicIabr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iabr0::R`](R) reader structure"]
impl crate::Readable for NvicIabr0Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_iabr0::W`](W) writer structure"]
impl crate::Writable for NvicIabr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IABR0 to value 0"]
impl crate::Resettable for NvicIabr0Spec {
    const RESET_VALUE: u32 = 0;
}
