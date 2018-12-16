#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PROCON2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `S0ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S0ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S0ROMR::CONST_0 => false,
            S0ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0ROMR {
        match value {
            false => S0ROMR::CONST_0,
            true => S0ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S0ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S0ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S1ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S1ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S1ROMR::CONST_0 => false,
            S1ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1ROMR {
        match value {
            false => S1ROMR::CONST_0,
            true => S1ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S1ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S1ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S2ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S2ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S2ROMR::CONST_0 => false,
            S2ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2ROMR {
        match value {
            false => S2ROMR::CONST_0,
            true => S2ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S2ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S2ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S3ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S3ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S3ROMR::CONST_0 => false,
            S3ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3ROMR {
        match value {
            false => S3ROMR::CONST_0,
            true => S3ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S3ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S3ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S4ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S4ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S4ROMR::CONST_0 => false,
            S4ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S4ROMR {
        match value {
            false => S4ROMR::CONST_0,
            true => S4ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S4ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S4ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S5ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S5ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S5ROMR::CONST_0 => false,
            S5ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S5ROMR {
        match value {
            false => S5ROMR::CONST_0,
            true => S5ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S5ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S5ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S6ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S6ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S6ROMR::CONST_0 => false,
            S6ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S6ROMR {
        match value {
            false => S6ROMR::CONST_0,
            true => S6ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S6ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S6ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S7ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S7ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S7ROMR::CONST_0 => false,
            S7ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S7ROMR {
        match value {
            false => S7ROMR::CONST_0,
            true => S7ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S7ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S7ROMR::CONST_1
    }
}
#[doc = "Possible values of the field `S8ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8ROMR {
    #[doc = "No ROM functionality configured for sector n."]
    CONST_0,
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    CONST_1,
}
impl S8ROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            S8ROMR::CONST_0 => false,
            S8ROMR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S8ROMR {
        match value {
            false => S8ROMR::CONST_0,
            true => S8ROMR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == S8ROMR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == S8ROMR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sector 0 Locked Forever by User 2"]
    #[inline]
    pub fn s0rom(&self) -> S0ROMR {
        S0ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Sector 1 Locked Forever by User 2"]
    #[inline]
    pub fn s1rom(&self) -> S1ROMR {
        S1ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Sector 2 Locked Forever by User 2"]
    #[inline]
    pub fn s2rom(&self) -> S2ROMR {
        S2ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Sector 3 Locked Forever by User 2"]
    #[inline]
    pub fn s3rom(&self) -> S3ROMR {
        S3ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Sector 4 Locked Forever by User 2"]
    #[inline]
    pub fn s4rom(&self) -> S4ROMR {
        S4ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Sector 5 Locked Forever by User 2"]
    #[inline]
    pub fn s5rom(&self) -> S5ROMR {
        S5ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Sector 6 Locked Forever by User 2"]
    #[inline]
    pub fn s6rom(&self) -> S6ROMR {
        S6ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Sector 7 Locked Forever by User 2"]
    #[inline]
    pub fn s7rom(&self) -> S7ROMR {
        S7ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Sector 8 Locked Forever by User 2"]
    #[inline]
    pub fn s8rom(&self) -> S8ROMR {
        S8ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
