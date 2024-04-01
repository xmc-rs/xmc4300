#[doc = "Register `SEVNP` reader"]
pub type R = crate::R<SevnpSpec>;
#[doc = "Register `SEVNP` writer"]
pub type W = crate::W<SevnpSpec>;
#[doc = "Service Request Node Pointer Source Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sev0np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Sev0np> for u8 {
    #[inline(always)]
    fn from(variant: Sev0np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sev0np {
    type Ux = u8;
}
impl crate::IsEnum for Sev0np {}
#[doc = "Field `SEV0NP` reader - Service Request Node Pointer Source Event i"]
pub type Sev0npR = crate::FieldReader<Sev0np>;
impl Sev0npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sev0np> {
        match self.bits {
            0 => Some(Sev0np::Value1),
            3 => Some(Sev0np::Value2),
            4 => Some(Sev0np::Value3),
            7 => Some(Sev0np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sev0np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sev0np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sev0np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sev0np::Value4
    }
}
#[doc = "Field `SEV0NP` writer - Service Request Node Pointer Source Event i"]
pub type Sev0npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sev0np>;
impl<'a, REG> Sev0npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value4)
    }
}
#[doc = "Service Request Node Pointer Source Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sev1np {
    #[doc = "0: Select service request line 0 of group x"]
    Value1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3"]
    Value4 = 7,
}
impl From<Sev1np> for u8 {
    #[inline(always)]
    fn from(variant: Sev1np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sev1np {
    type Ux = u8;
}
impl crate::IsEnum for Sev1np {}
#[doc = "Field `SEV1NP` reader - Service Request Node Pointer Source Event i"]
pub type Sev1npR = crate::FieldReader<Sev1np>;
impl Sev1npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sev1np> {
        match self.bits {
            0 => Some(Sev1np::Value1),
            3 => Some(Sev1np::Value2),
            4 => Some(Sev1np::Value3),
            7 => Some(Sev1np::Value4),
            _ => None,
        }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sev1np::Value1
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sev1np::Value2
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sev1np::Value3
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sev1np::Value4
    }
}
#[doc = "Field `SEV1NP` writer - Service Request Node Pointer Source Event i"]
pub type Sev1npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sev1np>;
impl<'a, REG> Sev1npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1np::Value1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1np::Value2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1np::Value3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1np::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev0np(&self) -> Sev0npR {
        Sev0npR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev1np(&self) -> Sev1npR {
        Sev1npR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    #[must_use]
    pub fn sev0np(&mut self) -> Sev0npW<SevnpSpec> {
        Sev0npW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    #[must_use]
    pub fn sev1np(&mut self) -> Sev1npW<SevnpSpec> {
        Sev1npW::new(self, 4)
    }
}
#[doc = "Source Event Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sevnp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sevnp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SevnpSpec;
impl crate::RegisterSpec for SevnpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sevnp::R`](R) reader structure"]
impl crate::Readable for SevnpSpec {}
#[doc = "`write(|w| ..)` method takes [`sevnp::W`](W) writer structure"]
impl crate::Writable for SevnpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEVNP to value 0"]
impl crate::Resettable for SevnpSpec {
    const RESET_VALUE: u32 = 0;
}
