#[doc = "Register `GLOBEVNP` reader"]
pub type R = crate::R<GLOBEVNP_SPEC>;
#[doc = "Register `GLOBEVNP` writer"]
pub type W = crate::W<GLOBEVNP_SPEC>;
#[doc = "Service Request Node Pointer Backgr. Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEV0NP_A {
    #[doc = "0: Select shared service request line 0 of common service request group 0"]
    VALUE1 = 0,
    #[doc = "3: Select shared service request line 3 of common service request group 0"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0 of common service request group 1"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3 of common service request group 1"]
    VALUE4 = 7,
}
impl From<SEV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: SEV0NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEV0NP_A {
    type Ux = u8;
}
impl crate::IsEnum for SEV0NP_A {}
#[doc = "Field `SEV0NP` reader - Service Request Node Pointer Backgr. Source"]
pub type SEV0NP_R = crate::FieldReader<SEV0NP_A>;
impl SEV0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEV0NP_A> {
        match self.bits {
            0 => Some(SEV0NP_A::VALUE1),
            3 => Some(SEV0NP_A::VALUE2),
            4 => Some(SEV0NP_A::VALUE3),
            7 => Some(SEV0NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEV0NP_A::VALUE1
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEV0NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SEV0NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SEV0NP_A::VALUE4
    }
}
#[doc = "Field `SEV0NP` writer - Service Request Node Pointer Backgr. Source"]
pub type SEV0NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SEV0NP_A>;
impl<'a, REG> SEV0NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0NP_A::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0NP_A::VALUE4)
    }
}
#[doc = "Service Request Node Pointer Backgr. Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV0NP_A {
    #[doc = "0: Select shared service request line 0 of common service request group 0"]
    VALUE1 = 0,
    #[doc = "3: Select shared service request line 3 of common service request group 0"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0 of common service request group 1"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3 of common service request group 1"]
    VALUE4 = 7,
}
impl From<REV0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: REV0NP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV0NP_A {
    type Ux = u8;
}
impl crate::IsEnum for REV0NP_A {}
#[doc = "Field `REV0NP` reader - Service Request Node Pointer Backgr. Result"]
pub type REV0NP_R = crate::FieldReader<REV0NP_A>;
impl REV0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV0NP_A> {
        match self.bits {
            0 => Some(REV0NP_A::VALUE1),
            3 => Some(REV0NP_A::VALUE2),
            4 => Some(REV0NP_A::VALUE3),
            7 => Some(REV0NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV0NP_A::VALUE1
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV0NP_A::VALUE2
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV0NP_A::VALUE3
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV0NP_A::VALUE4
    }
}
#[doc = "Field `REV0NP` writer - Service Request Node Pointer Backgr. Result"]
pub type REV0NP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, REV0NP_A>;
impl<'a, REG> REV0NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV0NP_A::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(REV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(REV0NP_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline(always)]
    pub fn sev0np(&self) -> SEV0NP_R {
        SEV0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline(always)]
    pub fn rev0np(&self) -> REV0NP_R {
        REV0NP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline(always)]
    pub fn sev0np(&mut self) -> SEV0NP_W<GLOBEVNP_SPEC> {
        SEV0NP_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline(always)]
    pub fn rev0np(&mut self) -> REV0NP_W<GLOBEVNP_SPEC> {
        REV0NP_W::new(self, 16)
    }
}
#[doc = "Global Event Node Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`globevnp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`globevnp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBEVNP_SPEC;
impl crate::RegisterSpec for GLOBEVNP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globevnp::R`](R) reader structure"]
impl crate::Readable for GLOBEVNP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globevnp::W`](W) writer structure"]
impl crate::Writable for GLOBEVNP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBEVNP to value 0"]
impl crate::Resettable for GLOBEVNP_SPEC {
    const RESET_VALUE: u32 = 0;
}
