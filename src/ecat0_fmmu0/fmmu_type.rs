#[doc = "Reader of register FMMU_TYPE"]
pub type R = crate::R<u8, super::FMMU_TYPE>;
#[doc = "Read Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_ACC_A {
    #[doc = "0: Ignore mapping for read accesses"]
    VALUE1,
    #[doc = "1: Use mapping for read accesses"]
    VALUE2,
}
impl From<R_ACC_A> for bool {
    #[inline(always)]
    fn from(variant: R_ACC_A) -> Self {
        match variant {
            R_ACC_A::VALUE1 => false,
            R_ACC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `R_ACC`"]
pub type R_ACC_R = crate::R<bool, R_ACC_A>;
impl R_ACC_R {
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
        *self == R_ACC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == R_ACC_A::VALUE2
    }
}
#[doc = "Write Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_ACC_A {
    #[doc = "0: Ignore mapping for write accesses"]
    VALUE1,
    #[doc = "1: Use mapping for write accesses"]
    VALUE2,
}
impl From<W_ACC_A> for bool {
    #[inline(always)]
    fn from(variant: W_ACC_A) -> Self {
        match variant {
            W_ACC_A::VALUE1 => false,
            W_ACC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `W_ACC`"]
pub type W_ACC_R = crate::R<bool, W_ACC_A>;
impl W_ACC_R {
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
        *self == W_ACC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == W_ACC_A::VALUE2
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
