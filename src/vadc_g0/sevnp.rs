#[doc = "Register `SEVNP` reader"]
pub struct R(crate::R<SEVNP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEVNP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEVNP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEVNP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEVNP` writer"]
pub struct W(crate::W<SEVNP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEVNP_SPEC>;
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
impl From<crate::W<SEVNP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEVNP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEV0NP` reader - Service Request Node Pointer Source Event i"]
pub type SEV0NP_R = crate::FieldReader<u8, SEV0NP_A>;
#[doc = "Service Request Node Pointer Source Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEV0NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
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
#[doc = "Field `SEV0NP` writer - Service Request Node Pointer Source Event i"]
pub type SEV0NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEVNP_SPEC, u8, SEV0NP_A, 4, O>;
impl<'a, const O: u8> SEV0NP_W<'a, O> {
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SEV0NP_A::VALUE4)
    }
}
#[doc = "Field `SEV1NP` reader - Service Request Node Pointer Source Event i"]
pub type SEV1NP_R = crate::FieldReader<u8, SEV1NP_A>;
#[doc = "Service Request Node Pointer Source Event i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEV1NP_A {
    #[doc = "0: Select service request line 0 of group x"]
    VALUE1 = 0,
    #[doc = "3: Select service request line 3 of group x"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
}
impl From<SEV1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: SEV1NP_A) -> Self {
        variant as _
    }
}
impl SEV1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEV1NP_A> {
        match self.bits {
            0 => Some(SEV1NP_A::VALUE1),
            3 => Some(SEV1NP_A::VALUE2),
            4 => Some(SEV1NP_A::VALUE3),
            7 => Some(SEV1NP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEV1NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEV1NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SEV1NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SEV1NP_A::VALUE4
    }
}
#[doc = "Field `SEV1NP` writer - Service Request Node Pointer Source Event i"]
pub type SEV1NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEVNP_SPEC, u8, SEV1NP_A, 4, O>;
impl<'a, const O: u8> SEV1NP_W<'a, O> {
    #[doc = "Select service request line 0 of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SEV1NP_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev0np(&self) -> SEV0NP_R {
        SEV0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    pub fn sev1np(&self) -> SEV1NP_R {
        SEV1NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    #[must_use]
    pub fn sev0np(&mut self) -> SEV0NP_W<0> {
        SEV0NP_W::new(self)
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline(always)]
    #[must_use]
    pub fn sev1np(&mut self) -> SEV1NP_W<4> {
        SEV1NP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Event Node Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sevnp](index.html) module"]
pub struct SEVNP_SPEC;
impl crate::RegisterSpec for SEVNP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sevnp::R](R) reader structure"]
impl crate::Readable for SEVNP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sevnp::W](W) writer structure"]
impl crate::Writable for SEVNP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEVNP to value 0"]
impl crate::Resettable for SEVNP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
