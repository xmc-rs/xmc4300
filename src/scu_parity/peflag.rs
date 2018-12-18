#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PEFLAG {
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
#[doc = "Possible values of the field `PEFPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFPSR {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFPSR {
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
            PEFPSR::CONST_0 => false,
            PEFPSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEFPSR {
        match value {
            false => PEFPSR::CONST_0,
            true => PEFPSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEFPSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEFPSR::CONST_1
    }
}
#[doc = "Possible values of the field `PEFDS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFDS1R {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFDS1R {
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
            PEFDS1R::CONST_0 => false,
            PEFDS1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEFDS1R {
        match value {
            false => PEFDS1R::CONST_0,
            true => PEFDS1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEFDS1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEFDS1R::CONST_1
    }
}
#[doc = "Possible values of the field `PEFU0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFU0R {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFU0R {
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
            PEFU0R::CONST_0 => false,
            PEFU0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEFU0R {
        match value {
            false => PEFU0R::CONST_0,
            true => PEFU0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEFU0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEFU0R::CONST_1
    }
}
#[doc = "Possible values of the field `PEFU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFU1R {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFU1R {
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
            PEFU1R::CONST_0 => false,
            PEFU1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEFU1R {
        match value {
            false => PEFU1R::CONST_0,
            true => PEFU1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEFU1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEFU1R::CONST_1
    }
}
#[doc = "Possible values of the field `PEFMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFMCR {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFMCR {
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
            PEFMCR::CONST_0 => false,
            PEFMCR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEFMCR {
        match value {
            false => PEFMCR::CONST_0,
            true => PEFMCR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEFMCR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEFMCR::CONST_1
    }
}
#[doc = "Possible values of the field `PEFPPRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFPPRFR {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFPPRFR {
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
            PEFPPRFR::CONST_0 => false,
            PEFPPRFR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEFPPRFR {
        match value {
            false => PEFPPRFR::CONST_0,
            true => PEFPPRFR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEFPPRFR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEFPPRFR::CONST_1
    }
}
#[doc = "Possible values of the field `PEUSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEUSBR {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEUSBR {
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
            PEUSBR::CONST_0 => false,
            PEUSBR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEUSBR {
        match value {
            false => PEUSBR::CONST_0,
            true => PEUSBR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEUSBR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEUSBR::CONST_1
    }
}
#[doc = "Possible values of the field `PEETH0TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEETH0TXR {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEETH0TXR {
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
            PEETH0TXR::CONST_0 => false,
            PEETH0TXR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEETH0TXR {
        match value {
            false => PEETH0TXR::CONST_0,
            true => PEETH0TXR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEETH0TXR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEETH0TXR::CONST_1
    }
}
#[doc = "Possible values of the field `PEETH0RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEETH0RXR {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEETH0RXR {
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
            PEETH0RXR::CONST_0 => false,
            PEETH0RXR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEETH0RXR {
        match value {
            false => PEETH0RXR::CONST_0,
            true => PEETH0RXR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEETH0RXR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEETH0RXR::CONST_1
    }
}
#[doc = "Possible values of the field `PESD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD0R {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PESD0R {
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
            PESD0R::CONST_0 => false,
            PESD0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PESD0R {
        match value {
            false => PESD0R::CONST_0,
            true => PESD0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PESD0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PESD0R::CONST_1
    }
}
#[doc = "Possible values of the field `PESD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD1R {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PESD1R {
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
            PESD1R::CONST_0 => false,
            PESD1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PESD1R {
        match value {
            false => PESD1R::CONST_0,
            true => PESD1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PESD1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PESD1R::CONST_1
    }
}
#[doc = "Possible values of the field `PEECAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEECAT0R {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEECAT0R {
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
            PEECAT0R::CONST_0 => false,
            PEECAT0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEECAT0R {
        match value {
            false => PEECAT0R::CONST_0,
            true => PEECAT0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PEECAT0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PEECAT0R::CONST_1
    }
}
#[doc = "Values that can be written to the field `PEFPS`"]
pub enum PEFPSW {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEFPSW::CONST_0 => false,
            PEFPSW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEFPSW<'a> {
    w: &'a mut W,
}
impl<'a> _PEFPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEFPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFPSW::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFPSW::CONST_1)
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
#[doc = "Values that can be written to the field `PEFDS1`"]
pub enum PEFDS1W {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFDS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEFDS1W::CONST_0 => false,
            PEFDS1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEFDS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PEFDS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEFDS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFDS1W::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFDS1W::CONST_1)
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
#[doc = "Values that can be written to the field `PEFU0`"]
pub enum PEFU0W {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEFU0W::CONST_0 => false,
            PEFU0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEFU0W<'a> {
    w: &'a mut W,
}
impl<'a> _PEFU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEFU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFU0W::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFU0W::CONST_1)
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
#[doc = "Values that can be written to the field `PEFU1`"]
pub enum PEFU1W {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFU1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEFU1W::CONST_0 => false,
            PEFU1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEFU1W<'a> {
    w: &'a mut W,
}
impl<'a> _PEFU1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEFU1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFU1W::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFU1W::CONST_1)
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
#[doc = "Values that can be written to the field `PEFMC`"]
pub enum PEFMCW {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEFMCW::CONST_0 => false,
            PEFMCW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEFMCW<'a> {
    w: &'a mut W,
}
impl<'a> _PEFMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEFMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFMCW::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFMCW::CONST_1)
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
#[doc = "Values that can be written to the field `PEFPPRF`"]
pub enum PEFPPRFW {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEFPPRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEFPPRFW::CONST_0 => false,
            PEFPPRFW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEFPPRFW<'a> {
    w: &'a mut W,
}
impl<'a> _PEFPPRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEFPPRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEFPPRFW::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEFPPRFW::CONST_1)
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
#[doc = "Values that can be written to the field `PEUSB`"]
pub enum PEUSBW {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEUSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEUSBW::CONST_0 => false,
            PEUSBW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _PEUSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEUSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEUSBW::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEUSBW::CONST_1)
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
#[doc = "Values that can be written to the field `PEETH0TX`"]
pub enum PEETH0TXW {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEETH0TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEETH0TXW::CONST_0 => false,
            PEETH0TXW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEETH0TXW<'a> {
    w: &'a mut W,
}
impl<'a> _PEETH0TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEETH0TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEETH0TXW::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEETH0TXW::CONST_1)
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
#[doc = "Values that can be written to the field `PEETH0RX`"]
pub enum PEETH0RXW {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEETH0RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEETH0RXW::CONST_0 => false,
            PEETH0RXW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEETH0RXW<'a> {
    w: &'a mut W,
}
impl<'a> _PEETH0RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEETH0RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEETH0RXW::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEETH0RXW::CONST_1)
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
#[doc = "Values that can be written to the field `PESD0`"]
pub enum PESD0W {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PESD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PESD0W::CONST_0 => false,
            PESD0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PESD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PESD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PESD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PESD0W::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PESD0W::CONST_1)
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
#[doc = "Values that can be written to the field `PESD1`"]
pub enum PESD1W {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PESD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PESD1W::CONST_0 => false,
            PESD1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PESD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PESD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PESD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PESD1W::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PESD1W::CONST_1)
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
#[doc = "Values that can be written to the field `PEECAT0`"]
pub enum PEECAT0W {
    #[doc = "No parity error detected"]
    CONST_0,
    #[doc = "Parity error detected"]
    CONST_1,
}
impl PEECAT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEECAT0W::CONST_0 => false,
            PEECAT0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEECAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PEECAT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEECAT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PEECAT0W::CONST_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PEECAT0W::CONST_1)
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
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline]
    pub fn pefps(&self) -> PEFPSR {
        PEFPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline]
    pub fn pefds1(&self) -> PEFDS1R {
        PEFDS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline]
    pub fn pefu0(&self) -> PEFU0R {
        PEFU0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline]
    pub fn pefu1(&self) -> PEFU1R {
        PEFU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline]
    pub fn pefmc(&self) -> PEFMCR {
        PEFMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline]
    pub fn pefpprf(&self) -> PEFPPRFR {
        PEFPPRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline]
    pub fn peusb(&self) -> PEUSBR {
        PEUSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline]
    pub fn peeth0tx(&self) -> PEETH0TXR {
        PEETH0TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline]
    pub fn peeth0rx(&self) -> PEETH0RXR {
        PEETH0RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline]
    pub fn pesd0(&self) -> PESD0R {
        PESD0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline]
    pub fn pesd1(&self) -> PESD1R {
        PESD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Parity Error Flag for ECAT0 Memory"]
    #[inline]
    pub fn peecat0(&self) -> PEECAT0R {
        PEECAT0R::_from({
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
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline]
    pub fn pefps(&mut self) -> _PEFPSW {
        _PEFPSW { w: self }
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline]
    pub fn pefds1(&mut self) -> _PEFDS1W {
        _PEFDS1W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline]
    pub fn pefu0(&mut self) -> _PEFU0W {
        _PEFU0W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline]
    pub fn pefu1(&mut self) -> _PEFU1W {
        _PEFU1W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline]
    pub fn pefmc(&mut self) -> _PEFMCW {
        _PEFMCW { w: self }
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline]
    pub fn pefpprf(&mut self) -> _PEFPPRFW {
        _PEFPPRFW { w: self }
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline]
    pub fn peusb(&mut self) -> _PEUSBW {
        _PEUSBW { w: self }
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline]
    pub fn peeth0tx(&mut self) -> _PEETH0TXW {
        _PEETH0TXW { w: self }
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline]
    pub fn peeth0rx(&mut self) -> _PEETH0RXW {
        _PEETH0RXW { w: self }
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline]
    pub fn pesd0(&mut self) -> _PESD0W {
        _PESD0W { w: self }
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline]
    pub fn pesd1(&mut self) -> _PESD1W {
        _PESD1W { w: self }
    }
    #[doc = "Bit 24 - Parity Error Flag for ECAT0 Memory"]
    #[inline]
    pub fn peecat0(&mut self) -> _PEECAT0W {
        _PEECAT0W { w: self }
    }
}
