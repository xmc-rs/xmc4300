#[doc = "Register `NVIC_ICPR0` reader"]
pub type R = crate::R<NvicIcpr0Spec>;
#[doc = "Register `NVIC_ICPR0` writer"]
pub type W = crate::W<NvicIcpr0Spec>;
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Clrpend {
    #[doc = "0: interrupt is not pending"]
    Value3 = 0,
    #[doc = "1: interrupt is pending."]
    Value4 = 1,
}
impl From<Clrpend> for u32 {
    #[inline(always)]
    fn from(variant: Clrpend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clrpend {
    type Ux = u32;
}
impl crate::IsEnum for Clrpend {}
#[doc = "Field `CLRPEND` reader - Interrupt set-pending bits."]
pub type ClrpendR = crate::FieldReader<Clrpend>;
impl ClrpendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clrpend> {
        match self.bits {
            0 => Some(Clrpend::Value3),
            1 => Some(Clrpend::Value4),
            _ => None,
        }
    }
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Clrpend::Value3
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Clrpend::Value4
    }
}
#[doc = "Field `CLRPEND` writer - Interrupt set-pending bits."]
pub type ClrpendW<'a, REG> = crate::FieldWriter<'a, REG, 32, Clrpend>;
impl<'a, REG> ClrpendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Clrpend::Value3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Clrpend::Value4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn clrpend(&self) -> ClrpendR {
        ClrpendR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend(&mut self) -> ClrpendW<NvicIcpr0Spec> {
        ClrpendW::new(self, 0)
    }
}
#[doc = "Interrupt Clear-pending Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIcpr0Spec;
impl crate::RegisterSpec for NvicIcpr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icpr0::R`](R) reader structure"]
impl crate::Readable for NvicIcpr0Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_icpr0::W`](W) writer structure"]
impl crate::Writable for NvicIcpr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ICPR0 to value 0"]
impl crate::Resettable for NvicIcpr0Spec {
    const RESET_VALUE: u32 = 0;
}
