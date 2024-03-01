#[doc = "Register `GLOBEVNP` reader"]
pub type R = crate::R<GlobevnpSpec>;
#[doc = "Register `GLOBEVNP` writer"]
pub type W = crate::W<GlobevnpSpec>;
#[doc = "Service Request Node Pointer Backgr. Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sev0np {
    #[doc = "0: Select shared service request line 0 of common service request group 0"]
    Value1 = 0,
    #[doc = "3: Select shared service request line 3 of common service request group 0"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0 of common service request group 1"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3 of common service request group 1"]
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
#[doc = "Field `SEV0NP` reader - Service Request Node Pointer Backgr. Source"]
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
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sev0np::Value1
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sev0np::Value2
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sev0np::Value3
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sev0np::Value4
    }
}
#[doc = "Field `SEV0NP` writer - Service Request Node Pointer Backgr. Source"]
pub type Sev0npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sev0np>;
impl<'a, REG> Sev0npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0np::Value4)
    }
}
#[doc = "Service Request Node Pointer Backgr. Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rev0np {
    #[doc = "0: Select shared service request line 0 of common service request group 0"]
    Value1 = 0,
    #[doc = "3: Select shared service request line 3 of common service request group 0"]
    Value2 = 3,
    #[doc = "4: Select shared service request line 0 of common service request group 1"]
    Value3 = 4,
    #[doc = "7: Select shared service request line 3 of common service request group 1"]
    Value4 = 7,
}
impl From<Rev0np> for u8 {
    #[inline(always)]
    fn from(variant: Rev0np) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rev0np {
    type Ux = u8;
}
#[doc = "Field `REV0NP` reader - Service Request Node Pointer Backgr. Result"]
pub type Rev0npR = crate::FieldReader<Rev0np>;
impl Rev0npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rev0np> {
        match self.bits {
            0 => Some(Rev0np::Value1),
            3 => Some(Rev0np::Value2),
            4 => Some(Rev0np::Value3),
            7 => Some(Rev0np::Value4),
            _ => None,
        }
    }
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev0np::Value1
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev0np::Value2
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rev0np::Value3
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rev0np::Value4
    }
}
#[doc = "Field `REV0NP` writer - Service Request Node Pointer Backgr. Result"]
pub type Rev0npW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rev0np>;
impl<'a, REG> Rev0npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0np::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline(always)]
    pub fn sev0np(&self) -> Sev0npR {
        Sev0npR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline(always)]
    pub fn rev0np(&self) -> Rev0npR {
        Rev0npR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline(always)]
    #[must_use]
    pub fn sev0np(&mut self) -> Sev0npW<GlobevnpSpec> {
        Sev0npW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline(always)]
    #[must_use]
    pub fn rev0np(&mut self) -> Rev0npW<GlobevnpSpec> {
        Rev0npW::new(self, 16)
    }
}
#[doc = "Global Event Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globevnp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globevnp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobevnpSpec;
impl crate::RegisterSpec for GlobevnpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globevnp::R`](R) reader structure"]
impl crate::Readable for GlobevnpSpec {}
#[doc = "`write(|w| ..)` method takes [`globevnp::W`](W) writer structure"]
impl crate::Writable for GlobevnpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBEVNP to value 0"]
impl crate::Resettable for GlobevnpSpec {
    const RESET_VALUE: u32 = 0;
}
