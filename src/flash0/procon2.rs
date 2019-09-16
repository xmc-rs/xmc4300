#[doc = "Reader of register PROCON2"]
pub type R = crate::R<u32, super::PROCON2>;
#[doc = "Sector 0 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S0ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S0ROM_A) -> Self {
        match variant {
            S0ROM_A::CONST_0 => false,
            S0ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S0ROM`"]
pub type S0ROM_R = crate::R<bool, S0ROM_A>;
impl S0ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0ROM_A {
        match self.bits {
            false => S0ROM_A::CONST_0,
            true => S0ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S0ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S0ROM_A::CONST_1
    }
}
#[doc = "Sector 1 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S1ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S1ROM_A) -> Self {
        match variant {
            S1ROM_A::CONST_0 => false,
            S1ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S1ROM`"]
pub type S1ROM_R = crate::R<bool, S1ROM_A>;
impl S1ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1ROM_A {
        match self.bits {
            false => S1ROM_A::CONST_0,
            true => S1ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S1ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S1ROM_A::CONST_1
    }
}
#[doc = "Sector 2 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S2ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S2ROM_A) -> Self {
        match variant {
            S2ROM_A::CONST_0 => false,
            S2ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S2ROM`"]
pub type S2ROM_R = crate::R<bool, S2ROM_A>;
impl S2ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2ROM_A {
        match self.bits {
            false => S2ROM_A::CONST_0,
            true => S2ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S2ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S2ROM_A::CONST_1
    }
}
#[doc = "Sector 3 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S3ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S3ROM_A) -> Self {
        match variant {
            S3ROM_A::CONST_0 => false,
            S3ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S3ROM`"]
pub type S3ROM_R = crate::R<bool, S3ROM_A>;
impl S3ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3ROM_A {
        match self.bits {
            false => S3ROM_A::CONST_0,
            true => S3ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S3ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S3ROM_A::CONST_1
    }
}
#[doc = "Sector 4 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S4ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S4ROM_A) -> Self {
        match variant {
            S4ROM_A::CONST_0 => false,
            S4ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S4ROM`"]
pub type S4ROM_R = crate::R<bool, S4ROM_A>;
impl S4ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S4ROM_A {
        match self.bits {
            false => S4ROM_A::CONST_0,
            true => S4ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S4ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S4ROM_A::CONST_1
    }
}
#[doc = "Sector 5 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S5ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S5ROM_A) -> Self {
        match variant {
            S5ROM_A::CONST_0 => false,
            S5ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S5ROM`"]
pub type S5ROM_R = crate::R<bool, S5ROM_A>;
impl S5ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S5ROM_A {
        match self.bits {
            false => S5ROM_A::CONST_0,
            true => S5ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S5ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S5ROM_A::CONST_1
    }
}
#[doc = "Sector 6 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S6ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S6ROM_A) -> Self {
        match variant {
            S6ROM_A::CONST_0 => false,
            S6ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S6ROM`"]
pub type S6ROM_R = crate::R<bool, S6ROM_A>;
impl S6ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S6ROM_A {
        match self.bits {
            false => S6ROM_A::CONST_0,
            true => S6ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S6ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S6ROM_A::CONST_1
    }
}
#[doc = "Sector 7 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S7ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S7ROM_A) -> Self {
        match variant {
            S7ROM_A::CONST_0 => false,
            S7ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S7ROM`"]
pub type S7ROM_R = crate::R<bool, S7ROM_A>;
impl S7ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S7ROM_A {
        match self.bits {
            false => S7ROM_A::CONST_0,
            true => S7ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S7ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S7ROM_A::CONST_1
    }
}
#[doc = "Sector 8 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl From<S8ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S8ROM_A) -> Self {
        match variant {
            S8ROM_A::CONST_0 => false,
            S8ROM_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `S8ROM`"]
pub type S8ROM_R = crate::R<bool, S8ROM_A>;
impl S8ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S8ROM_A {
        match self.bits {
            false => S8ROM_A::CONST_0,
            true => S8ROM_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S8ROM_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S8ROM_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s0rom(&self) -> S0ROM_R {
        S0ROM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s1rom(&self) -> S1ROM_R {
        S1ROM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s2rom(&self) -> S2ROM_R {
        S2ROM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s3rom(&self) -> S3ROM_R {
        S3ROM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s4rom(&self) -> S4ROM_R {
        S4ROM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s5rom(&self) -> S5ROM_R {
        S5ROM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s6rom(&self) -> S6ROM_R {
        S6ROM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s7rom(&self) -> S7ROM_R {
        S7ROM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s8rom(&self) -> S8ROM_R {
        S8ROM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
