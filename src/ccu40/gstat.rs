#[doc = "Reader of register GSTAT"]
pub type R = crate::R<u32, super::GSTAT>;
#[doc = "CC40 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S0I_A> for bool {
    #[inline(always)]
    fn from(variant: S0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S0I`"]
pub type S0I_R = crate::R<bool, S0I_A>;
impl S0I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0I_A {
        match self.bits {
            false => S0I_A::VALUE1,
            true => S0I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0I_A::VALUE2
    }
}
#[doc = "CC41 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S1I_A> for bool {
    #[inline(always)]
    fn from(variant: S1I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1I`"]
pub type S1I_R = crate::R<bool, S1I_A>;
impl S1I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1I_A {
        match self.bits {
            false => S1I_A::VALUE1,
            true => S1I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1I_A::VALUE2
    }
}
#[doc = "CC42 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S2I_A> for bool {
    #[inline(always)]
    fn from(variant: S2I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S2I`"]
pub type S2I_R = crate::R<bool, S2I_A>;
impl S2I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2I_A {
        match self.bits {
            false => S2I_A::VALUE1,
            true => S2I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2I_A::VALUE2
    }
}
#[doc = "CC43 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S3I_A> for bool {
    #[inline(always)]
    fn from(variant: S3I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S3I`"]
pub type S3I_R = crate::R<bool, S3I_A>;
impl S3I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3I_A {
        match self.bits {
            false => S3I_A::VALUE1,
            true => S3I_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3I_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3I_A::VALUE2
    }
}
#[doc = "Prescaler Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRB_A {
    #[doc = "0: Prescaler is stopped"]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running"]
    VALUE2 = 1,
}
impl From<PRB_A> for bool {
    #[inline(always)]
    fn from(variant: PRB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRB`"]
pub type PRB_R = crate::R<bool, PRB_A>;
impl PRB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRB_A {
        match self.bits {
            false => PRB_A::VALUE1,
            true => PRB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRB_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - CC40 IDLE status"]
    #[inline(always)]
    pub fn s0i(&self) -> S0I_R {
        S0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CC41 IDLE status"]
    #[inline(always)]
    pub fn s1i(&self) -> S1I_R {
        S1I_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CC42 IDLE status"]
    #[inline(always)]
    pub fn s2i(&self) -> S2I_R {
        S2I_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CC43 IDLE status"]
    #[inline(always)]
    pub fn s3i(&self) -> S3I_R {
        S3I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Prescaler Run Bit"]
    #[inline(always)]
    pub fn prb(&self) -> PRB_R {
        PRB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
