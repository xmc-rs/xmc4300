#[doc = "Register `FMMU_TYPE` reader"]
pub struct R(crate::R<FMMU_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Read Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_ACC_A {
    #[doc = "0: Ignore mapping for read accesses"]
    VALUE1 = 0,
    #[doc = "1: Use mapping for read accesses"]
    VALUE2 = 1,
}
impl From<R_ACC_A> for bool {
    #[inline(always)]
    fn from(variant: R_ACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_ACC` reader - Read Access"]
pub struct R_ACC_R(crate::FieldReader<bool, R_ACC_A>);
impl R_ACC_R {
    pub(crate) fn new(bits: bool) -> Self {
        R_ACC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_ACC_A {
        match self.bits {
            false => R_ACC_A::VALUE1,
            true => R_ACC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == R_ACC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == R_ACC_A::VALUE2
    }
}
impl core::ops::Deref for R_ACC_R {
    type Target = crate::FieldReader<bool, R_ACC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_ACC_A {
    #[doc = "0: Ignore mapping for write accesses"]
    VALUE1 = 0,
    #[doc = "1: Use mapping for write accesses"]
    VALUE2 = 1,
}
impl From<W_ACC_A> for bool {
    #[inline(always)]
    fn from(variant: W_ACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `W_ACC` reader - Write Access"]
pub struct W_ACC_R(crate::FieldReader<bool, W_ACC_A>);
impl W_ACC_R {
    pub(crate) fn new(bits: bool) -> Self {
        W_ACC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W_ACC_A {
        match self.bits {
            false => W_ACC_A::VALUE1,
            true => W_ACC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == W_ACC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == W_ACC_A::VALUE2
    }
}
impl core::ops::Deref for W_ACC_R {
    type Target = crate::FieldReader<bool, W_ACC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read Access"]
    #[inline(always)]
    pub fn r_acc(&self) -> R_ACC_R {
        R_ACC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Access"]
    #[inline(always)]
    pub fn w_acc(&self) -> W_ACC_R {
        W_ACC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "T0pe FMMU y\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_type](index.html) module"]
pub struct FMMU_TYPE_SPEC;
impl crate::RegisterSpec for FMMU_TYPE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fmmu_type::R](R) reader structure"]
impl crate::Readable for FMMU_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_TYPE to value 0"]
impl crate::Resettable for FMMU_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
