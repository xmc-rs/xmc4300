#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTAT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VADCRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADCRSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl VADCRSR {
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
            VADCRSR::CONST_0 => false,
            VADCRSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VADCRSR {
        match value {
            false => VADCRSR::CONST_0,
            true => VADCRSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == VADCRSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == VADCRSR::CONST_1
    }
}
#[doc = "Possible values of the field `CCU40RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl CCU40RSR {
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
            CCU40RSR::CONST_0 => false,
            CCU40RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU40RSR {
        match value {
            false => CCU40RSR::CONST_0,
            true => CCU40RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCU40RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCU40RSR::CONST_1
    }
}
#[doc = "Possible values of the field `CCU41RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl CCU41RSR {
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
            CCU41RSR::CONST_0 => false,
            CCU41RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU41RSR {
        match value {
            false => CCU41RSR::CONST_0,
            true => CCU41RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCU41RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCU41RSR::CONST_1
    }
}
#[doc = "Possible values of the field `CCU80RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl CCU80RSR {
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
            CCU80RSR::CONST_0 => false,
            CCU80RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU80RSR {
        match value {
            false => CCU80RSR::CONST_0,
            true => CCU80RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCU80RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCU80RSR::CONST_1
    }
}
#[doc = "Possible values of the field `USIC0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl USIC0RSR {
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
            USIC0RSR::CONST_0 => false,
            USIC0RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC0RSR {
        match value {
            false => USIC0RSR::CONST_0,
            true => USIC0RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USIC0RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USIC0RSR::CONST_1
    }
}
#[doc = "Possible values of the field `ERU1RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl ERU1RSR {
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
            ERU1RSR::CONST_0 => false,
            ERU1RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU1RSR {
        match value {
            false => ERU1RSR::CONST_0,
            true => ERU1RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ERU1RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ERU1RSR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VADC Reset Status"]
    #[inline]
    pub fn vadcrs(&self) -> VADCRSR {
        VADCRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline]
    pub fn ccu40rs(&self) -> CCU40RSR {
        CCU40RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline]
    pub fn ccu41rs(&self) -> CCU41RSR {
        CCU41RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline]
    pub fn ccu80rs(&self) -> CCU80RSR {
        CCU80RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline]
    pub fn usic0rs(&self) -> USIC0RSR {
        USIC0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline]
    pub fn eru1rs(&self) -> ERU1RSR {
        ERU1RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
