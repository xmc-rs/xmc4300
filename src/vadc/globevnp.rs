#[doc = "Register `GLOBEVNP` reader"]
pub struct R(crate::R<GLOBEVNP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBEVNP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBEVNP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBEVNP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBEVNP` writer"]
pub struct W(crate::W<GLOBEVNP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBEVNP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GLOBEVNP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBEVNP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEV0NP` reader - Service Request Node Pointer Backgr. Source"]
pub type SEV0NP_R = crate::FieldReader<u8, SEV0NP_A>;
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
impl SEV0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEV0NP_A> {
        match self.bits {
            0 => Some(SEV0NP_A::VALUE1),
            3 => Some(SEV0NP_A::VALUE2),
            4 => Some(SEV0NP_A::VALUE3),
            7 => Some(SEV0NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEV0NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEV0NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SEV0NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SEV0NP_A::VALUE4
    }
}
#[doc = "Field `SEV0NP` writer - Service Request Node Pointer Backgr. Source"]
pub type SEV0NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBEVNP_SPEC, u8, SEV0NP_A, 4, O>;
impl<'a, const O: u8> SEV0NP_W<'a, O> {
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE4)
    }
}
#[doc = "Field `REV0NP` reader - Service Request Node Pointer Backgr. Result"]
pub type REV0NP_R = crate::FieldReader<u8, REV0NP_A>;
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
impl REV0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REV0NP_A> {
        match self.bits {
            0 => Some(REV0NP_A::VALUE1),
            3 => Some(REV0NP_A::VALUE2),
            4 => Some(REV0NP_A::VALUE3),
            7 => Some(REV0NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REV0NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REV0NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == REV0NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == REV0NP_A::VALUE4
    }
}
#[doc = "Field `REV0NP` writer - Service Request Node Pointer Backgr. Result"]
pub type REV0NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBEVNP_SPEC, u8, REV0NP_A, 4, O>;
impl<'a, const O: u8> REV0NP_W<'a, O> {
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
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
    #[must_use]
    pub fn sev0np(&mut self) -> SEV0NP_W<0> {
        SEV0NP_W::new(self)
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline(always)]
    #[must_use]
    pub fn rev0np(&mut self) -> REV0NP_W<16> {
        REV0NP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Event Node Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globevnp](index.html) module"]
pub struct GLOBEVNP_SPEC;
impl crate::RegisterSpec for GLOBEVNP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globevnp::R](R) reader structure"]
impl crate::Readable for GLOBEVNP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globevnp::W](W) writer structure"]
impl crate::Writable for GLOBEVNP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBEVNP to value 0"]
impl crate::Resettable for GLOBEVNP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
