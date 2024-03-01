#[doc = "Register `NVIC_ICER1` reader"]
pub type R = crate::R<NvicIcer1Spec>;
#[doc = "Register `NVIC_ICER1` writer"]
pub type W = crate::W<NvicIcer1Spec>;
#[doc = "Interrupt clear-enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Clrena {
    #[doc = "0: interrupt disabled"]
    Value3 = 0,
    #[doc = "1: interrupt enabled."]
    Value4 = 1,
}
impl From<Clrena> for u32 {
    #[inline(always)]
    fn from(variant: Clrena) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clrena {
    type Ux = u32;
}
#[doc = "Field `CLRENA` reader - Interrupt clear-enable bits."]
pub type ClrenaR = crate::FieldReader<Clrena>;
impl ClrenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clrena> {
        match self.bits {
            0 => Some(Clrena::Value3),
            1 => Some(Clrena::Value4),
            _ => None,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Clrena::Value3
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Clrena::Value4
    }
}
#[doc = "Field `CLRENA` writer - Interrupt clear-enable bits."]
pub type ClrenaW<'a, REG> = crate::FieldWriter<'a, REG, 32, Clrena>;
impl<'a, REG> ClrenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Clrena::Value3)
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Clrena::Value4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline(always)]
    pub fn clrena(&self) -> ClrenaR {
        ClrenaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn clrena(&mut self) -> ClrenaW<NvicIcer1Spec> {
        ClrenaW::new(self, 0)
    }
}
#[doc = "Interrupt Clear-enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIcer1Spec;
impl crate::RegisterSpec for NvicIcer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icer1::R`](R) reader structure"]
impl crate::Readable for NvicIcer1Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_icer1::W`](W) writer structure"]
impl crate::Writable for NvicIcer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ICER1 to value 0"]
impl crate::Resettable for NvicIcer1Spec {
    const RESET_VALUE: u32 = 0;
}
