#[doc = "Reader of register GCST"]
pub type R = crate::R<u32, super::GCST>;
#[doc = "Slice 0 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0SS_A> for bool {
    #[inline(always)]
    fn from(variant: S0SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S0SS`"]
pub type S0SS_R = crate::R<bool, S0SS_A>;
impl S0SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0SS_A {
        match self.bits {
            false => S0SS_A::VALUE1,
            true => S0SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0SS_A::VALUE2
    }
}
#[doc = "Slice 0 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S0DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S0DSS`"]
pub type S0DSS_R = crate::R<bool, S0DSS_A>;
impl S0DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0DSS_A {
        match self.bits {
            false => S0DSS_A::VALUE1,
            true => S0DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0DSS_A::VALUE2
    }
}
#[doc = "Slice 0 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S0PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S0PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S0PSS`"]
pub type S0PSS_R = crate::R<bool, S0PSS_A>;
impl S0PSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0PSS_A {
        match self.bits {
            false => S0PSS_A::VALUE1,
            true => S0PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0PSS_A::VALUE2
    }
}
#[doc = "Slice 1 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1SS_A> for bool {
    #[inline(always)]
    fn from(variant: S1SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1SS`"]
pub type S1SS_R = crate::R<bool, S1SS_A>;
impl S1SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1SS_A {
        match self.bits {
            false => S1SS_A::VALUE1,
            true => S1SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1SS_A::VALUE2
    }
}
#[doc = "Slice 1 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S1DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1DSS`"]
pub type S1DSS_R = crate::R<bool, S1DSS_A>;
impl S1DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1DSS_A {
        match self.bits {
            false => S1DSS_A::VALUE1,
            true => S1DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1DSS_A::VALUE2
    }
}
#[doc = "Slice 1 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S1PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S1PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1PSS`"]
pub type S1PSS_R = crate::R<bool, S1PSS_A>;
impl S1PSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1PSS_A {
        match self.bits {
            false => S1PSS_A::VALUE1,
            true => S1PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1PSS_A::VALUE2
    }
}
#[doc = "Slice 2 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2SS_A> for bool {
    #[inline(always)]
    fn from(variant: S2SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S2SS`"]
pub type S2SS_R = crate::R<bool, S2SS_A>;
impl S2SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2SS_A {
        match self.bits {
            false => S2SS_A::VALUE1,
            true => S2SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2SS_A::VALUE2
    }
}
#[doc = "Slice 2 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S2DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S2DSS`"]
pub type S2DSS_R = crate::R<bool, S2DSS_A>;
impl S2DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2DSS_A {
        match self.bits {
            false => S2DSS_A::VALUE1,
            true => S2DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2DSS_A::VALUE2
    }
}
#[doc = "Slice 2 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S2PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S2PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S2PSS`"]
pub type S2PSS_R = crate::R<bool, S2PSS_A>;
impl S2PSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2PSS_A {
        match self.bits {
            false => S2PSS_A::VALUE1,
            true => S2PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2PSS_A::VALUE2
    }
}
#[doc = "Slice 3 shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3SS_A {
    #[doc = "0: Shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3SS_A> for bool {
    #[inline(always)]
    fn from(variant: S3SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S3SS`"]
pub type S3SS_R = crate::R<bool, S3SS_A>;
impl S3SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3SS_A {
        match self.bits {
            false => S3SS_A::VALUE1,
            true => S3SS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3SS_A::VALUE2
    }
}
#[doc = "Slice 3 Dither shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3DSS_A {
    #[doc = "0: Dither shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Dither shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3DSS_A> for bool {
    #[inline(always)]
    fn from(variant: S3DSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S3DSS`"]
pub type S3DSS_R = crate::R<bool, S3DSS_A>;
impl S3DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3DSS_A {
        match self.bits {
            false => S3DSS_A::VALUE1,
            true => S3DSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3DSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3DSS_A::VALUE2
    }
}
#[doc = "Slice 3 Prescaler shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3PSS_A {
    #[doc = "0: Prescaler shadow transfer has not been requested"]
    VALUE1 = 0,
    #[doc = "1: Prescaler shadow transfer has been requested"]
    VALUE2 = 1,
}
impl From<S3PSS_A> for bool {
    #[inline(always)]
    fn from(variant: S3PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S3PSS`"]
pub type S3PSS_R = crate::R<bool, S3PSS_A>;
impl S3PSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3PSS_A {
        match self.bits {
            false => S3PSS_A::VALUE1,
            true => S3PSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3PSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3PSS_A::VALUE2
    }
}
#[doc = "Reader of field `CC40ST`"]
pub type CC40ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC41ST`"]
pub type CC41ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC42ST`"]
pub type CC42ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC43ST`"]
pub type CC43ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Slice 0 shadow transfer status"]
    #[inline(always)]
    pub fn s0ss(&self) -> S0SS_R {
        S0SS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s0dss(&self) -> S0DSS_R {
        S0DSS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s0pss(&self) -> S0PSS_R {
        S0PSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer status"]
    #[inline(always)]
    pub fn s1ss(&self) -> S1SS_R {
        S1SS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s1dss(&self) -> S1DSS_R {
        S1DSS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s1pss(&self) -> S1PSS_R {
        S1PSS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer status"]
    #[inline(always)]
    pub fn s2ss(&self) -> S2SS_R {
        S2SS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s2dss(&self) -> S2DSS_R {
        S2DSS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s2pss(&self) -> S2PSS_R {
        S2PSS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer status"]
    #[inline(always)]
    pub fn s3ss(&self) -> S3SS_R {
        S3SS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer status"]
    #[inline(always)]
    pub fn s3dss(&self) -> S3DSS_R {
        S3DSS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer status"]
    #[inline(always)]
    pub fn s3pss(&self) -> S3PSS_R {
        S3PSS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slice 0 status bit"]
    #[inline(always)]
    pub fn cc40st(&self) -> CC40ST_R {
        CC40ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slice 1 status bit"]
    #[inline(always)]
    pub fn cc41st(&self) -> CC41ST_R {
        CC41ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slice 2 status bit"]
    #[inline(always)]
    pub fn cc42st(&self) -> CC42ST_R {
        CC42ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Slice 3 status bit"]
    #[inline(always)]
    pub fn cc43st(&self) -> CC43ST_R {
        CC43ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
