#[doc = "Reader of register PROCON0"]
pub type R = crate::R<u32, super::PROCON0>;
#[doc = "Sector 0 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S0L_A> for bool {
    #[inline(always)]
    fn from(variant: S0L_A) -> Self {
        match variant {
            S0L_A::CONST_0 => false,
            S0L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S0L`"]
pub type S0L_R = crate::R<bool, S0L_A>;
impl S0L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0L_A {
        match self.bits {
            false => S0L_A::CONST_0,
            true => S0L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S0L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S0L_A::CONST_1
    }
}
#[doc = "Sector 1 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S1L_A> for bool {
    #[inline(always)]
    fn from(variant: S1L_A) -> Self {
        match variant {
            S1L_A::CONST_0 => false,
            S1L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S1L`"]
pub type S1L_R = crate::R<bool, S1L_A>;
impl S1L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1L_A {
        match self.bits {
            false => S1L_A::CONST_0,
            true => S1L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S1L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S1L_A::CONST_1
    }
}
#[doc = "Sector 2 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S2L_A> for bool {
    #[inline(always)]
    fn from(variant: S2L_A) -> Self {
        match variant {
            S2L_A::CONST_0 => false,
            S2L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S2L`"]
pub type S2L_R = crate::R<bool, S2L_A>;
impl S2L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2L_A {
        match self.bits {
            false => S2L_A::CONST_0,
            true => S2L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S2L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S2L_A::CONST_1
    }
}
#[doc = "Sector 3 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S3L_A> for bool {
    #[inline(always)]
    fn from(variant: S3L_A) -> Self {
        match variant {
            S3L_A::CONST_0 => false,
            S3L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S3L`"]
pub type S3L_R = crate::R<bool, S3L_A>;
impl S3L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3L_A {
        match self.bits {
            false => S3L_A::CONST_0,
            true => S3L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S3L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S3L_A::CONST_1
    }
}
#[doc = "Sector 4 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S4L_A> for bool {
    #[inline(always)]
    fn from(variant: S4L_A) -> Self {
        match variant {
            S4L_A::CONST_0 => false,
            S4L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S4L`"]
pub type S4L_R = crate::R<bool, S4L_A>;
impl S4L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S4L_A {
        match self.bits {
            false => S4L_A::CONST_0,
            true => S4L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S4L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S4L_A::CONST_1
    }
}
#[doc = "Sector 5 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S5L_A> for bool {
    #[inline(always)]
    fn from(variant: S5L_A) -> Self {
        match variant {
            S5L_A::CONST_0 => false,
            S5L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S5L`"]
pub type S5L_R = crate::R<bool, S5L_A>;
impl S5L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S5L_A {
        match self.bits {
            false => S5L_A::CONST_0,
            true => S5L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S5L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S5L_A::CONST_1
    }
}
#[doc = "Sector 6 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S6L_A> for bool {
    #[inline(always)]
    fn from(variant: S6L_A) -> Self {
        match variant {
            S6L_A::CONST_0 => false,
            S6L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S6L`"]
pub type S6L_R = crate::R<bool, S6L_A>;
impl S6L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S6L_A {
        match self.bits {
            false => S6L_A::CONST_0,
            true => S6L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S6L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S6L_A::CONST_1
    }
}
#[doc = "Sector 7 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S7L_A> for bool {
    #[inline(always)]
    fn from(variant: S7L_A) -> Self {
        match variant {
            S7L_A::CONST_0 => false,
            S7L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S7L`"]
pub type S7L_R = crate::R<bool, S7L_A>;
impl S7L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S7L_A {
        match self.bits {
            false => S7L_A::CONST_0,
            true => S7L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S7L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S7L_A::CONST_1
    }
}
#[doc = "Sector 8 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8L_A {
    #[doc = "0: No write protection is configured for sector n."]
    CONST_0,
    #[doc = "1: Write protection is configured for sector n."]
    CONST_1,
}
impl From<S8L_A> for bool {
    #[inline(always)]
    fn from(variant: S8L_A) -> Self {
        match variant {
            S8L_A::CONST_0 => false,
            S8L_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S8L`"]
pub type S8L_R = crate::R<bool, S8L_A>;
impl S8L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S8L_A {
        match self.bits {
            false => S8L_A::CONST_0,
            true => S8L_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S8L_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S8L_A::CONST_1
    }
}
#[doc = "Read Protection Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPRO_A {
    #[doc = "0: No read protection configured"]
    CONST_0,
    #[doc = "1: Read protection and global write protection is configured by user 0 (master user)"]
    CONST_1,
}
impl From<RPRO_A> for bool {
    #[inline(always)]
    fn from(variant: RPRO_A) -> Self {
        match variant {
            RPRO_A::CONST_0 => false,
            RPRO_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RPRO`"]
pub type RPRO_R = crate::R<bool, RPRO_A>;
impl RPRO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPRO_A {
        match self.bits {
            false => RPRO_A::CONST_0,
            true => RPRO_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RPRO_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RPRO_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s0l(&self) -> S0L_R {
        S0L_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s1l(&self) -> S1L_R {
        S1L_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s2l(&self) -> S2L_R {
        S2L_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s3l(&self) -> S3L_R {
        S3L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s4l(&self) -> S4L_R {
        S4L_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s5l(&self) -> S5L_R {
        S5L_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s6l(&self) -> S6L_R {
        S6L_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s7l(&self) -> S7L_R {
        S7L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s8l(&self) -> S8L_R {
        S8L_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Read Protection Configuration"]
    #[inline(always)]
    pub fn rpro(&self) -> RPRO_R {
        RPRO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
