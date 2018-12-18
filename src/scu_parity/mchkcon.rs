#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCHKCON {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SELPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELPSR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELPSR {
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
            SELPSR::CONST_0 => false,
            SELPSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELPSR {
        match value {
            false => SELPSR::CONST_0,
            true => SELPSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELPSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELPSR::CONST_1
    }
}
#[doc = "Possible values of the field `SELDS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELDS1R {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELDS1R {
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
            SELDS1R::CONST_0 => false,
            SELDS1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELDS1R {
        match value {
            false => SELDS1R::CONST_0,
            true => SELDS1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELDS1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELDS1R::CONST_1
    }
}
#[doc = "Possible values of the field `USIC0DRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0DRAR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl USIC0DRAR {
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
            USIC0DRAR::CONST_0 => false,
            USIC0DRAR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC0DRAR {
        match value {
            false => USIC0DRAR::CONST_0,
            true => USIC0DRAR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USIC0DRAR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USIC0DRAR::CONST_1
    }
}
#[doc = "Possible values of the field `USIC1DRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1DRAR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl USIC1DRAR {
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
            USIC1DRAR::CONST_0 => false,
            USIC1DRAR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC1DRAR {
        match value {
            false => USIC1DRAR::CONST_0,
            true => USIC1DRAR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USIC1DRAR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USIC1DRAR::CONST_1
    }
}
#[doc = "Possible values of the field `MCANDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCANDRAR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl MCANDRAR {
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
            MCANDRAR::CONST_0 => false,
            MCANDRAR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCANDRAR {
        match value {
            false => MCANDRAR::CONST_0,
            true => MCANDRAR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == MCANDRAR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == MCANDRAR::CONST_1
    }
}
#[doc = "Possible values of the field `PPRFDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRFDRAR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl PPRFDRAR {
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
            PPRFDRAR::CONST_0 => false,
            PPRFDRAR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPRFDRAR {
        match value {
            false => PPRFDRAR::CONST_0,
            true => PPRFDRAR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PPRFDRAR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PPRFDRAR::CONST_1
    }
}
#[doc = "Possible values of the field `SELUSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELUSBR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELUSBR {
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
            SELUSBR::CONST_0 => false,
            SELUSBR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELUSBR {
        match value {
            false => SELUSBR::CONST_0,
            true => SELUSBR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELUSBR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELUSBR::CONST_1
    }
}
#[doc = "Possible values of the field `SELETH0TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELETH0TXR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELETH0TXR {
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
            SELETH0TXR::CONST_0 => false,
            SELETH0TXR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELETH0TXR {
        match value {
            false => SELETH0TXR::CONST_0,
            true => SELETH0TXR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELETH0TXR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELETH0TXR::CONST_1
    }
}
#[doc = "Possible values of the field `SELETH0RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELETH0RXR {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELETH0RXR {
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
            SELETH0RXR::CONST_0 => false,
            SELETH0RXR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELETH0RXR {
        match value {
            false => SELETH0RXR::CONST_0,
            true => SELETH0RXR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELETH0RXR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELETH0RXR::CONST_1
    }
}
#[doc = "Possible values of the field `SELSD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELSD0R {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELSD0R {
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
            SELSD0R::CONST_0 => false,
            SELSD0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELSD0R {
        match value {
            false => SELSD0R::CONST_0,
            true => SELSD0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELSD0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELSD0R::CONST_1
    }
}
#[doc = "Possible values of the field `SELSD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELSD1R {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELSD1R {
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
            SELSD1R::CONST_0 => false,
            SELSD1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELSD1R {
        match value {
            false => SELSD1R::CONST_0,
            true => SELSD1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELSD1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELSD1R::CONST_1
    }
}
#[doc = "Possible values of the field `SELECAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECAT0R {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELECAT0R {
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
            SELECAT0R::CONST_0 => false,
            SELECAT0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELECAT0R {
        match value {
            false => SELECAT0R::CONST_0,
            true => SELECAT0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SELECAT0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SELECAT0R::CONST_1
    }
}
#[doc = "Values that can be written to the field `SELPS`"]
pub enum SELPSW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELPSW::CONST_0 => false,
            SELPSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELPSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELPSW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELPSW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELDS1`"]
pub enum SELDS1W {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELDS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELDS1W::CONST_0 => false,
            SELDS1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELDS1W<'a> {
    w: &'a mut W,
}
impl<'a> _SELDS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELDS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELDS1W::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELDS1W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USIC0DRA`"]
pub enum USIC0DRAW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl USIC0DRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC0DRAW::CONST_0 => false,
            USIC0DRAW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC0DRAW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC0DRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC0DRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC0DRAW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC0DRAW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USIC1DRA`"]
pub enum USIC1DRAW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl USIC1DRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC1DRAW::CONST_0 => false,
            USIC1DRAW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC1DRAW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC1DRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC1DRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC1DRAW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC1DRAW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCANDRA`"]
pub enum MCANDRAW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl MCANDRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCANDRAW::CONST_0 => false,
            MCANDRAW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCANDRAW<'a> {
    w: &'a mut W,
}
impl<'a> _MCANDRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCANDRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MCANDRAW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MCANDRAW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPRFDRA`"]
pub enum PPRFDRAW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl PPRFDRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPRFDRAW::CONST_0 => false,
            PPRFDRAW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPRFDRAW<'a> {
    w: &'a mut W,
}
impl<'a> _PPRFDRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPRFDRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPRFDRAW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPRFDRAW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELUSB`"]
pub enum SELUSBW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELUSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELUSBW::CONST_0 => false,
            SELUSBW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _SELUSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELUSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELUSBW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELUSBW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELETH0TX`"]
pub enum SELETH0TXW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELETH0TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELETH0TXW::CONST_0 => false,
            SELETH0TXW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELETH0TXW<'a> {
    w: &'a mut W,
}
impl<'a> _SELETH0TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELETH0TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELETH0TXW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELETH0TXW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELETH0RX`"]
pub enum SELETH0RXW {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELETH0RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELETH0RXW::CONST_0 => false,
            SELETH0RXW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELETH0RXW<'a> {
    w: &'a mut W,
}
impl<'a> _SELETH0RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELETH0RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELETH0RXW::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELETH0RXW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELSD0`"]
pub enum SELSD0W {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELSD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELSD0W::CONST_0 => false,
            SELSD0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELSD0W<'a> {
    w: &'a mut W,
}
impl<'a> _SELSD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELSD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELSD0W::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELSD0W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELSD1`"]
pub enum SELSD1W {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELSD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELSD1W::CONST_0 => false,
            SELSD1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELSD1W<'a> {
    w: &'a mut W,
}
impl<'a> _SELSD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELSD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELSD1W::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELSD1W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELECAT0`"]
pub enum SELECAT0W {
    #[doc = "Not selected"]
    CONST_0,
    #[doc = "Selected"]
    CONST_1,
}
impl SELECAT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELECAT0W::CONST_0 => false,
            SELECAT0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELECAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SELECAT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELECAT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SELECAT0W::CONST_0)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SELECAT0W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline]
    pub fn selps(&self) -> SELPSR {
        SELPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline]
    pub fn selds1(&self) -> SELDS1R {
        SELDS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline]
    pub fn usic0dra(&self) -> USIC0DRAR {
        USIC0DRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline]
    pub fn usic1dra(&self) -> USIC1DRAR {
        USIC1DRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline]
    pub fn mcandra(&self) -> MCANDRAR {
        MCANDRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline]
    pub fn pprfdra(&self) -> PPRFDRAR {
        PPRFDRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline]
    pub fn selusb(&self) -> SELUSBR {
        SELUSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline]
    pub fn seleth0tx(&self) -> SELETH0TXR {
        SELETH0TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline]
    pub fn seleth0rx(&self) -> SELETH0RXR {
        SELETH0RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline]
    pub fn selsd0(&self) -> SELSD0R {
        SELSD0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline]
    pub fn selsd1(&self) -> SELSD1R {
        SELSD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline]
    pub fn selecat0(&self) -> SELECAT0R {
        SELECAT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline]
    pub fn selps(&mut self) -> _SELPSW {
        _SELPSW { w: self }
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline]
    pub fn selds1(&mut self) -> _SELDS1W {
        _SELDS1W { w: self }
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline]
    pub fn usic0dra(&mut self) -> _USIC0DRAW {
        _USIC0DRAW { w: self }
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline]
    pub fn usic1dra(&mut self) -> _USIC1DRAW {
        _USIC1DRAW { w: self }
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline]
    pub fn mcandra(&mut self) -> _MCANDRAW {
        _MCANDRAW { w: self }
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline]
    pub fn pprfdra(&mut self) -> _PPRFDRAW {
        _PPRFDRAW { w: self }
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline]
    pub fn selusb(&mut self) -> _SELUSBW {
        _SELUSBW { w: self }
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline]
    pub fn seleth0tx(&mut self) -> _SELETH0TXW {
        _SELETH0TXW { w: self }
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline]
    pub fn seleth0rx(&mut self) -> _SELETH0RXW {
        _SELETH0RXW { w: self }
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline]
    pub fn selsd0(&mut self) -> _SELSD0W {
        _SELSD0W { w: self }
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline]
    pub fn selsd1(&mut self) -> _SELSD1W {
        _SELSD1W { w: self }
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline]
    pub fn selecat0(&mut self) -> _SELECAT0W {
        _SELECAT0W { w: self }
    }
}
