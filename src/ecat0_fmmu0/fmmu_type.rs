#[doc = "Register `FMMU_TYPE` reader"]
pub type R = crate::R<FMMU_TYPE_SPEC>;
#[doc = "Field `R_ACC` reader - Read Access"]
pub type R_ACC_R = crate::BitReader<R_ACC_A>;
#[doc = "Read Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl R_ACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_ACC_A {
        match self.bits {
            false => R_ACC_A::VALUE1,
            true => R_ACC_A::VALUE2,
        }
    }
    #[doc = "Ignore mapping for read accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == R_ACC_A::VALUE1
    }
    #[doc = "Use mapping for read accesses"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == R_ACC_A::VALUE2
    }
}
#[doc = "Field `W_ACC` reader - Write Access"]
pub type W_ACC_R = crate::BitReader<W_ACC_A>;
#[doc = "Write Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl W_ACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W_ACC_A {
        match self.bits {
            false => W_ACC_A::VALUE1,
            true => W_ACC_A::VALUE2,
        }
    }
    #[doc = "Ignore mapping for write accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == W_ACC_A::VALUE1
    }
    #[doc = "Use mapping for write accesses"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == W_ACC_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Read Access"]
    #[inline(always)]
    pub fn r_acc(&self) -> R_ACC_R {
        R_ACC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Access"]
    #[inline(always)]
    pub fn w_acc(&self) -> W_ACC_R {
        W_ACC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "T0pe FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_type::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_TYPE_SPEC;
impl crate::RegisterSpec for FMMU_TYPE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_type::R`](R) reader structure"]
impl crate::Readable for FMMU_TYPE_SPEC {}
#[doc = "`reset()` method sets FMMU_TYPE to value 0"]
impl crate::Resettable for FMMU_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
