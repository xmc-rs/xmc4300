#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CGATSTAT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADCR {
    #[doc = "Gating de-asserted"]
    CONST_0,
    #[doc = "Gating asserted"]
    CONST_1,
}
impl VADCR {
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
            VADCR::CONST_0 => false,
            VADCR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VADCR {
        match value {
            false => VADCR::CONST_0,
            true => VADCR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == VADCR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == VADCR::CONST_1
    }
}
#[doc = "Possible values of the field `CCU40`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40R {
    #[doc = "Gating de-asserted"]
    CONST_0,
    #[doc = "Gating asserted"]
    CONST_1,
}
impl CCU40R {
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
            CCU40R::CONST_0 => false,
            CCU40R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU40R {
        match value {
            false => CCU40R::CONST_0,
            true => CCU40R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCU40R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCU40R::CONST_1
    }
}
#[doc = "Possible values of the field `CCU41`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41R {
    #[doc = "Gating de-asserted"]
    CONST_0,
    #[doc = "Gating asserted"]
    CONST_1,
}
impl CCU41R {
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
            CCU41R::CONST_0 => false,
            CCU41R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU41R {
        match value {
            false => CCU41R::CONST_0,
            true => CCU41R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCU41R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCU41R::CONST_1
    }
}
#[doc = "Possible values of the field `CCU80`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80R {
    #[doc = "Gating de-asserted"]
    CONST_0,
    #[doc = "Gating asserted"]
    CONST_1,
}
impl CCU80R {
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
            CCU80R::CONST_0 => false,
            CCU80R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU80R {
        match value {
            false => CCU80R::CONST_0,
            true => CCU80R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCU80R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCU80R::CONST_1
    }
}
#[doc = "Possible values of the field `POSIF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0R {
    #[doc = "Gating de-asserted"]
    CONST_0,
    #[doc = "Gating asserted"]
    CONST_1,
}
impl POSIF0R {
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
            POSIF0R::CONST_0 => false,
            POSIF0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POSIF0R {
        match value {
            false => POSIF0R::CONST_0,
            true => POSIF0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == POSIF0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == POSIF0R::CONST_1
    }
}
#[doc = "Possible values of the field `USIC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0R {
    #[doc = "Gating de-asserted"]
    CONST_0,
    #[doc = "Gating asserted"]
    CONST_1,
}
impl USIC0R {
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
            USIC0R::CONST_0 => false,
            USIC0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC0R {
        match value {
            false => USIC0R::CONST_0,
            true => USIC0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USIC0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USIC0R::CONST_1
    }
}
#[doc = "Possible values of the field `ERU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1R {
    #[doc = "Gating de-asserted"]
    CONST_0,
    #[doc = "Gating asserted"]
    CONST_1,
}
impl ERU1R {
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
            ERU1R::CONST_0 => false,
            ERU1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU1R {
        match value {
            false => ERU1R::CONST_0,
            true => ERU1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ERU1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ERU1R::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline]
    pub fn vadc(&self) -> VADCR {
        VADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline]
    pub fn ccu40(&self) -> CCU40R {
        CCU40R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline]
    pub fn ccu41(&self) -> CCU41R {
        CCU41R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline]
    pub fn ccu80(&self) -> CCU80R {
        CCU80R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline]
    pub fn posif0(&self) -> POSIF0R {
        POSIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline]
    pub fn usic0(&self) -> USIC0R {
        USIC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline]
    pub fn eru1(&self) -> ERU1R {
        ERU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
