#[doc = "Register `CPACR` reader"]
pub type R = crate::R<CPACR_SPEC>;
#[doc = "Register `CPACR` writer"]
pub type W = crate::W<CPACR_SPEC>;
#[doc = "Access privileges for coprocessor 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP10_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2 = 1,
    #[doc = "3: Full access."]
    VALUE4 = 3,
}
impl From<CP10_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CP10_A {
    type Ux = u8;
}
impl crate::IsEnum for CP10_A {}
#[doc = "Field `CP10` reader - Access privileges for coprocessor 10"]
pub type CP10_R = crate::FieldReader<CP10_A>;
impl CP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CP10_A> {
        match self.bits {
            0 => Some(CP10_A::VALUE1),
            1 => Some(CP10_A::VALUE2),
            3 => Some(CP10_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CP10_A::VALUE1
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CP10_A::VALUE2
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CP10_A::VALUE4
    }
}
#[doc = "Field `CP10` writer - Access privileges for coprocessor 10"]
pub type CP10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CP10_A>;
impl<'a, REG> CP10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CP10_A::VALUE1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CP10_A::VALUE2)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CP10_A::VALUE4)
    }
}
#[doc = "Access privileges for coprocessor 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP11_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2 = 1,
    #[doc = "3: Full access."]
    VALUE4 = 3,
}
impl From<CP11_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CP11_A {
    type Ux = u8;
}
impl crate::IsEnum for CP11_A {}
#[doc = "Field `CP11` reader - Access privileges for coprocessor 11"]
pub type CP11_R = crate::FieldReader<CP11_A>;
impl CP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CP11_A> {
        match self.bits {
            0 => Some(CP11_A::VALUE1),
            1 => Some(CP11_A::VALUE2),
            3 => Some(CP11_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CP11_A::VALUE1
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CP11_A::VALUE2
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CP11_A::VALUE4
    }
}
#[doc = "Field `CP11` writer - Access privileges for coprocessor 11"]
pub type CP11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CP11_A>;
impl<'a, REG> CP11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CP11_A::VALUE1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CP11_A::VALUE2)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CP11_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    #[must_use]
    pub fn cp10(&mut self) -> CP10_W<CPACR_SPEC> {
        CP10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    #[must_use]
    pub fn cp11(&mut self) -> CP11_W<CPACR_SPEC> {
        CP11_W::new(self, 22)
    }
}
#[doc = "Coprocessor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPACR_SPEC;
impl crate::RegisterSpec for CPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpacr::R`](R) reader structure"]
impl crate::Readable for CPACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpacr::W`](W) writer structure"]
impl crate::Writable for CPACR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CPACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
