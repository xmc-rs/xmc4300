#[doc = "Register `CPACR` reader"]
pub type R = crate::R<CpacrSpec>;
#[doc = "Register `CPACR` writer"]
pub type W = crate::W<CpacrSpec>;
#[doc = "Access privileges for coprocessor 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cp10 {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    Value1 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    Value2 = 1,
    #[doc = "3: Full access."]
    Value4 = 3,
}
impl From<Cp10> for u8 {
    #[inline(always)]
    fn from(variant: Cp10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cp10 {
    type Ux = u8;
}
impl crate::IsEnum for Cp10 {}
#[doc = "Field `CP10` reader - Access privileges for coprocessor 10"]
pub type Cp10R = crate::FieldReader<Cp10>;
impl Cp10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cp10> {
        match self.bits {
            0 => Some(Cp10::Value1),
            1 => Some(Cp10::Value2),
            3 => Some(Cp10::Value4),
            _ => None,
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cp10::Value1
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cp10::Value2
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cp10::Value4
    }
}
#[doc = "Field `CP10` writer - Access privileges for coprocessor 10"]
pub type Cp10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cp10>;
impl<'a, REG> Cp10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cp10::Value1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cp10::Value2)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cp10::Value4)
    }
}
#[doc = "Access privileges for coprocessor 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cp11 {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    Value1 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    Value2 = 1,
    #[doc = "3: Full access."]
    Value4 = 3,
}
impl From<Cp11> for u8 {
    #[inline(always)]
    fn from(variant: Cp11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cp11 {
    type Ux = u8;
}
impl crate::IsEnum for Cp11 {}
#[doc = "Field `CP11` reader - Access privileges for coprocessor 11"]
pub type Cp11R = crate::FieldReader<Cp11>;
impl Cp11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cp11> {
        match self.bits {
            0 => Some(Cp11::Value1),
            1 => Some(Cp11::Value2),
            3 => Some(Cp11::Value4),
            _ => None,
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cp11::Value1
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cp11::Value2
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cp11::Value4
    }
}
#[doc = "Field `CP11` writer - Access privileges for coprocessor 11"]
pub type Cp11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cp11>;
impl<'a, REG> Cp11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cp11::Value1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cp11::Value2)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cp11::Value4)
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&self) -> Cp10R {
        Cp10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&self) -> Cp11R {
        Cp11R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    #[must_use]
    pub fn cp10(&mut self) -> Cp10W<CpacrSpec> {
        Cp10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    #[must_use]
    pub fn cp11(&mut self) -> Cp11W<CpacrSpec> {
        Cp11W::new(self, 22)
    }
}
#[doc = "Coprocessor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpacrSpec;
impl crate::RegisterSpec for CpacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpacr::R`](R) reader structure"]
impl crate::Readable for CpacrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpacr::W`](W) writer structure"]
impl crate::Writable for CpacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CpacrSpec {
    const RESET_VALUE: u32 = 0;
}
