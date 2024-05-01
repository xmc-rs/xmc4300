#[doc = "Register `PROCON2` reader"]
pub type R = crate::R<Procon2Spec>;
#[doc = "Sector 0 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S0rom> for bool {
    #[inline(always)]
    fn from(variant: S0rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0ROM` reader - Sector 0 Locked Forever by User 2"]
pub type S0romR = crate::BitReader<S0rom>;
impl S0romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0rom {
        match self.bits {
            false => S0rom::Const0,
            true => S0rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S0rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S0rom::Const1
    }
}
#[doc = "Sector 1 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S1rom> for bool {
    #[inline(always)]
    fn from(variant: S1rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1ROM` reader - Sector 1 Locked Forever by User 2"]
pub type S1romR = crate::BitReader<S1rom>;
impl S1romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1rom {
        match self.bits {
            false => S1rom::Const0,
            true => S1rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S1rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S1rom::Const1
    }
}
#[doc = "Sector 2 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S2rom> for bool {
    #[inline(always)]
    fn from(variant: S2rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2ROM` reader - Sector 2 Locked Forever by User 2"]
pub type S2romR = crate::BitReader<S2rom>;
impl S2romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2rom {
        match self.bits {
            false => S2rom::Const0,
            true => S2rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S2rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S2rom::Const1
    }
}
#[doc = "Sector 3 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S3rom> for bool {
    #[inline(always)]
    fn from(variant: S3rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3ROM` reader - Sector 3 Locked Forever by User 2"]
pub type S3romR = crate::BitReader<S3rom>;
impl S3romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3rom {
        match self.bits {
            false => S3rom::Const0,
            true => S3rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S3rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S3rom::Const1
    }
}
#[doc = "Sector 4 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S4rom> for bool {
    #[inline(always)]
    fn from(variant: S4rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4ROM` reader - Sector 4 Locked Forever by User 2"]
pub type S4romR = crate::BitReader<S4rom>;
impl S4romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4rom {
        match self.bits {
            false => S4rom::Const0,
            true => S4rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S4rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S4rom::Const1
    }
}
#[doc = "Sector 5 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S5rom> for bool {
    #[inline(always)]
    fn from(variant: S5rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5ROM` reader - Sector 5 Locked Forever by User 2"]
pub type S5romR = crate::BitReader<S5rom>;
impl S5romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5rom {
        match self.bits {
            false => S5rom::Const0,
            true => S5rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S5rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S5rom::Const1
    }
}
#[doc = "Sector 6 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S6rom> for bool {
    #[inline(always)]
    fn from(variant: S6rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6ROM` reader - Sector 6 Locked Forever by User 2"]
pub type S6romR = crate::BitReader<S6rom>;
impl S6romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6rom {
        match self.bits {
            false => S6rom::Const0,
            true => S6rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S6rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S6rom::Const1
    }
}
#[doc = "Sector 7 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S7rom> for bool {
    #[inline(always)]
    fn from(variant: S7rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7ROM` reader - Sector 7 Locked Forever by User 2"]
pub type S7romR = crate::BitReader<S7rom>;
impl S7romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7rom {
        match self.bits {
            false => S7rom::Const0,
            true => S7rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S7rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S7rom::Const1
    }
}
#[doc = "Sector 8 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Const0 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Const1 = 1,
}
impl From<S8rom> for bool {
    #[inline(always)]
    fn from(variant: S8rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8ROM` reader - Sector 8 Locked Forever by User 2"]
pub type S8romR = crate::BitReader<S8rom>;
impl S8romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8rom {
        match self.bits {
            false => S8rom::Const0,
            true => S8rom::Const1,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == S8rom::Const0
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == S8rom::Const1
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s0rom(&self) -> S0romR {
        S0romR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s1rom(&self) -> S1romR {
        S1romR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s2rom(&self) -> S2romR {
        S2romR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s3rom(&self) -> S3romR {
        S3romR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s4rom(&self) -> S4romR {
        S4romR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s5rom(&self) -> S5romR {
        S5romR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s6rom(&self) -> S6romR {
        S6romR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s7rom(&self) -> S7romR {
        S7romR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s8rom(&self) -> S8romR {
        S8romR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Procon2Spec;
impl crate::RegisterSpec for Procon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procon2::R`](R) reader structure"]
impl crate::Readable for Procon2Spec {}
#[doc = "`reset()` method sets PROCON2 to value 0"]
impl crate::Resettable for Procon2Spec {
    const RESET_VALUE: u32 = 0;
}
